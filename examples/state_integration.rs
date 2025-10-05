//! Example of state management integration
//!
//! This example demonstrates how to use the state management system
//! with GRBL responses and event notifications.

use rcandle::state::{AppState, StateEventBroadcaster, StateUpdater, StateEvent, Position, CoordinateSystem};
use rcandle::grbl::GrblResponse;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("=== rCandle State Management Example ===\n");
    
    // Create the state infrastructure
    let app_state = AppState::new();
    let broadcaster = StateEventBroadcaster::new(100);
    let updater = StateUpdater::new(app_state.clone(), broadcaster.clone());
    
    // Subscribe to state events
    let mut event_receiver = broadcaster.subscribe();
    tokio::spawn(async move {
        println!("Event monitor started...\n");
        while let Ok(event) = event_receiver.recv().await {
            match event {
                StateEvent::MachineStatusChanged { old, new } => {
                    println!("📊 Machine Status: {:?} -> {:?}", old, new);
                }
                StateEvent::MachinePositionChanged { machine_pos, work_pos } => {
                    println!("📍 Position: Machine[{}] Work[{}]", machine_pos, work_pos);
                }
                StateEvent::SpindleStateChanged { enabled, speed } => {
                    println!("🔄 Spindle: {} at {} RPM", 
                        if enabled { "ON" } else { "OFF" }, speed);
                }
                StateEvent::FeedRateChanged { feed_rate } => {
                    println!("⚡ Feed Rate: {:.1} mm/min", feed_rate);
                }
                StateEvent::OverridesChanged { feed, rapid, spindle } => {
                    println!("🎚️  Overrides: Feed={}% Rapid={}% Spindle={}%", 
                        feed, rapid, spindle);
                }
                StateEvent::ProgramStateChanged { old, new } => {
                    println!("📝 Program State: {:?} -> {:?}", old, new);
                }
                StateEvent::ProgramProgressChanged { current_line, total_lines, progress } => {
                    println!("📈 Progress: Line {}/{} ({:.1}%)", 
                        current_line, total_lines, progress * 100.0);
                }
                StateEvent::ErrorOccurred { message } => {
                    println!("❌ Error: {}", message);
                }
                StateEvent::ConnectionChanged { connected } => {
                    println!("🔌 Connection: {}", if connected { "CONNECTED" } else { "DISCONNECTED" });
                }
                _ => {}
            }
        }
    });
    
    // Give the event monitor a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Simulate GRBL welcome message
    println!("Simulating GRBL connection...");
    let welcome = GrblResponse::Welcome {
        version: "1.1f".to_string(),
    };
    updater.process_response(&welcome);
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    // Load a program
    println!("\nLoading program...");
    updater.load_program("example.gcode".to_string(), 100);
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    // Simulate setting a work offset
    println!("\nSetting work offset for G54...");
    updater.set_work_offset(
        CoordinateSystem::G54,
        Position::new(10.0, 20.0, 5.0)
    );
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    // Simulate starting the program
    println!("\nStarting program...");
    updater.start_program();
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    // Simulate some status reports
    println!("\nSimulating machine operation...");
    for i in 0..3 {
        let status = simulate_status_report(i);
        updater.process_response(&status);
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    // Simulate OK responses (command completion)
    println!("\nSimulating command completions...");
    for _ in 0..5 {
        updater.process_response(&GrblResponse::Ok);
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
    
    // Complete the program
    println!("\nCompleting program...");
    updater.complete_program();
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    // Read final state
    {
        println!("\n=== Final State ===");
        let machine = app_state.machine.read();
        println!("Machine Status: {}", machine.status);
        println!("Machine Position: {}", machine.machine_position);
        println!("Work Position: {}", machine.work_position);
        println!("Feed Rate: {:.1} mm/min", machine.feed_rate);
        println!("Spindle: {} at {:.0} RPM", 
            if machine.spindle_enabled { "ON" } else { "OFF" },
            machine.spindle_speed);
        
        let program = app_state.program.read();
        println!("\nProgram State: {}", program.state);
        println!("Progress: {:.1}%", program.progress() * 100.0);
        println!("Lines Completed: {}/{}", program.lines_completed, program.total_lines);
    }
    
    println!("\n=== Example Complete ===");
}

/// Simulate a GRBL status report for demonstration
fn simulate_status_report(step: usize) -> GrblResponse {
    // Parse status strings to create realistic status reports
    let status_str = match step {
        0 => "<Run|MPos:10.000,15.000,2.000|WPos:0.000,-5.000,-3.000|F:500.0|S:1000>",
        1 => "<Run|MPos:20.000,25.000,2.000|WPos:10.000,5.000,-3.000|F:600.0|S:1000|Ov:100,100,100>",
        2 => "<Run|MPos:30.000,35.000,2.000|WPos:20.000,15.000,-3.000|F:700.0|S:1200|Ov:120,100,100>",
        _ => "<Idle|MPos:0.000,0.000,0.000|WPos:0.000,0.000,0.000>",
    };
    
    // This is a simplified parse - in real use, the GRBL response parser would handle this
    GrblResponse::parse(status_str).unwrap_or(GrblResponse::Message("Invalid".to_string()))
}
