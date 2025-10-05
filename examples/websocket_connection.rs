//! Example: WebSocket connection to a GRBL controller
//!
//! This example demonstrates how to connect to a GRBL controller using
//! WebSocket protocol, useful for web-based interfaces and cloud connections.
//!
//! Usage:
//!   cargo run --example websocket_connection

use rcandle::connection::{Connection, WebSocketConfig, WebSocketConnection};
use rcandle::grbl::RealtimeCommand;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== rCandle WebSocket Connection Example ===\n");

    // Create WebSocket connection configuration
    let config = WebSocketConfig {
        url: "ws://192.168.1.100:8080/grbl".to_string(), // Adjust for your setup
        connect_timeout_ms: 5000,
        read_timeout_ms: 1000,
        ping_interval_secs: 30,
        auto_reconnect: false,
    };

    println!("Creating WebSocket connection to {}...", config.url);
    let mut connection = WebSocketConnection::new(config);

    // Connect to the controller
    println!("Connecting...");
    match connection.connect(Duration::from_secs(5)).await {
        Ok(_) => println!("✓ Connected successfully!"),
        Err(e) => {
            eprintln!("✗ Failed to connect: {}", e);
            eprintln!("\nNote: Make sure:");
            eprintln!("  1. Your GRBL WebSocket server is running");
            eprintln!("  2. The WebSocket URL is correct (ws:// or wss://)");
            eprintln!("  3. The server accepts WebSocket connections");
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

    // Send some G-code commands
    println!("\nSending G-code commands...");
    let commands = vec![
        "G21",    // Metric units
        "G90",    // Absolute positioning
        "G0 X0 Y0", // Rapid to origin
        "$G",     // Get parser state
    ];

    for cmd in commands {
        println!("Sending: {}", cmd);
        connection.send_line(cmd).await?;
        
        // Wait for response
        if let Ok(Some(response)) = connection.receive_line(Duration::from_secs(1)).await {
            println!("Response: {}", response);
        }
    }

    // Test real-time commands
    println!("\nTesting real-time commands...");
    
    // Feed hold
    println!("Sending: Feed Hold (!)");
    connection
        .send_bytes(&[RealtimeCommand::FeedHold.as_byte()])
        .await?;
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Cycle start
    println!("Sending: Cycle Start (~)");
    connection
        .send_bytes(&[RealtimeCommand::CycleStartResume.as_byte()])
        .await?;

    // Disconnect
    println!("\nDisconnecting...");
    connection.disconnect().await?;
    println!("✓ Disconnected\n");

    Ok(())
}
