//! Example: Telnet connection to a networked GRBL controller
//!
//! This example demonstrates how to connect to a GRBL controller over
//! a TCP/IP network using Telnet protocol.
//!
//! Usage:
//!   cargo run --example telnet_connection

use rcandle::connection::{Connection, TelnetConfig, TelnetConnection};
use rcandle::grbl::RealtimeCommand;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== rCandle Telnet Connection Example ===\n");

    // Create Telnet connection configuration
    let config = TelnetConfig {
        host: "192.168.1.100".to_string(), // Adjust for your GRBL controller IP
        port: 23,
        connect_timeout_ms: 5000,
        read_timeout_ms: 1000,
        keepalive: true,
        keepalive_interval_secs: 60,
    };

    println!("Creating Telnet connection to {}:{}...", config.host, config.port);
    let mut connection = TelnetConnection::new(config);

    // Connect to the controller
    println!("Connecting...");
    match connection.connect(Duration::from_secs(5)).await {
        Ok(_) => println!("✓ Connected successfully!"),
        Err(e) => {
            eprintln!("✗ Failed to connect: {}", e);
            eprintln!("\nNote: Make sure:");
            eprintln!("  1. Your GRBL controller is connected to the network");
            eprintln!("  2. The IP address and port are correct");
            eprintln!("  3. No firewall is blocking the connection");
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
    ];

    for cmd in commands {
        println!("Sending: {}", cmd);
        connection.send_line(cmd).await?;
        
        // Wait for response
        if let Ok(Some(response)) = connection.receive_line(Duration::from_secs(1)).await {
            println!("Response: {}", response);
        }
    }

    // Disconnect
    println!("\nDisconnecting...");
    connection.disconnect().await?;
    println!("✓ Disconnected\n");

    Ok(())
}
