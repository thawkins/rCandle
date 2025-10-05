//! Integration tests for connection module
//!
//! These tests verify end-to-end functionality of the connection system
//! using a mock GRBL simulator.

use rcandle::connection::{Connection, TelnetConnection, ConnectionManager, ConnectionManagerConfig};
use rcandle::grbl::{GrblCommand, MachineState};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

// Include mock GRBL module
#[path = "common/mock_grbl.rs"]
mod mock_grbl;
use mock_grbl::MockGrbl;

/// Test connecting to a mock GRBL server via Telnet
#[tokio::test]
async fn test_telnet_connection_to_mock_grbl() {
    // Start mock GRBL server on a random high port
    let port = 23456;
    let grbl = Arc::new(MockGrbl::new());
    let grbl_clone = Arc::clone(&grbl);
    
    // Start server in background
    tokio::spawn(async move {
        if let Err(e) = grbl_clone.start_tcp_server(port).await {
            eprintln!("Mock GRBL server error: {}", e);
        }
    });
    
    // Give server time to start
    sleep(Duration::from_millis(100)).await;
    
    // Create telnet connection
    let mut connection = TelnetConnection::with_address("127.0.0.1".to_string(), port);
    
    // Connect
    let result = connection.connect(Duration::from_secs(5)).await;
    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());
    assert!(connection.is_connected());
    
    // Receive welcome message
    let welcome = connection.receive_line(Duration::from_secs(2)).await;
    assert!(welcome.is_ok());
    if let Ok(Some(welcome_msg)) = welcome {
        assert!(welcome_msg.contains("Grbl"), "Expected Grbl welcome message, got: {}", welcome_msg);
    }
    
    // Send a status query
    let result = connection.send_line("?").await;
    assert!(result.is_ok());
    
    // Receive status response
    let response = connection.receive_line(Duration::from_secs(1)).await;
    assert!(response.is_ok());
    if let Ok(Some(status)) = response {
        assert!(status.starts_with("<"), "Expected status report, got: {}", status);
        assert!(status.contains("Idle"), "Expected Idle state in: {}", status);
    }
    
    // Send a G-code command
    let result = connection.send_line("G0 X10").await;
    assert!(result.is_ok());
    
    // Receive OK response
    let response = connection.receive_line(Duration::from_secs(1)).await;
    assert!(response.is_ok());
    if let Ok(Some(ok_msg)) = response {
        assert!(ok_msg.contains("ok"), "Expected 'ok' response, got: {}", ok_msg);
    }
    
    // Verify command was received by mock
    let commands = grbl.get_commands();
    assert!(commands.contains(&"?".to_string()));
    assert!(commands.contains(&"G0 X10".to_string()));
    
    // Disconnect
    let result = connection.disconnect().await;
    assert!(result.is_ok());
    assert!(!connection.is_connected());
}

/// Test connection manager with mock GRBL
#[tokio::test]
async fn test_connection_manager_with_mock_grbl() {
    // Start mock GRBL server
    let port = 23457;
    let grbl = Arc::new(MockGrbl::new());
    let grbl_clone = Arc::clone(&grbl);
    
    tokio::spawn(async move {
        if let Err(e) = grbl_clone.start_tcp_server(port).await {
            eprintln!("Mock GRBL server error: {}", e);
        }
    });
    
    sleep(Duration::from_millis(200)).await;
    
    // Create connection and manager
    let connection = Box::new(TelnetConnection::with_address("127.0.0.1".to_string(), port));
    
    let manager_config = ConnectionManagerConfig::default();
    let mut manager = ConnectionManager::with_config(connection, manager_config);
    
    // Subscribe to status updates
    let mut status_rx = manager.subscribe_status();
    
    // Connect
    let result = manager.connect(Duration::from_secs(5)).await;
    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());
    
    // Wait for first status update (increase timeout for more reliable test)
    let status = tokio::time::timeout(Duration::from_secs(5), status_rx.recv()).await;
    assert!(status.is_ok(), "Timeout waiting for status");
    
    if let Ok(Ok(status)) = status {
        assert_eq!(status.state, MachineState::Idle);
    }
    
    // Send a command
    let cmd = GrblCommand::GCode("G0 X10 Y20".to_string());
    let result = manager.send_command(cmd).await;
    assert!(result.is_ok());
    
    // Give time for command to be processed
    sleep(Duration::from_millis(500)).await;
    
    // Verify command was received
    let commands = grbl.get_commands();
    assert!(commands.iter().any(|c| c.contains("G0 X10 Y20")), 
            "Expected G0 command in: {:?}", commands);
    
    // Disconnect
    let result = manager.disconnect().await;
    assert!(result.is_ok());
}

/// Test command queueing through manager
#[tokio::test]
async fn test_command_queue_through_manager() {
    let port = 23458;
    let grbl = Arc::new(MockGrbl::new());
    let grbl_clone = Arc::clone(&grbl);
    
    tokio::spawn(async move {
        if let Err(e) = grbl_clone.start_tcp_server(port).await {
            eprintln!("Mock GRBL server error: {}", e);
        }
    });
    
    sleep(Duration::from_millis(200)).await;
    
    let connection = Box::new(TelnetConnection::with_address("127.0.0.1".to_string(), port));
    let mut manager = ConnectionManager::new(connection);
    
    manager.connect(Duration::from_secs(5)).await.unwrap();
    sleep(Duration::from_millis(500)).await;
    
    // Queue multiple commands
    let commands = vec![
        GrblCommand::GCode("G0 X10".to_string()),
        GrblCommand::GCode("G1 Y20 F100".to_string()),
        GrblCommand::GCode("G1 Z5 F50".to_string()),
    ];
    
    for cmd in commands {
        manager.send_command(cmd).await.unwrap();
    }
    
    // Give time for all commands to be processed
    sleep(Duration::from_secs(2)).await;
    
    // Verify all commands were received
    let received = grbl.get_commands();
    println!("Received commands: {:?}", received);
    assert!(received.iter().any(|c| c.contains("G0 X10")), "Expected 'G0 X10' in: {:?}", received);
    assert!(received.iter().any(|c| c.contains("G1 Y20")), "Expected 'G1 Y20' in: {:?}", received);
    assert!(received.iter().any(|c| c.contains("G1 Z5")), "Expected 'G1 Z5' in: {:?}", received);
    
    manager.disconnect().await.unwrap();
}

/// Test error handling when server is not available
#[tokio::test]
async fn test_connection_error_handling() {
    // Try to connect to a port that doesn't have a server
    let mut connection = TelnetConnection::with_address("127.0.0.1".to_string(), 54321);
    
    let result = connection.connect(Duration::from_secs(1)).await;
    assert!(result.is_err(), "Expected connection to fail");
}

/// Test reconnection behavior
#[tokio::test]
async fn test_reconnection() {
    let port = 23459;
    let grbl = Arc::new(MockGrbl::new());
    let grbl_clone = Arc::clone(&grbl);
    
    tokio::spawn(async move {
        if let Err(e) = grbl_clone.start_tcp_server(port).await {
            eprintln!("Mock GRBL server error: {}", e);
        }
    });
    
    sleep(Duration::from_millis(100)).await;
    
    let mut connection = TelnetConnection::with_address("127.0.0.1".to_string(), port);
    
    // First connection
    connection.connect(Duration::from_secs(5)).await.unwrap();
    assert!(connection.is_connected());
    
    // Disconnect
    connection.disconnect().await.unwrap();
    assert!(!connection.is_connected());
    
    // Reconnect
    connection.connect(Duration::from_secs(5)).await.unwrap();
    assert!(connection.is_connected());
    
    connection.disconnect().await.unwrap();
}
