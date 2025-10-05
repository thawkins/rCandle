//! Example: Serial connection to a GRBL controller
//!
//! This example demonstrates how to connect to a GRBL controller via serial port,
//! send commands, and receive responses.
//!
//! Usage:
//!   cargo run --example serial_connection

use rcandle::connection::{Connection, SerialConnection};
use rcandle::grbl::{GrblCommand, RealtimeCommand};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== rCandle Serial Connection Example ===\n");

    // Create serial connection
    let port = "/dev/ttyUSB0".to_string(); // Adjust for your system
    let baud_rate = 115200;

    println!("Creating serial connection to {} at {} baud...", port, baud_rate);
    let mut connection = SerialConnection::new(port.clone(), baud_rate);

    // Connect to the controller
    println!("Connecting...");
    match connection.connect(Duration::from_secs(5)).await {
        Ok(_) => println!("✓ Connected successfully!"),
        Err(e) => {
            eprintln!("✗ Failed to connect: {}", e);
            eprintln!("\nNote: Make sure:");
            eprintln!("  1. Your GRBL controller is connected");
            eprintln!("  2. The port name is correct for your system:");
            eprintln!("     - Linux: /dev/ttyUSB0, /dev/ttyACM0");
            eprintln!("     - macOS: /dev/cu.usbserial-*");
            eprintln!("     - Windows: COM3, COM4, etc.");
            eprintln!("  3. Port: {}", port);
            return Err(e.into());
        }
    }

    println!("\nConnection status: {:?}", connection.status());
    println!("Description: {}\n", connection.description());

    // Wait for GRBL welcome message
    println!("Waiting for GRBL welcome message...");
    if let Ok(Some(msg)) = connection.receive_line(Duration::from_secs(2)).await {
        println!("Received: {}", msg);
    }

    // Send a status query (real-time command)
    println!("\nSending status query (?)...");
    connection
        .send_bytes(&[RealtimeCommand::StatusQuery.as_byte()])
        .await?;

    // Wait for status response
    if let Ok(Some(status)) = connection.receive_line(Duration::from_secs(1)).await {
        println!("Status: {}", status);
    }

    // Send a simple G-code command
    println!("\nSending G-code command: G21 (metric units)");
    let command = GrblCommand::GCode("G21".to_string());
    connection.send_line(&command.to_string()).await?;

    // Wait for ok response
    if let Ok(Some(response)) = connection.receive_line(Duration::from_secs(1)).await {
        println!("Response: {}", response);
    }

    // Disconnect
    println!("\nDisconnecting...");
    connection.disconnect().await?;
    println!("✓ Disconnected\n");

    Ok(())
}
