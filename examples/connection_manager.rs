//! Example: Using the ConnectionManager for advanced GRBL control
//!
//! This example demonstrates the full power of the ConnectionManager,
//! including command queueing, status monitoring, and event handling.
//!
//! Usage:
//!   cargo run --example connection_manager

use rcandle::connection::{ConnectionManager, ConnectionManagerConfig, SerialConnection};
use rcandle::grbl::{GrblCommand, RealtimeCommand};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== rCandle ConnectionManager Example ===\n");

    // Create a serial connection
    let port = "/dev/ttyUSB0".to_string(); // Adjust for your system
    let baud_rate = 115200;
    let connection = Box::new(SerialConnection::new(port, baud_rate));

    // Create ConnectionManager with custom configuration
    let manager_config = ConnectionManagerConfig {
        status_interval_ms: 250, // Query status every 250ms
        response_timeout: Duration::from_millis(2000),
        reconnect_attempts: 3,
        reconnect_delay: Duration::from_secs(1),
        auto_status_query: true,
    };

    println!("Creating ConnectionManager...");
    let mut manager = ConnectionManager::with_config(connection, manager_config);

    // Subscribe to status updates
    let mut status_rx = manager.subscribe_status();
    println!("✓ Subscribed to status updates");

    // Subscribe to connection events
    let mut event_rx = manager.subscribe_events();
    println!("✓ Subscribed to connection events");

    // Subscribe to all responses
    let mut response_rx = manager.subscribe_responses();
    println!("✓ Subscribed to all responses\n");

    // Spawn a task to print status updates
    let status_task = tokio::spawn(async move {
        while let Ok(status) = status_rx.recv().await {
            println!("[STATUS] State: {:?}, Position: ({:.2}, {:.2}, {:.2})", 
                status.state,
                status.mpos.map(|p| p.x).unwrap_or(0.0),
                status.mpos.map(|p| p.y).unwrap_or(0.0),
                status.mpos.map(|p| p.z).unwrap_or(0.0)
            );
        }
    });

    // Spawn a task to print connection events
    let event_task = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            println!("[EVENT] {:?}", event);
        }
    });

    // Spawn a task to print responses
    let response_task = tokio::spawn(async move {
        while let Ok(response) = response_rx.recv().await {
            println!("[RESPONSE] {:?}", response);
        }
    });

    // Connect to the controller
    println!("Connecting to GRBL controller...");
    match manager.connect(Duration::from_secs(5)).await {
        Ok(_) => println!("✓ Connected successfully!\n"),
        Err(e) => {
            eprintln!("✗ Failed to connect: {}", e);
            return Err(e.into());
        }
    }

    // Wait a bit for welcome message
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Queue some commands
    println!("Queueing G-code commands...");
    let commands = vec![
        GrblCommand::GCode("G21".to_string()),    // Metric units
        GrblCommand::GCode("G90".to_string()),    // Absolute positioning
        GrblCommand::GCode("G0 X10 Y10".to_string()), // Rapid move
        GrblCommand::GCode("G1 X20 Y20 F1000".to_string()), // Linear move
    ];

    for cmd in commands {
        println!("Queuing: {}", cmd);
        manager.send_command(cmd).await?;
    }

    println!("\n✓ Commands queued. Watching execution...\n");

    // Let the manager process commands and receive status updates
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Send a real-time command (bypasses queue)
    println!("\nSending real-time Feed Hold command...");
    manager.send_realtime(RealtimeCommand::FeedHold.as_byte()).await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    // Resume
    println!("Sending real-time Cycle Start command...");
    manager.send_realtime(RealtimeCommand::CycleStartResume.as_byte()).await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // Pause the queue
    println!("\nPausing command queue...");
    manager.pause().await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    // Resume the queue
    println!("Resuming command queue...");
    manager.resume().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // Clear any remaining commands
    println!("\nClearing command queue...");
    manager.clear_queue().await?;

    // Disconnect
    println!("\nDisconnecting...");
    manager.disconnect().await?;
    println!("✓ Disconnected\n");

    // Stop background tasks
    status_task.abort();
    event_task.abort();
    response_task.abort();

    Ok(())
}
