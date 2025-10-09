# rCandle Architecture Document

## 1. System Architecture Overview

rCandle follows a modular, layered architecture designed to separate concerns and enable maintainability. The system is organized into distinct layers that communicate through well-defined interfaces.

```
┌─────────────────────────────────────────────────────────────┐
│                      User Interface Layer                    │
│  (Iced/egui framework - windows, widgets, event handling)   │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────┴───────────────────────────────────┐
│                   Application Logic Layer                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │    State     │  │   Commands   │  │  Controller  │      │
│  │  Management  │  │   Processor  │  │   Mediator   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└──────────┬────────────────┬──────────────────┬──────────────┘
           │                │                  │
┌──────────┴────────┐ ┌────┴─────────┐ ┌──────┴──────────────┐
│  Parser Module    │ │ Connection   │ │  Renderer Module    │
│  (G-Code parsing) │ │   Module     │ │  (3D Visualization) │
│                   │ │ (GRBL Comm)  │ │                     │
└───────────────────┘ └──────┬───────┘ └─────────────────────┘
                             │
                    ┌────────┴─────────┐
                    │  Hardware Layer  │
                    │ (Serial/Network) │
                    └──────────────────┘
```

## 2. Module Breakdown

### 2.1 Connection Module

**Purpose**: Abstract communication with GRBL controllers

**Components**:

```rust
// Connection trait - abstract interface
pub trait Connection: Send {
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;
    fn is_connected(&self) -> bool;
    async fn send(&mut self, data: &[u8]) -> Result<()>;
    async fn receive(&mut self) -> Result<Option<Vec<u8>>>;
}

// Serial implementation
pub struct SerialConnection {
    port: tokio_serial::SerialStream,
    config: SerialConfig,
    rx_buffer: VecDeque<u8>,
}

// Manager handles connection lifecycle
pub struct ConnectionManager {
    connection: Box<dyn Connection>,
    command_queue: mpsc::Sender<Command>,
    response_queue: mpsc::Receiver<Response>,
    status_tx: broadcast::Sender<MachineStatus>,
}
```

**Key Responsibilities**:
- Maintain connection state
- Queue commands for transmission
- Parse incoming data into responses
- Handle reconnection logic
- Provide real-time status updates

**Communication Pattern**:
```
UI/Logic Layer
      ↓ (send command)
ConnectionManager
      ↓ (queue)
Command Queue → Connection Implementation → Hardware
      ↑ (receive)
Response Parser ← Connection Implementation ← Hardware
      ↑
Status Broadcast → Subscribers (UI, Logger, etc.)
```

### 2.2 Parser Module

**Purpose**: Parse and preprocess G-Code

**Architecture**:

```rust
// Main parser structure
pub struct GCodeParser {
    lexer: GCodeLexer,
    state: ParserState,  // Modal state (G90/G91, units, etc.)
}

// Token representation
pub enum Token {
    Command { code: char, number: u32 },
    Parameter { letter: char, value: f64 },
    Comment(String),
    LineNumber(u32),
    Checksum(u8),
}

// Parsed command
pub struct GCodeCommand {
    pub line_number: Option<u32>,
    pub modal_group: ModalGroup,
    pub command_type: CommandType,
    pub parameters: HashMap<char, f64>,
    pub comment: Option<String>,
}

// Segment for visualization
pub enum Segment {
    Line(LineSegment),
    Arc(ArcSegment),
    Rapid(RapidSegment),
}

pub struct LineSegment {
    pub start: Point3D,
    pub end: Point3D,
    pub feed_rate: f64,
}
```

**Parsing Pipeline**:
```
Raw G-Code String
      ↓
   Lexer (tokenize)
      ↓
   Parser (syntax analysis)
      ↓
  Preprocessor (arc expansion, unit conversion)
      ↓
  Segment Generator (visualization data)
      ↓
Internal Representation (Vec<Segment>)
```

**Modal State Tracking**:
The parser maintains state for modal commands:
- Motion mode (G0, G1, G2, G3, etc.)
- Coordinate system (G90/G91, G20/G21)
- Plane selection (G17/G18/G19)
- Feed rate mode (G93/G94)
- Tool offset (G43/G49)
- Work coordinate system (G54-G59)

### 2.3 Renderer Module

**Purpose**: 3D visualization of toolpaths and machine state

**Architecture based on WGPU**:

```rust
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    camera: Camera,
    scene: Scene,
}

pub struct Scene {
    drawables: Vec<Box<dyn Drawable>>,
    transforms: Vec<Transform>,
}

pub trait Drawable {
    fn vertex_data(&self) -> &[Vertex];
    fn index_data(&self) -> &[u16];
    fn pipeline(&self) -> &wgpu::RenderPipeline;
    fn update(&mut self, dt: f32);
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

// Vertex structure
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

// Camera for 3D navigation
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}
```

**Rendering Pipeline**:
```
Scene Graph
      ↓
  For each Drawable:
    ↓
  Update Transform
    ↓
  Generate/Update Vertex Buffer
    ↓
  Bind Pipeline & Uniforms
    ↓
  Draw Call (GPU)
      ↓
  Framebuffer → Display
```

**Drawable Implementations**:
- `GCodeDrawer`: Renders toolpath lines/arcs
- `ToolDrawer`: Current tool position (animated)
- `OriginDrawer`: Coordinate system origin marker
- `GridDrawer`: Reference grid
- `HeightMapDrawer`: Surface height visualization
- `BoundsDrawer`: Machine limits box
- `SelectionDrawer`: Selected lines/regions

### 2.4 State Management Module

**Purpose**: Centralized application and machine state

**Design Pattern**: Single source of truth with immutable updates

```rust
pub struct AppState {
    pub machine: MachineState,
    pub program: ProgramState,
    pub settings: Settings,
    pub ui: UiState,
}

pub struct MachineState {
    pub status: MachineStatus,
    pub position: Position,
    pub coordinates: CoordinateSystem,
    pub spindle: SpindleState,
    pub feed_rate: f64,
    pub overrides: Overrides,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MachineStatus {
    Idle,
    Run,
    Hold,
    Jog,
    Alarm,
    Door,
    Check,
    Home,
    Sleep,
}

pub struct Position {
    pub machine: Point3D,  // Machine coordinates
    pub work: Point3D,     // Work coordinates
    pub offset: Point3D,   // WCO
}

pub struct ProgramState {
    pub gcode: Vec<String>,
    pub parsed: Vec<Segment>,
    pub current_line: usize,
    pub execution_state: ExecutionState,
    pub progress: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecutionState {
    Idle,
    Running,
    Paused,
    Stopped,
    Error,
}
```

**State Update Flow**:
```
User Action / GRBL Response
         ↓
    State Update Request
         ↓
  State Manager (validate)
         ↓
  Apply Update (immutable)
         ↓
  Broadcast Change Event
         ↓
  UI Update / Renderer Update
```

**Thread Safety**:
- Shared state wrapped in `Arc<RwLock<AppState>>`
- Read operations use `read()` lock
- Write operations use `write()` lock
- Consider using `parking_lot::RwLock` for better performance

### 2.5 Height Map Module

**Purpose**: Surface scanning and height compensation

**Data Structure**:

```rust
pub struct HeightMap {
    pub grid: Grid2D<f64>,
    pub bounds: Bounds,
    pub resolution: (usize, usize),
    pub probe_points: Vec<ProbePoint>,
}

pub struct Grid2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid2D<f64> {
    pub fn interpolate(&self, x: f64, y: f64) -> f64 {
        // Bilinear or bicubic interpolation
    }
}

pub struct ProbePoint {
    pub position: Point3D,
    pub height: f64,
    pub measured: bool,
}
```

**Probing Sequence**:
```
1. Define probe area (X min/max, Y min/max)
2. Calculate grid points
3. Move to first point (rapids)
4. Probe Z axis (G38.2)
5. Record height
6. Repeat for all points
7. Build height map
8. Apply interpolation for unmeasured points
```

**G-Code Transformation**:
```rust
pub fn apply_heightmap(
    gcode: &[Segment], 
    heightmap: &HeightMap
) -> Vec<Segment> {
    gcode.iter().map(|segment| {
        match segment {
            Segment::Line(line) => {
                let start_z = heightmap.interpolate(line.start.x, line.start.y);
                let end_z = heightmap.interpolate(line.end.x, line.end.y);
                // Create new segment with adjusted Z
            }
            // Handle arcs similarly
        }
    }).collect()
}
```

### 2.6 UI Module

**Purpose**: User interface using egui immediate mode framework

**egui Architecture**:

```rust
use eframe::egui;

// Main application structure
pub struct RCandleApp {
    state: Arc<RwLock<AppState>>,
    connection_mgr: ConnectionManager,
    viewport: Viewport3D,
    
    // UI state
    selected_tab: Tab,
    console_input: String,
    show_settings: bool,
}

impl eframe::App for RCandleApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Top panel - menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        // Open file dialog
                    }
                    if ui.button("Save").clicked() {
                        // Save file
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Settings").clicked() {
                        self.show_settings = true;
                    }
                });
            });
        });
        
        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Split view - code editor and 3D viewport
            let available = ui.available_size();
            
            ui.horizontal(|ui| {
                // Left panel - G-Code editor and console
                ui.vertical(|ui| {
                    ui.set_width(available.x * 0.4);
                    self.show_gcode_editor(ui);
                    self.show_console(ui);
                });
                
                // Right panel - 3D viewport
                ui.vertical(|ui| {
                    self.show_viewport(ui, frame);
                });
            });
        });
        
        // Bottom panel - control panels
        egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
            self.show_control_panels(ui);
        });
        
        // Settings dialog
        if self.show_settings {
            self.show_settings_window(ctx);
        }
        
        // Request repaint for animations
        ctx.request_repaint();
    }
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Cleanup on exit
    }
}

impl RCandleApp {
    fn show_gcode_editor(&mut self, ui: &mut egui::Ui) {
        // G-Code editor implementation
    }
    
    fn show_console(&mut self, ui: &mut egui::Ui) {
        // Console implementation
    }
    
    fn show_viewport(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // 3D viewport with wgpu integration
    }
    
    fn show_control_panels(&mut self, ui: &mut egui::Ui) {
        // State, jog, spindle controls
    }
    
    fn show_settings_window(&mut self, ctx: &egui::Context) {
        // Settings dialog
    }
}
```

**Layout Structure with egui**:
```
Main Window (eframe::App)
├── Top Panel (egui::TopBottomPanel::top)
│   └── Menu Bar (egui::menu::bar)
│       ├── File (Open, Save, Exit)
│       ├── Edit (Settings)
│       ├── View (Panels, Zoom)
│       └── Help (About, Manual)
│
├── Central Panel (egui::CentralPanel)
│   └── Horizontal Split
│       ├── Left Panel (40%)
│       │   ├── G-Code Editor (egui::TextEdit with syntax highlighting)
│       │   └── Console (egui::ScrollArea with text)
│       └── Right Panel (60%)
│           └── 3D Viewport (custom wgpu integration)
│
└── Bottom Panel (egui::TopBottomPanel::bottom)
    ├── Collapsible sections
    ├── State Panel (status display)
    ├── Control Panel (buttons)
    ├── Coordinate System Panel (labels)
    ├── Spindle Panel (controls)
    ├── Jog Panel (button grid)
    ├── Override Panel (sliders)
    └── Height Map Panel (controls)
```

**egui Widgets Used**:
- `egui::TextEdit`: G-Code editor with custom syntax highlighting
- `egui::ScrollArea`: Console log display with auto-scroll
- `egui::Window`: Floating dialogs (settings, about)
- `egui::Button`: Action buttons throughout UI
- `egui::Slider`: Override controls
- `egui::Grid`: Layout for jog controls
- `egui::Label`: Status and coordinate displays
- Custom widget: `Viewport3D` - Embedded WGPU rendering surface

### 2.7 Script Module

**Purpose**: Custom scripting and automation

**Using Rhai Engine**:

```rust
pub struct ScriptEngine {
    engine: rhai::Engine,
    scope: rhai::Scope<'static>,
}

impl ScriptEngine {
    pub fn new(app_state: Arc<RwLock<AppState>>) -> Self {
        let mut engine = rhai::Engine::new();
        
        // Register API functions
        engine.register_fn("send_command", move |cmd: &str| {
            // Send G-Code command
        });
        
        engine.register_fn("get_position", move || {
            // Return current position
        });
        
        engine.register_fn("wait_idle", move || {
            // Wait for machine to be idle
        });
        
        // More API functions...
        
        Self {
            engine,
            scope: rhai::Scope::new(),
        }
    }
    
    pub fn eval(&mut self, script: &str) -> Result<rhai::Dynamic> {
        self.engine.eval_with_scope(&mut self.scope, script)
    }
}
```

**Example User Script**:
```javascript
// User script to probe and set work zero
fn probe_and_zero() {
    send_command("G38.2 Z-10 F50");  // Probe down
    wait_idle();
    send_command("G10 L20 P0 Z0");   // Set Z to zero
    send_command("G0 Z5");           // Retract
}

probe_and_zero();
```

## 3. Communication Patterns

### 3.1 Message Passing

**UI to Connection**:
```rust
// Command channel (bounded)
let (cmd_tx, cmd_rx) = mpsc::channel::<Command>(100);

// UI sends command
cmd_tx.send(Command::GCode("G0 X10".to_string())).await?;

// Connection manager receives and processes
while let Some(cmd) = cmd_rx.recv().await {
    connection.send(cmd.as_bytes()).await?;
}
```

**Connection to UI (Status Updates)**:
```rust
// Broadcast channel for status
let (status_tx, _) = broadcast::channel::<MachineStatus>(10);

// Multiple subscribers
let mut status_rx1 = status_tx.subscribe();
let mut status_rx2 = status_tx.subscribe();

// Connection broadcasts
status_tx.send(MachineStatus::Idle)?;

// Subscribers receive
let status = status_rx1.recv().await?;
```

### 3.2 Async Task Structure

```rust
// Main async runtime
#[tokio::main]
async fn main() {
    // Spawn connection task
    let conn_handle = tokio::spawn(async move {
        connection_loop().await
    });
    
    // Spawn status parser task
    let status_handle = tokio::spawn(async move {
        status_parser_loop().await
    });
    
    // Run UI on main thread (required for graphics)
    run_ui();
    
    // Cleanup
    conn_handle.abort();
    status_handle.abort();
}
```

## 4. Data Flow Examples

### 4.1 Loading and Displaying G-Code

```
User clicks "Open File"
         ↓
   File Dialog (UI)
         ↓
   Read file to String
         ↓
   Parser::parse(gcode_string)
         ↓
   Generate Segments
         ↓
   Update AppState.program
         ↓
   Notify Renderer
         ↓
   GCodeDrawer::update(segments)
         ↓
   Generate vertex buffers
         ↓
   Render scene
```

### 4.2 Sending Command and Getting Response

```
User clicks "🏠" button
         ↓
   UI generates Message::HomeCommand
         ↓
   Command sent to ConnectionManager
         ↓
   ConnectionManager queues "$H"
         ↓
   Serial write "$H\n"
         ↓
   Wait for response
         ↓
   Response "ok" received
         ↓
   Parse and broadcast StatusUpdate
         ↓
   UI receives update
         ↓
   UI displays "Homing complete"
```

### 4.3 Real-time Status Updates

```
GRBL sends status report (every 200ms)
"<Idle|MPos:10.000,20.000,5.000|FS:500,0>"
         ↓
   ConnectionManager receives
         ↓
   StatusParser::parse()
         ↓
   Extract: state=Idle, pos=(10,20,5)
         ↓
   Broadcast MachineStatus
         ↓
   Multiple subscribers:
   ├─→ UI (update position display)
   ├─→ Renderer (update tool position)
   └─→ Logger (log status)
```

## 5. Error Handling Strategy

### 5.1 Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RCandleError {
    #[error("Connection error: {0}")]
    Connection(#[from] std::io::Error),
    
    #[error("Parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
    
    #[error("GRBL error: {0}")]
    Grbl(String),
    
    #[error("File error: {0}")]
    File(String),
    
    #[error("Render error: {0}")]
    Render(String),
    
    #[error("State error: {0}")]
    State(String),
}

pub type Result<T> = std::result::Result<T, RCandleError>;
```

### 5.2 Error Propagation

```rust
// In library code: use Result<T>
pub fn parse_gcode(input: &str) -> Result<Vec<Command>> {
    // Use ? operator for propagation
    let tokens = tokenize(input)?;
    let commands = parse_tokens(tokens)?;
    Ok(commands)
}

// In application code: use anyhow
use anyhow::Context;

pub async fn load_file(path: &Path) -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string(path)
        .await
        .context("Failed to read G-Code file")?;
    Ok(content)
}

// In UI: display user-friendly messages
match load_file(&path).await {
    Ok(content) => { /* success */ }
    Err(e) => {
        show_error_dialog(&format!("Could not load file: {}", e));
    }
}
```

## 6. Performance Optimization Strategies

### 6.1 Parser Optimization
- Use `nom` parser combinators for zero-copy parsing where possible
- Cache parsed segments
- Lazy evaluation for large files
- Stream processing for files too large for memory

### 6.2 Renderer Optimization
- Level-of-detail (LOD) for complex paths
- Frustum culling
- Instanced rendering for repeated geometry
- Vertex buffer reuse
- Occlusion culling for large models

### 6.3 Memory Management
- Use object pooling for frequently allocated objects
- Streaming for large files
- Memory-mapped files for huge G-Code files
- Lazy loading of visualization data

### 6.4 Concurrency
- Offload parsing to background thread
- Async I/O for all file operations
- Parallel processing of independent segments
- Lock-free data structures where appropriate

## 7. Testing Architecture

### 7.1 Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_g0_command() {
        let input = "G0 X10 Y20";
        let result = parse_gcode(input).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].command_type, CommandType::Rapid);
    }
}
```

### 7.2 Integration Tests
```rust
// tests/integration/parser_tests.rs
#[tokio::test]
async fn test_load_and_parse_file() {
    let content = load_test_file("simple.nc").await.unwrap();
    let segments = parse_gcode(&content).unwrap();
    assert!(segments.len() > 0);
}
```

### 7.3 Mock Objects
```rust
pub struct MockConnection {
    sent_data: Vec<Vec<u8>>,
    response_queue: VecDeque<Vec<u8>>,
}

#[async_trait]
impl Connection for MockConnection {
    async fn send(&mut self, data: &[u8]) -> Result<()> {
        self.sent_data.push(data.to_vec());
        Ok(())
    }
    
    async fn receive(&mut self) -> Result<Option<Vec<u8>>> {
        Ok(self.response_queue.pop_front())
    }
}
```

## 8. Build and Deployment

### 8.1 Cargo Configuration

```toml
[package]
name = "rcandle"
version = "0.1.0"
edition = "2021"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.dev]
opt-level = 0
debug = true

[profile.dev.package."*"]
opt-level = 3  # Optimize dependencies even in dev
```

### 8.2 Cross-Platform Builds

**Linux**:
```bash
cargo build --release
```

**Windows** (from Linux):
```bash
cargo install cross
cross build --target x86_64-pc-windows-gnu --release
```

**macOS** (from Linux):
```bash
cargo build --target x86_64-apple-darwin --release
```

### 8.3 Packaging

**Linux AppImage**:
```bash
# Use linuxdeploy or appimage-builder
linuxdeploy --executable target/release/rcandle --output appimage
```

**Windows Installer**:
```bash
# Use cargo-wix or NSIS
cargo install cargo-wix
cargo wix
```

**macOS Bundle**:
```bash
cargo bundle --release
```

## 9. Monitoring and Logging

```rust
use tracing::{info, warn, error, debug, instrument};

#[instrument]
async fn connect_to_grbl(port: &str) -> Result<()> {
    info!("Connecting to GRBL on port {}", port);
    
    match serial::open(port).await {
        Ok(conn) => {
            info!("Connection established");
            Ok(())
        }
        Err(e) => {
            error!("Connection failed: {}", e);
            Err(e.into())
        }
    }
}

// Configure logging
fn init_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
```

## 10. Security Considerations

### 10.1 Input Validation
- Sanitize all file paths
- Validate G-Code syntax before execution
- Limit script capabilities in sandboxed environment
- Rate limiting on command sending

### 10.2 Safe Defaults
- Start with machine in locked/alarm state
- Confirm before executing destructive operations
- Emergency stop easily accessible
- Soft limits enabled by default

### 10.3 Dependency Audit
```bash
cargo audit
cargo deny check
```

---

This architecture provides a solid foundation for the rCandle migration, balancing modern Rust idioms with the practical requirements of CNC machine control.
