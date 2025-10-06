//! Serial port connection implementation
//!
//! Provides serial communication with GRBL controllers.

use crate::Result;
use crate::utils::error::Error;
use async_trait::async_trait;
use serialport::{SerialPort, SerialPortInfo};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::traits::{Connection, ConnectionStatus};

/// Serial connection configuration
#[derive(Debug, Clone)]
pub struct SerialConfig {
    /// Serial port name (e.g., "COM3", "/dev/ttyUSB0")
    pub port: String,
    /// Baud rate
    pub baud_rate: u32,
    /// Data bits (5, 6, 7, or 8)
    pub data_bits: serialport::DataBits,
    /// Stop bits
    pub stop_bits: serialport::StopBits,
    /// Parity
    pub parity: serialport::Parity,
    /// Flow control
    pub flow_control: serialport::FlowControl,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            port: String::new(),
            baud_rate: 115200,
            data_bits: serialport::DataBits::Eight,
            stop_bits: serialport::StopBits::One,
            parity: serialport::Parity::None,
            flow_control: serialport::FlowControl::None,
        }
    }
}

/// Serial port connection
pub struct SerialConnection {
    config: SerialConfig,
    port: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
    status: ConnectionStatus,
    buffer: Arc<Mutex<Vec<String>>>,
}

impl SerialConnection {
    /// Create a new serial connection
    ///
    /// # Arguments
    /// * `port` - Serial port name
    /// * `baud_rate` - Baud rate (typically 115200 for GRBL)
    pub fn new(port: String, baud_rate: u32) -> Self {
        Self {
            config: SerialConfig {
                port,
                baud_rate,
                ..Default::default()
            },
            port: Arc::new(Mutex::new(None)),
            status: ConnectionStatus::Disconnected,
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Create a new serial connection with custom configuration
    ///
    /// # Arguments
    /// * `config` - Serial port configuration
    pub fn with_config(config: SerialConfig) -> Self {
        Self {
            config,
            port: Arc::new(Mutex::new(None)),
            status: ConnectionStatus::Disconnected,
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// List available serial ports
    ///
    /// # Returns
    /// Vector of available serial port information
    /// 
    /// On Linux, only returns USB serial ports (/dev/ttyUSB* or /dev/ttyACM*)
    /// to avoid showing system TTY devices. On other platforms, returns all ports.
    pub fn list_ports() -> Result<Vec<SerialPortInfo>> {
        let all_ports = serialport::available_ports()
            .map_err(|e| Error::Connection(e.to_string()))?;
        
        // On Linux, filter to only show USB serial ports (Issue #5)
        #[cfg(target_os = "linux")]
        {
            let filtered_ports: Vec<SerialPortInfo> = all_ports
                .into_iter()
                .filter(|port| {
                    let port_name = &port.port_name;
                    port_name.starts_with("/dev/ttyUSB") || port_name.starts_with("/dev/ttyACM")
                })
                .collect();
            Ok(filtered_ports)
        }
        
        // On other platforms (Windows, macOS), return all ports
        #[cfg(not(target_os = "linux"))]
        {
            Ok(all_ports)
        }
    }

    /// Get the current configuration
    pub fn config(&self) -> &SerialConfig {
        &self.config
    }

    /// Read available lines from the port into the buffer
    fn read_available_lines(&self) -> Result<()> {
        let port_guard = self.port.lock().unwrap();
        if let Some(port) = port_guard.as_ref() {
            // Clone the port for reading
            let port_clone = port
                .try_clone()
                .map_err(|e| Error::Connection(format!("Failed to clone port: {}", e)))?;
            drop(port_guard); // Release lock before reading

            let mut reader = BufReader::new(port_clone);
            let mut line = String::new();
            
            // Read all available lines without blocking
            while reader.read_line(&mut line).unwrap_or(0) > 0 {
                let trimmed = line.trim().to_string();
                if !trimmed.is_empty() {
                    self.buffer.lock().unwrap().push(trimmed);
                }
                line.clear();
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Connection for SerialConnection {
    async fn connect(&mut self, _connection_timeout: Duration) -> Result<()> {
        self.status = ConnectionStatus::Connecting;

        let port = serialport::new(&self.config.port, self.config.baud_rate)
            .data_bits(self.config.data_bits)
            .stop_bits(self.config.stop_bits)
            .parity(self.config.parity)
            .flow_control(self.config.flow_control)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| Error::Connection(format!("Failed to open port: {}", e)))?;

        *self.port.lock().unwrap() = Some(port);
        self.status = ConnectionStatus::Connected;

        // Wait a bit for the connection to stabilize
        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        let mut port_guard = self.port.lock().unwrap();
        if port_guard.is_some() {
            *port_guard = None;
            self.status = ConnectionStatus::Disconnected;
        }
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.port.lock().unwrap().is_some() && self.status == ConnectionStatus::Connected
    }

    fn status(&self) -> ConnectionStatus {
        self.status
    }

    async fn send_line(&mut self, data: &str) -> Result<()> {
        let mut line = data.to_string();
        if !line.ends_with('\n') {
            line.push('\n');
        }
        self.send_bytes(line.as_bytes()).await
    }

    async fn send_bytes(&mut self, data: &[u8]) -> Result<()> {
        let mut port_guard = self.port.lock().unwrap();
        if let Some(port) = port_guard.as_mut() {
            port.write_all(data)
                .map_err(|e| Error::Connection(format!("Failed to send data: {}", e)))?;
            port.flush()
                .map_err(|e| Error::Connection(format!("Failed to flush: {}", e)))?;
            Ok(())
        } else {
            Err(Error::Connection("Not connected".to_string()))
        }
    }

    async fn receive_line(&mut self, read_timeout: Duration) -> Result<Option<String>> {
        let start = std::time::Instant::now();
        
        loop {
            // Check buffer first
            {
                let mut buffer = self.buffer.lock().unwrap();
                if !buffer.is_empty() {
                    return Ok(Some(buffer.remove(0)));
                }
            }

            // Try to read more data
            self.read_available_lines()?;

            // Check if timeout exceeded
            if start.elapsed() >= read_timeout {
                return Ok(None);
            }

            // Small delay to avoid busy waiting
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    fn description(&self) -> String {
        format!("{} @ {} baud", self.config.port, self.config.baud_rate)
    }

    async fn flush(&mut self) -> Result<()> {
        let mut port_guard = self.port.lock().unwrap();
        if let Some(port) = port_guard.as_mut() {
            port.flush()
                .map_err(|e| Error::Connection(format!("Failed to flush: {}", e)))?;
        }
        Ok(())
    }
}

impl Drop for SerialConnection {
    fn drop(&mut self) {
        // Ensure port is closed when dropped
        *self.port.lock().unwrap() = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_config_default() {
        let config = SerialConfig::default();
        assert_eq!(config.baud_rate, 115200);
        assert_eq!(config.data_bits, serialport::DataBits::Eight);
        assert_eq!(config.stop_bits, serialport::StopBits::One);
        assert_eq!(config.parity, serialport::Parity::None);
    }

    #[test]
    fn test_serial_connection_new() {
        let conn = SerialConnection::new("/dev/ttyUSB0".to_string(), 115200);
        assert_eq!(conn.config.port, "/dev/ttyUSB0");
        assert_eq!(conn.config.baud_rate, 115200);
        assert_eq!(conn.status(), ConnectionStatus::Disconnected);
        assert!(!conn.is_connected());
    }

    #[test]
    fn test_list_ports() {
        // This may fail if no ports available, but should not panic
        let _ = SerialConnection::list_ports();
    }

    #[test]
    fn test_connection_description() {
        let conn = SerialConnection::new("COM3".to_string(), 115200);
        assert_eq!(conn.description(), "COM3 @ 115200 baud");
    }

    #[tokio::test]
    async fn test_send_line_not_connected() {
        let mut conn = SerialConnection::new("INVALID_PORT".to_string(), 115200);
        let result = conn.send_line("G0 X10").await;
        assert!(result.is_err());
    }
}
