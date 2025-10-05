//! G-Code Parser Example
//!
//! This example demonstrates how to use the rCandle G-Code parser to:
//! - Tokenize G-Code text
//! - Parse commands with modal state
//! - Generate motion segments
//! - Preprocess segments (arc expansion, optimization)

use rcandle::parser::{Parser, Preprocessor, Tokenizer};

fn main() {
    // Example G-Code program
    let gcode = r#"
; Simple square with arc in corner
G21         ; Use millimeters
G90         ; Absolute positioning
G17         ; XY plane
G0 Z5       ; Rapid to safe height
G0 X0 Y0    ; Rapid to start position
G1 Z-1 F200 ; Plunge to cutting depth
G1 X10      ; Move to (10, 0)
G1 Y10      ; Move to (10, 10)
G2 X0 Y20 I-10 J0  ; Arc to (0, 20)
G1 Y0       ; Move back to start
G0 Z5       ; Retract
    "#;

    println!("=== G-Code Parser Example ===\n");
    println!("Input G-Code:");
    println!("{}", gcode);
    println!("\n--- Parsing ---\n");

    // Step 1: Tokenize
    let mut tokenizer = Tokenizer::new(gcode);
    let tokens = match tokenizer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Tokenization error: {}", e);
            return;
        }
    };
    
    println!("Tokenized {} tokens", tokens.len());

    // Step 2: Parse into commands
    let mut parser = Parser::new();
    let commands = match parser.parse_tokens(&tokens) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Parsing error: {}", e);
            return;
        }
    };
    
    println!("Parsed {} commands", commands.len());
    
    // Display commands
    println!("\nCommands:");
    for (i, cmd) in commands.iter().enumerate() {
        if let Some(g) = cmd.g_command {
            print!("  {}: G{}", i, g);
            for (letter, value) in &cmd.parameters {
                print!(" {}{}", letter, value);
            }
            if let Some(f) = cmd.feed_rate {
                print!(" F{}", f);
            }
            println!();
        }
    }

    // Step 3: Generate segments
    let segments = match parser.generate_segments(&commands) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Segment generation error: {}", e);
            return;
        }
    };
    
    println!("\n--- Motion Segments ---\n");
    println!("Generated {} motion segments", segments.len());
    
    // Display segments
    for (i, seg) in segments.iter().enumerate() {
        println!("Segment {}:", i);
        println!("  Type: {:?}", seg.segment_type);
        println!("  Start: {}", seg.start);
        println!("  End: {}", seg.end);
        println!("  Length: {:.2} mm", seg.length());
        if seg.feed_rate > 0.0 {
            println!("  Feed Rate: {:.0} mm/min", seg.feed_rate);
            println!("  Time: {:.2} seconds", seg.estimated_time());
        }
        if let Some(center) = seg.center {
            println!("  Center: {}", center);
        }
        println!();
    }

    // Step 4: Preprocess (expand arcs)
    println!("--- Preprocessing ---\n");
    
    let preprocessor = Preprocessor::new().with_arc_precision(0.5);
    let processed = match preprocessor.process(&segments) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Preprocessing error: {}", e);
            return;
        }
    };
    
    println!("After arc expansion: {} segments", processed.len());
    
    // Calculate total statistics
    let total_length: f64 = processed.iter().map(|s| s.length()).sum();
    let total_time: f64 = processed.iter().map(|s| s.estimated_time()).sum();
    let cutting_segments = processed.iter().filter(|s| s.is_cutting()).count();
    
    println!("\n=== Statistics ===");
    println!("Total segments: {}", processed.len());
    println!("Cutting segments: {}", cutting_segments);
    println!("Rapid segments: {}", processed.len() - cutting_segments);
    println!("Total path length: {:.2} mm", total_length);
    println!("Estimated time: {:.2} seconds", total_time);
    
    // Show parser state
    println!("\n=== Final Parser State ===");
    let state = parser.state();
    println!("Position: {}", state.position);
    println!("Units: {:?}", state.units);
    println!("Positioning Mode: {:?}", state.positioning_mode);
    println!("Plane: {:?}", state.plane);
    println!("Feed Rate: {:.0} mm/min", state.feed_rate);
}
