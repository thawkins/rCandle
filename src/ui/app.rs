//! Main application structure for rCandle

use crate::{
    connection::{ConnectionManager, ConnectionManagerConfig, SerialConnection},
    grbl::{CommandQueue, GrblCommand, GrblResponse, OverrideCommand, FeedRateOverride, SpindleOverride, RapidOverride},
    parser::{Parser, Preprocessor, Segment, SegmentType, Tokenizer},
    renderer::{Renderer, ViewPreset},
    script::{ScriptLibrary, UserCommandLibrary, UserScript},
    settings::Settings,
    state::{AppState, ExecutionState, MachineStatus},
    ui::widgets::{Console, GCodeEditor},
};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;

/// Main rCandle application state
pub struct RCandleApp {
    /// Application settings
    settings: Settings,
    /// Application state (machine, program, etc.)
    app_state: AppState,
    /// Connection status display
    status_message: String,
    /// Currently loaded file path
    current_file: Option<PathBuf>,
    /// G-Code content
    gcode_content: String,
    /// Parser instance
    parser: Parser,
    /// Preprocessor instance
    preprocessor: Preprocessor,
    /// G-Code editor widget
    gcode_editor: GCodeEditor,
    /// Console widget
    console: Console,
    /// Show console panel
    show_console: bool,
    /// 3D renderer (optional until WGPU is initialized)
    renderer: Option<Renderer>,
    /// Parsed segments for rendering
    segments: Vec<Segment>,
    /// Jog step size (in mm or inches depending on units)
    jog_step_size: f64,
    /// Spindle speed (RPM)
    spindle_speed: f64,
    /// Feed rate override (percentage, 0-200)
    feed_override: f64,
    /// Rapid override (percentage, 25-100)
    rapid_override: f64,
    /// Spindle override (percentage, 0-200)
    spindle_override: f64,
    /// Program execution speed (percentage, 0-200)
    execution_speed: f64,
    /// Step mode enabled
    step_mode: bool,
    /// Program start time (for elapsed time calculation)
    program_start_time: Option<std::time::Instant>,
    /// Program paused time (for pause duration tracking)
    program_paused_time: Option<std::time::Instant>,
    /// Total paused duration
    total_paused_duration: std::time::Duration,
    /// Current executing line number (0-based)
    current_line: usize,
    /// Connection manager (wrapped in Arc<TokioMutex> for async access)
    connection_manager: Option<Arc<TokioMutex<ConnectionManager>>>,
    /// Pending connection manager (set by async connection task)
    pending_connection_manager: Option<Arc<TokioMutex<Option<Arc<TokioMutex<ConnectionManager>>>>>>,
    /// Command queue for GRBL
    _command_queue: Arc<TokioMutex<CommandQueue>>,
    /// Selected serial port for connection
    selected_port: String,
    /// Available serial ports
    available_ports: Vec<String>,
    /// Show settings dialog
    show_settings_dialog: bool,
    /// Temporary settings being edited (None when dialog is closed)
    temp_settings: Option<Settings>,
    /// Script library for user scripts
    script_library: ScriptLibrary,
    /// User command library for custom buttons
    user_command_library: UserCommandLibrary,
    /// Show script editor dialog
    show_script_editor: bool,
    /// Currently editing script (None when not editing)
    editing_script: Option<UserScript>,
    /// Show user commands panel
    show_user_commands: bool,
    /// Previous feed override value (for change detection)
    prev_feed_override: f64,
    /// Previous rapid override value (for change detection)
    prev_rapid_override: f64,
    /// Previous spindle override value (for change detection)
    prev_spindle_override: f64,
    /// Response receiver for GRBL responses
    response_receiver: Option<tokio::sync::broadcast::Receiver<GrblResponse>>,
    /// Status receiver for GRBL status updates
    status_receiver: Option<tokio::sync::broadcast::Receiver<crate::grbl::GrblStatus>>,
}

impl RCandleApp {
    /// Create a new rCandle application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load settings first
        let settings = Settings::load_or_default();
        
        // Apply theme from settings
        Self::apply_theme(&cc.egui_ctx, settings.ui.dark_mode);
        
        // Apply font size from settings
        Self::apply_font_size(&cc.egui_ctx, settings.ui.font_size);
        
        // Configure egui style for better interactivity
        let mut style = (*cc.egui_ctx.style()).clone();
        style.interaction.selectable_labels = true;
        cc.egui_ctx.set_style(style);
        
        // Initialize application state
        let app_state = AppState::new();
        
        // Create parser and preprocessor
        let parser = Parser::new();
        let preprocessor = Preprocessor::new();
        
        // Create G-Code editor
        let gcode_editor = GCodeEditor::new();
        
        // Create console
        let mut console = Console::new();
        console.info("rCandle initialized".to_string());
        console.info("Ready to connect to GRBL device".to_string());
        
        // Initialize WGPU renderer
        let renderer = Self::init_renderer(cc);
        
        if renderer.is_some() {
            console.info("3D renderer initialized".to_string());
        } else {
            console.warning("Failed to initialize 3D renderer".to_string());
        }
        
        tracing::info!("rCandle UI initialized");
        
        // Initialize command queue
        let command_queue = Arc::new(TokioMutex::new(CommandQueue::new()));
        
        // Get available serial ports
        let available_ports = SerialConnection::list_ports()
            .ok()
            .map(|ports| ports.iter().map(|p| p.port_name.clone()).collect())
            .unwrap_or_else(Vec::new);
        
        Self {
            settings,
            app_state,
            status_message: "Ready".to_string(),
            current_file: None,
            gcode_content: String::new(),
            parser,
            preprocessor,
            gcode_editor,
            console,
            show_console: true,
            renderer,
            segments: Vec::new(),
            jog_step_size: 1.0,
            spindle_speed: 1000.0,
            feed_override: 100.0,
            rapid_override: 100.0,
            spindle_override: 100.0,
            execution_speed: 100.0,
            step_mode: false,
            program_start_time: None,
            program_paused_time: None,
            total_paused_duration: std::time::Duration::ZERO,
            current_line: 0,
            connection_manager: None,
            pending_connection_manager: None,
            _command_queue: command_queue,
            selected_port: available_ports.first().cloned().unwrap_or_default(),
            available_ports,
            show_settings_dialog: false,
            temp_settings: None,
            script_library: ScriptLibrary::new(),
            user_command_library: UserCommandLibrary::default(),
            show_script_editor: false,
            editing_script: None,
            show_user_commands: true,
            prev_feed_override: 100.0,
            prev_rapid_override: 100.0,
            prev_spindle_override: 100.0,
            response_receiver: None,
            status_receiver: None,
        }
    }

    /// Initialize WGPU renderer
    fn init_renderer(cc: &eframe::CreationContext<'_>) -> Option<Renderer> {
        // Get WGPU render state from eframe
        let wgpu_render_state = cc.wgpu_render_state.as_ref()?;
        
        let device = wgpu_render_state.device.clone();
        let queue = wgpu_render_state.queue.clone();
        let target_format = wgpu_render_state.target_format;
        
        Some(Renderer::new(device, queue, target_format))
    }

    /// Open a G-Code file
    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("G-Code", &["gcode", "nc", "ngc", "txt"])
            .add_filter("All Files", &["*"])
            .pick_file()
        {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    self.gcode_content = content;
                    self.current_file = Some(path.clone());
                    self.status_message = format!("Loaded: {}", path.display());
                    self.console.info(format!("Loaded file: {}", path.display()));
                    tracing::info!("Loaded G-Code file: {:?}", path);
                    
                    // Parse the G-Code
                    self.parse_gcode();
                }
                Err(e) => {
                    self.status_message = format!("Error loading file: {}", e);
                    self.console.error(format!("Failed to load file: {}", e));
                    tracing::error!("Failed to load file {:?}: {}", path, e);
                }
            }
        }
    }

    /// Save the current G-Code to a file
    fn save_file(&mut self) {
        if let Some(path) = &self.current_file {
            if let Err(e) = std::fs::write(path, &self.gcode_content) {
                self.status_message = format!("Error saving file: {}", e);
                self.console.error(format!("Failed to save file: {}", e));
                tracing::error!("Failed to save file {:?}: {}", path, e);
            } else {
                self.status_message = format!("Saved: {}", path.display());
                self.console.info(format!("Saved file: {}", path.display()));
                tracing::info!("Saved G-Code file: {:?}", path);
            }
        } else {
            self.save_file_as();
        }
    }

    /// Save the current G-Code to a new file
    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("G-Code", &["gcode", "nc", "ngc"])
            .save_file()
        {
            if let Err(e) = std::fs::write(&path, &self.gcode_content) {
                self.status_message = format!("Error saving file: {}", e);
                self.console.error(format!("Failed to save file: {}", e));
                tracing::error!("Failed to save file {:?}: {}", path, e);
            } else {
                self.current_file = Some(path.clone());
                self.status_message = format!("Saved: {}", path.display());
                self.console.info(format!("Saved file: {}", path.display()));
                tracing::info!("Saved G-Code file: {:?}", path);
            }
        }
    }

    /// Parse the current G-Code content
    fn parse_gcode(&mut self) {
        self.console.info("Parsing G-Code...".to_string());
        
        // Tokenize
        let mut tokenizer = Tokenizer::new(&self.gcode_content);
        let tokens = match tokenizer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                self.status_message = format!("Tokenization error: {}", e);
                self.console.error(format!("Tokenization failed: {}", e));
                tracing::error!("Failed to tokenize G-Code: {}", e);
                return;
            }
        };
        
        self.console.debug(format!("Tokenized {} tokens", tokens.len()));

        // Parse tokens to commands
        let commands = match self.parser.parse_tokens(&tokens) {
            Ok(c) => c,
            Err(e) => {
                self.status_message = format!("Parse error: {}", e);
                self.console.error(format!("Parse failed: {}", e));
                tracing::error!("Failed to parse G-Code: {}", e);
                return;
            }
        };
        
        self.console.debug(format!("Parsed {} commands", commands.len()));

        // Generate segments
        let segments = match self.parser.generate_segments(&commands) {
            Ok(s) => s,
            Err(e) => {
                self.status_message = format!("Segment generation error: {}", e);
                self.console.error(format!("Segment generation failed: {}", e));
                tracing::error!("Failed to generate segments: {}", e);
                return;
            }
        };

        let segment_count = segments.len();
        self.console.info(format!("Generated {} segments", segment_count));
        tracing::info!("Parsed {} segments", segment_count);
        
        // Apply preprocessing
        let processed = match self.preprocessor.process(&segments) {
            Ok(p) => p,
            Err(e) => {
                self.status_message = format!("Preprocessing error: {}", e);
                self.console.error(format!("Preprocessing failed: {}", e));
                tracing::error!("Failed to preprocess segments: {}", e);
                return;
            }
        };
        
        let processed_count = processed.len();
        self.console.info(format!("Preprocessed to {} segments", processed_count));
        tracing::info!("Preprocessed to {} segments", processed_count);
        
        // Store segments for rendering
        self.segments = processed.clone();
        
        // Update renderer with new toolpath
        if let Some(ref mut renderer) = self.renderer {
            renderer.set_segments(processed);
            self.console.info("3D view updated with toolpath".to_string());
        }
        
        // Update program state with the parsed data
        let mut program = self.app_state.program.write();
        program.total_lines = self.gcode_content.lines().count();
        
        self.status_message = format!(
            "Parsed {} segments ({} after preprocessing)",
            segment_count, processed_count
        );
        self.console.info("G-Code parsing complete".to_string());
    }

    /// Refresh list of available serial ports
    fn refresh_ports(&mut self) {
        self.available_ports = SerialConnection::list_ports()
            .ok()
            .map(|ports| ports.iter().map(|p| p.port_name.clone()).collect())
            .unwrap_or_else(Vec::new);
        
        if !self.available_ports.is_empty() && !self.available_ports.contains(&self.selected_port) {
            self.selected_port = self.available_ports[0].clone();
        }
        
        self.console.info(format!("Found {} serial port(s)", self.available_ports.len()));
    }

    /// Connect to GRBL device
    fn connect_to_grbl(&mut self, ctx: &egui::Context) {
        if self.selected_port.is_empty() {
            self.status_message = "No port selected".to_string();
            self.console.error("Cannot connect: no port selected".to_string());
            return;
        }
        
        self.status_message = format!("Connecting to {}...", self.selected_port);
        self.console.info(format!("Attempting to connect to {}", self.selected_port));
        
        // Clone data needed for async operation
        let port = self.selected_port.clone();
        let ctx = ctx.clone();
        let app_state = self.app_state.clone();
        
        // Create a shared slot for the connection manager
        let manager_slot = Arc::new(TokioMutex::new(None::<Arc<TokioMutex<ConnectionManager>>>));
        let manager_slot_write = manager_slot.clone();
        
        // Spawn connection task
        tokio::spawn(async move {
            let serial_conn = SerialConnection::new(port.clone(), 115200);
            let config = ConnectionManagerConfig::default();
            let mut manager = ConnectionManager::with_config(Box::new(serial_conn), config);
            
            match manager.connect(Duration::from_secs(5)).await {
                Ok(()) => {
                    tracing::info!("Successfully connected to {}", port);
                    *app_state.connected.write() = true;
                    
                    // Store the manager in the shared slot
                    let manager_arc = Arc::new(TokioMutex::new(manager));
                    *manager_slot_write.lock().await = Some(manager_arc);
                }
                Err(e) => {
                    tracing::error!("Connection failed: {}", e);
                    *app_state.connected.write() = false;
                }
            }
            ctx.request_repaint();
        });
        
        // Store the manager slot so we can retrieve it in the update loop
        self.pending_connection_manager = Some(manager_slot);
    }

    /// Disconnect from GRBL device
    fn disconnect_from_grbl(&mut self) {
        if let Some(manager) = self.connection_manager.take() {
            self.status_message = "Disconnecting...".to_string();
            self.console.info("Disconnecting from device".to_string());
            
            // Spawn disconnect task
            tokio::spawn(async move {
                let mut mgr = manager.lock().await;
                if let Err(e) = mgr.disconnect().await {
                    tracing::error!("Error during disconnect: {}", e);
                }
            });
            
            *self.app_state.connected.write() = false;
            self.status_message = "Disconnected".to_string();
            self.console.info("Disconnected".to_string());
        }
    }

    /// Send a command to GRBL
    fn send_command(&mut self, command: GrblCommand) {
        if self.connection_manager.is_none() {
            self.console.error("Not connected to device".to_string());
            return;
        }
        
        let command_str = command.format();
        self.console.sent(command_str.trim().to_string());
        
        // Clone the manager and send the command via the connection manager
        let manager = Arc::clone(self.connection_manager.as_ref().unwrap());
        tokio::spawn(async move {
            let mgr = manager.lock().await;
            if let Err(e) = mgr.send_command(command).await {
                tracing::error!("Failed to send command: {}", e);
            }
        });
    }

    /// Handle console command submission
    
    /// Send jog command for manual positioning
    fn send_jog_command(&mut self, x: f64, y: f64, z: f64) {
        let feed_rate = if z != 0.0 {
            self.settings.jog.z_feed_rate
        } else {
            self.settings.jog.xy_feed_rate
        };
        
        let command = GrblCommand::Jog {
            x: if x != 0.0 { Some(x) } else { None },
            y: if y != 0.0 { Some(y) } else { None },
            z: if z != 0.0 { Some(z) } else { None },
            feed_rate,
        };
        
        self.send_command(command);
        self.status_message = format!("Jogging: X{:.3} Y{:.3} Z{:.3}", x, y, z);
        tracing::info!("Jog command: X{:.3} Y{:.3} Z{:.3}", x, y, z);
    }
    
    /// Send home command ($H)
    fn send_home_command(&mut self) {
        let command = GrblCommand::HomingCycle;
        self.send_command(command);
        self.status_message = "Homing...".to_string();
        tracing::info!("Home command");
    }
    
    /// Send unlock command ($X) to clear alarm state
    fn send_unlock_command(&mut self) {
        // Send directly to device, bypassing the command queue
        let manager_opt = self.connection_manager.clone();
        self.status_message = "Unlocking alarm...".to_string();
        self.console.info("Sending unlock command ($X)".to_string());
        tracing::info!("Unlock command ($X)");

        if let Some(manager) = manager_opt {
            // Spawn an async task to send raw bytes via send_realtime on the manager
            tokio::spawn(async move {
                let bytes = b"$X\n";
                for &b in bytes.iter() {
                    let _ = manager.lock().await.send_realtime(b).await;
                    // Small delay between bytes to avoid overwhelming device
                    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                }
                tracing::info!("Unlock sequence sent");
            });
        } else {
            self.console.error("Not connected: cannot send unlock sequence".to_string());
        }
    }
    
    /// Zero a specific axis
    fn send_zero_axis(&mut self, axis: char) {
        let gcode = format!("G10 L20 P0 {}0", axis);
        let command = GrblCommand::GCode(gcode.clone());
        self.send_command(command);
        self.status_message = format!("Zeroing {} axis", axis);
        tracing::info!("Zero axis: {}", axis);
    }
    
    /// Zero all axes
    fn send_zero_all(&mut self) {
        let gcode = "G10 L20 P0 X0 Y0 Z0".to_string();
        let command = GrblCommand::GCode(gcode.clone());
        self.send_command(command);
        self.status_message = "Zeroing all axes".to_string();
        tracing::info!("Zero all axes");
    }
    
    /// Send work coordinate system command
    fn send_wcs_command(&mut self, wcs: u32) {
        let command = format!("G{}", wcs);
        self.console.sent(command.clone());
        self.status_message = format!("Switching to G{}", wcs);
        
        // TODO: Send to GRBL via connection manager
        tracing::info!("WCS command: G{}", wcs);
    }
    
    /// Handle a response received from GRBL
    fn handle_grbl_response(&mut self, response: GrblResponse) {
        // Skip status reports - they're handled separately and would flood the console
        // Issue #4: Don't display status updates in console
        if matches!(response, GrblResponse::Status(_)) {
            tracing::debug!("GRBL status response: {:?}", response);
            return;
        }
        
        // Format the response for display
        let response_text = match &response {
            GrblResponse::Ok => "ok".to_string(),
            GrblResponse::Error(code) => {
                let msg = response.error_message().unwrap_or("Unknown error");
                format!("error:{} ({})", code, msg)
            }
            GrblResponse::Alarm(code) => {
                let msg = response.error_message().unwrap_or("Unknown alarm");
                format!("ALARM:{} ({})", code, msg)
            }
            GrblResponse::Status(_) => {
                // This case is now unreachable due to early return above
                unreachable!()
            }
            GrblResponse::Welcome { version } => {
                format!("Grbl {} ['$' for help]", version)
            }
            GrblResponse::Setting { number, value } => {
                format!("${}={}", number, value)
            }
            GrblResponse::Feedback(msg) => {
                format!("[{}]", msg)
            }
            GrblResponse::Message(msg) => {
                msg.clone()
            }
        };
        
        // Add to console with appropriate styling
        if response.is_error() || response.is_alarm() {
            self.console.error(response_text);
        } else {
            self.console.received(response_text);
        }
        
        tracing::debug!("GRBL response: {:?}", response);
    }
    
    /// Handle GRBL status update - Issue #1
    /// 
    /// This method processes status reports from GRBL (received in response to `?` queries)
    /// and updates the machine state accordingly.
    fn handle_grbl_status_update(&mut self, status: crate::grbl::GrblStatus) {
        // Update machine state from the GRBL status
        let mut machine = self.app_state.machine.write();
        machine.update_from_grbl_status(&status);
        drop(machine);
        
        // Log status updates (reduced frequency to avoid spam)
        static STATUS_COUNT: AtomicUsize = AtomicUsize::new(0);
        let count = STATUS_COUNT.fetch_add(1, Ordering::Relaxed);
        if count % 10 == 0 {  // Log every 10th status update
            tracing::debug!("Status update: state={:?}, MPos={:?}, WPos={:?}", 
                status.state, status.mpos, status.wpos);
        }
    }
    
    /// Send spindle control command
    fn send_spindle_command(&mut self, cw: bool, ccw: bool) {
        let command = if cw {
            format!("M3 S{:.0}", self.spindle_speed)
        } else if ccw {
            format!("M4 S{:.0}", self.spindle_speed)
        } else {
            "M5".to_string()
        };
        
        self.console.sent(command.clone());
        self.status_message = if cw {
            format!("Spindle CW at {:.0} RPM", self.spindle_speed)
        } else if ccw {
            format!("Spindle CCW at {:.0} RPM", self.spindle_speed)
        } else {
            "Spindle off".to_string()
        };
        
        // TODO: Send to GRBL via connection manager
        tracing::info!("Spindle command: {}", command);
    }

    /// Send feed rate override command to GRBL
    fn send_feed_override(&mut self, target_percent: f64) {
        if self.connection_manager.is_none() {
            return; // Silently skip if not connected
        }
        
        let current = self.prev_feed_override;
        let diff = target_percent - current;
        
        if diff.abs() < 0.5 {
            return; // No significant change
        }
        
        // Determine which override commands to send
        if diff.abs() >= 10.0 {
            // Use coarse adjustments for large changes
            let steps = (diff / 10.0).round() as i32;
            let cmd = if steps > 0 {
                OverrideCommand::FeedRate(FeedRateOverride::CoarseUp)
            } else {
                OverrideCommand::FeedRate(FeedRateOverride::CoarseDown)
            };
            
            for _ in 0..steps.abs() {
                self.send_realtime_byte(cmd.to_byte());
            }
        } else {
            // Use fine adjustments for small changes
            let steps = diff.round() as i32;
            let cmd = if steps > 0 {
                OverrideCommand::FeedRate(FeedRateOverride::FineUp)
            } else {
                OverrideCommand::FeedRate(FeedRateOverride::FineDown)
            };
            
            for _ in 0..steps.abs() {
                self.send_realtime_byte(cmd.to_byte());
            }
        }
        
        self.prev_feed_override = target_percent;
        self.console.info(format!("Feed override: {:.0}%", target_percent));
        tracing::debug!("Feed rate override: {:.0}%", target_percent);
    }

    /// Send rapid override command to GRBL
    fn send_rapid_override(&mut self, target_percent: f64) {
        if self.connection_manager.is_none() {
            return; // Silently skip if not connected
        }
        
        let current = self.prev_rapid_override;
        
        if (target_percent - current).abs() < 0.5 {
            return; // No significant change
        }
        
        // GRBL rapid override is discrete: 25%, 50%, or 100%
        let cmd = if target_percent <= 25.0 {
            OverrideCommand::Rapid(RapidOverride::Low)
        } else if target_percent <= 50.0 {
            OverrideCommand::Rapid(RapidOverride::Medium)
        } else {
            OverrideCommand::Rapid(RapidOverride::Reset)
        };
        
        self.send_realtime_byte(cmd.to_byte());
        self.prev_rapid_override = target_percent;
        self.console.info(format!("Rapid override: {:.0}%", target_percent));
        tracing::debug!("Rapid override: {:.0}%", target_percent);
    }

    /// Send spindle override command to GRBL
    fn send_spindle_override(&mut self, target_percent: f64) {
        if self.connection_manager.is_none() {
            return; // Silently skip if not connected
        }
        
        let current = self.prev_spindle_override;
        let diff = target_percent - current;
        
        if diff.abs() < 0.5 {
            return; // No significant change
        }
        
        // Determine which override commands to send
        if diff.abs() >= 10.0 {
            // Use coarse adjustments for large changes
            let steps = (diff / 10.0).round() as i32;
            let cmd = if steps > 0 {
                OverrideCommand::SpindleSpeed(SpindleOverride::CoarseUp)
            } else {
                OverrideCommand::SpindleSpeed(SpindleOverride::CoarseDown)
            };
            
            for _ in 0..steps.abs() {
                self.send_realtime_byte(cmd.to_byte());
            }
        } else {
            // Use fine adjustments for small changes
            let steps = diff.round() as i32;
            let cmd = if steps > 0 {
                OverrideCommand::SpindleSpeed(SpindleOverride::FineUp)
            } else {
                OverrideCommand::SpindleSpeed(SpindleOverride::FineDown)
            };
            
            for _ in 0..steps.abs() {
                self.send_realtime_byte(cmd.to_byte());
            }
        }
        
        self.prev_spindle_override = target_percent;
        self.console.info(format!("Spindle override: {:.0}%", target_percent));
        tracing::debug!("Spindle speed override: {:.0}%", target_percent);
    }

    /// Send a real-time command byte to GRBL
    fn send_realtime_byte(&mut self, byte: u8) {
        if let Some(ref manager) = self.connection_manager {
            let manager = Arc::clone(manager);
            tokio::spawn(async move {
                let mgr = manager.lock().await;
                if let Err(e) = mgr.send_realtime(byte).await {
                    tracing::error!("Failed to send real-time command: {}", e);
                }
            });
        }
    }

    fn handle_console_command(&mut self, command: &str) {
        let cmd = command.trim();
        
        if cmd.is_empty() {
            return;
        }
        
        // Log the command response (simulate sending to GRBL)
        self.console.info(format!("Sending command: {}", cmd));
        
        // TODO: Send command to GRBL via connection manager
        // For now, just simulate a response
        self.console.received("ok".to_string());
        
        tracing::info!("Console command: {}", cmd);
    }
    
    /// Start program execution
    fn start_program(&mut self) {
        let mut program_state = self.app_state.program.write();
        
        // Check if we have a program loaded
        if program_state.total_lines == 0 {
            self.console.warning("No program loaded".to_string());
            drop(program_state);
            self.status_message = "No program loaded".to_string();
            return;
        }
        
        // Start or resume execution
        match program_state.state {
            ExecutionState::NotLoaded => {
                self.console.warning("No program loaded".to_string());
                drop(program_state);
                return;
            }
            ExecutionState::Loaded | ExecutionState::Completed => {
                // Start from beginning
                program_state.state = ExecutionState::Running;
                program_state.current_line = 0;
                program_state.lines_sent = 0;
                program_state.lines_completed = 0;
                self.current_line = 0;
                self.program_start_time = Some(std::time::Instant::now());
                self.total_paused_duration = std::time::Duration::ZERO;
                self.console.info("Program started".to_string());
                self.status_message = "Program started".to_string();
                tracing::info!("Program execution started");
            }
            ExecutionState::Paused => {
                // Resume from pause
                program_state.state = ExecutionState::Running;
                if let Some(paused_time) = self.program_paused_time.take() {
                    self.total_paused_duration += paused_time.elapsed();
                }
                self.console.info("Program resumed".to_string());
                self.status_message = "Program resumed".to_string();
                tracing::info!("Program execution resumed");
            }
            ExecutionState::Running => {
                self.console.warning("Program already running".to_string());
            }
            ExecutionState::Error => {
                self.console.warning("Cannot start - program in error state. Reset first.".to_string());
            }
        }
        
        drop(program_state);
        
        // TODO: Send to GRBL via connection manager
    }
    
    /// Pause program execution
    fn pause_program(&mut self) {
        let mut program_state = self.app_state.program.write();
        
        if matches!(program_state.state, ExecutionState::Running) {
            program_state.state = ExecutionState::Paused;
            self.program_paused_time = Some(std::time::Instant::now());
            self.console.info("Program paused".to_string());
            self.status_message = "Program paused".to_string();
            tracing::info!("Program execution paused");
            
            // TODO: Send pause command to GRBL (feed hold)
        } else {
            self.console.warning("Program is not running".to_string());
        }
        
        drop(program_state);
    }
    
    /// Stop program execution
    fn stop_program(&mut self) {
        let mut program_state = self.app_state.program.write();
        
        if !matches!(program_state.state, ExecutionState::Loaded) {
            program_state.state = ExecutionState::Loaded;
            self.program_start_time = None;
            self.program_paused_time = None;
            self.total_paused_duration = std::time::Duration::ZERO;
            self.console.warning("Program stopped".to_string());
            self.status_message = "Program stopped".to_string();
            tracing::info!("Program execution stopped");
            
            // TODO: Send stop command to GRBL (soft reset or queue clear)
        } else {
            self.console.warning("Program is not running".to_string());
        }
        
        drop(program_state);
    }
    
    /// Reset program to beginning
    fn reset_program(&mut self) {
        let mut program_state = self.app_state.program.write();
        
        program_state.state = ExecutionState::Loaded;
        program_state.current_line = 0;
        program_state.lines_sent = 0;
        program_state.lines_completed = 0;
        self.current_line = 0;
        self.program_start_time = None;
        self.program_paused_time = None;
        self.total_paused_duration = std::time::Duration::ZERO;
        
        self.console.info("Program reset".to_string());
        self.status_message = "Program reset".to_string();
        tracing::info!("Program reset to beginning");
        
        drop(program_state);
    }
    
    /// Execute a single step in step mode
    fn execute_single_step(&mut self) {
        let mut program_state = self.app_state.program.write();
        
        // Check if we're in a valid state to step
        if program_state.total_lines == 0 {
            self.console.warning("No program loaded".to_string());
            drop(program_state);
            return;
        }
        
        if self.current_line >= program_state.total_lines {
            self.console.info("End of program reached".to_string());
            program_state.state = ExecutionState::Completed;
            drop(program_state);
            return;
        }
        
        // Execute next line
        self.current_line += 1;
        program_state.current_line = self.current_line;
        program_state.lines_completed = self.current_line;
        
        self.console.debug(format!("Step: executing line {}", self.current_line));
        tracing::debug!("Step mode: executing line {}", self.current_line);
        
        // TODO: Send single line to GRBL
        
        drop(program_state);
    }
    
    /// Calculate time estimates for program execution
    fn calculate_time_estimates(&self) -> (String, String) {
        let program_state = self.app_state.program.read();
        
        // Calculate elapsed time
        let elapsed = if let Some(start_time) = self.program_start_time {
            let total_elapsed = start_time.elapsed();
            let active_elapsed = if let Some(paused_time) = self.program_paused_time {
                // Currently paused - subtract pause duration
                total_elapsed - self.total_paused_duration - paused_time.elapsed()
            } else {
                // Not paused - just subtract total paused duration
                total_elapsed - self.total_paused_duration
            };
            active_elapsed
        } else {
            std::time::Duration::ZERO
        };
        
        let elapsed_text = format_duration(elapsed);
        
        // Calculate remaining time estimate
        let remaining_text = if self.current_line > 0 && program_state.total_lines > self.current_line {
            let progress = self.current_line as f64 / program_state.total_lines as f64;
            let estimated_total = elapsed.as_secs_f64() / progress;
            let remaining_secs = estimated_total - elapsed.as_secs_f64();
            let remaining = std::time::Duration::from_secs_f64(remaining_secs.max(0.0));
            format_duration(remaining)
        } else if matches!(program_state.state, ExecutionState::Completed) {
            "Complete".to_string()
        } else {
            "--:--:--".to_string()
        };
        
        drop(program_state);
        
        (elapsed_text, remaining_text)
    }

    /// Draw toolpath in 2D (XY plane projection)
    fn draw_toolpath_2d(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        use egui::{Color32, Pos2, Stroke};
        
        if self.segments.is_empty() {
            return;
        }
        
        // Calculate bounding box
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        
        for segment in &self.segments {
            min_x = min_x.min(segment.start.x).min(segment.end.x);
            max_x = max_x.max(segment.start.x).max(segment.end.x);
            min_y = min_y.min(segment.start.y).min(segment.end.y);
            max_y = max_y.max(segment.start.y).max(segment.end.y);
        }
        
        // Add some padding
        let padding = 20.0;
        let width = (max_x - min_x) as f32;
        let height = (max_y - min_y) as f32;
        
        if width == 0.0 || height == 0.0 {
            return;
        }
        
        // Calculate scale to fit in viewport
        let viewport_width = rect.width() - padding * 2.0;
        let viewport_height = rect.height() - padding * 2.0;
        let scale = (viewport_width / width).min(viewport_height / height);
        
        // Center offset
        let offset_x = rect.left() + padding + (viewport_width - width * scale) / 2.0;
        let offset_y = rect.top() + padding + (viewport_height - height * scale) / 2.0;
        
        // Transform function from G-Code coordinates to screen coordinates
        let to_screen = |x: f64, y: f64| {
            Pos2::new(
                offset_x + ((x - min_x) as f32 * scale),
                // Flip Y axis (G-Code Y increases upward, screen Y increases downward)
                offset_y + viewport_height - ((y - min_y) as f32 * scale),
            )
        };
        
        // Draw grid
        let grid_color = Color32::from_rgb(40, 40, 50);
        let grid_spacing = 10.0; // mm
        
        // Vertical grid lines
        let mut x = (min_x / grid_spacing).floor() * grid_spacing;
        while x <= max_x {
            let p1 = to_screen(x, min_y);
            let p2 = to_screen(x, max_y);
            ui.painter().line_segment([p1, p2], Stroke::new(1.0, grid_color));
            x += grid_spacing;
        }
        
        // Horizontal grid lines
        let mut y = (min_y / grid_spacing).floor() * grid_spacing;
        while y <= max_y {
            let p1 = to_screen(min_x, y);
            let p2 = to_screen(max_x, y);
            ui.painter().line_segment([p1, p2], Stroke::new(1.0, grid_color));
            y += grid_spacing;
        }
        
        // Draw axes
        let origin = to_screen(0.0, 0.0);
        if min_x <= 0.0 && max_x >= 0.0 && min_y <= 0.0 && max_y >= 0.0 {
            // X axis (red)
            let x_end = to_screen(max_x, 0.0);
            ui.painter().line_segment(
                [origin, x_end],
                Stroke::new(2.0, Color32::from_rgb(200, 50, 50)),
            );
            
            // Y axis (green)
            let y_end = to_screen(0.0, max_y);
            ui.painter().line_segment(
                [origin, y_end],
                Stroke::new(2.0, Color32::from_rgb(50, 200, 50)),
            );
        }
        
        // Draw toolpath segments
        for segment in &self.segments {
            let start = to_screen(segment.start.x, segment.start.y);
            let end = to_screen(segment.end.x, segment.end.y);
            
            // Color based on segment type
            let (color, width) = match segment.segment_type {
                SegmentType::Rapid => (Color32::from_rgb(255, 100, 100), 1.0), // Red for rapids
                SegmentType::Linear => (Color32::from_rgb(100, 255, 100), 2.0), // Green for cuts
                SegmentType::ArcCW | SegmentType::ArcCCW => {
                    (Color32::from_rgb(100, 150, 255), 2.0) // Blue for arcs
                }
            };
            
            ui.painter().line_segment([start, end], Stroke::new(width, color));
        }
        
        // Draw start point marker
        if let Some(first) = self.segments.first() {
            let start = to_screen(first.start.x, first.start.y);
            ui.painter().circle_filled(start, 4.0, Color32::from_rgb(100, 255, 255));
            ui.painter().circle_stroke(start, 4.0, Stroke::new(1.0, Color32::WHITE));
        }
    }
    
    /// Apply theme (dark/light mode) to the UI
    fn apply_theme(ctx: &egui::Context, dark_mode: bool) {
        if dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
    }
    
    /// Apply font size to the UI
    fn apply_font_size(ctx: &egui::Context, font_size: f32) {
        let mut style = (*ctx.style()).clone();
        
        // Update text styles with new font size
        for (_text_style, font_id) in style.text_styles.iter_mut() {
            font_id.size = font_size;
        }
        
        ctx.set_style(style);
    }
    
    /// Show settings dialog window
    fn show_settings_window(&mut self, ctx: &egui::Context) {
        let mut open = true;
        let mut should_close = false;
        let mut should_save = false;
        let mut should_reset = false;
        
        egui::Window::new("⚙ Settings")
            .open(&mut open)
            .default_size([600.0, 500.0])
            .resizable(true)
            .show(ctx, |ui| {
                if let Some(ref mut temp_settings) = self.temp_settings {
                    // Tabs for different settings categories
                    egui::TopBottomPanel::top("settings_tabs").show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            let _ = ui.selectable_label(false, "General");
                            let _ = ui.selectable_label(false, "Connection");
                            let _ = ui.selectable_label(false, "Visualization");
                            let _ = ui.selectable_label(false, "Jog");
                            let _ = ui.selectable_label(false, "UI");
                        });
                    });
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        Self::show_general_settings(ui, &mut temp_settings.general);
                        
                        ui.separator();
                        ui.add_space(10.0);
                        
                        Self::show_connection_settings(ui, &mut temp_settings.connection);
                        
                        ui.separator();
                        ui.add_space(10.0);
                        
                        Self::show_visualization_settings(ui, &mut temp_settings.visualization);
                        
                        ui.separator();
                        ui.add_space(10.0);
                        
                        Self::show_jog_settings(ui, &mut temp_settings.jog);
                        
                        ui.separator();
                        ui.add_space(10.0);
                        
                        Self::show_ui_settings(ui, &mut temp_settings.ui);
                    });
                    
                    ui.separator();
                    
                    // Bottom buttons
                    ui.horizontal(|ui| {
                        if ui.button("💾 Save").clicked() {
                            should_save = true;
                        }
                        
                        if ui.button("🔄 Reset to Defaults").clicked() {
                            should_reset = true;
                        }
                        
                        if ui.button("❌ Cancel").clicked() {
                            should_close = true;
                        }
                    });
                }
            });
        
        // Handle actions outside the closure to avoid borrowing issues
        if should_save {
            if let Some(ref temp_settings) = self.temp_settings {
                // Check if theme or font size changed
                let theme_changed = self.settings.ui.dark_mode != temp_settings.ui.dark_mode;
                let font_changed = self.settings.ui.font_size != temp_settings.ui.font_size;
                
                self.settings = temp_settings.clone();
                
                // Apply theme and font changes immediately
                if theme_changed {
                    Self::apply_theme(ctx, self.settings.ui.dark_mode);
                }
                if font_changed {
                    Self::apply_font_size(ctx, self.settings.ui.font_size);
                }
                
                if let Err(e) = self.settings.save_default() {
                    self.console.error(format!("Failed to save settings: {}", e));
                } else {
                    self.console.info("Settings saved".to_string());
                    if theme_changed || font_changed {
                        self.console.info("Theme/font changes applied".to_string());
                    }
                }
            }
            self.show_settings_dialog = false;
            self.temp_settings = None;
        }
        
        if should_reset {
            self.temp_settings = Some(crate::settings::Settings::default());
            self.console.info("Settings reset to defaults".to_string());
        }
        
        if should_close || !open {
            self.show_settings_dialog = false;
            self.temp_settings = None;
        }
    }
    
    /// Show general settings
    fn show_general_settings(ui: &mut egui::Ui, settings: &mut crate::settings::GeneralSettings) {
        ui.heading("General Settings");
        ui.add_space(5.0);
        
        egui::Grid::new("general_settings_grid")
            .num_columns(2)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                ui.label("Units:");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut settings.units_metric, true, "Metric (mm)")
                        .on_hover_text("Use millimeters for measurements");
                    ui.radio_value(&mut settings.units_metric, false, "Imperial (inches)")
                        .on_hover_text("Use inches for measurements");
                });
                ui.end_row();
                
                ui.label("Arc Precision (°):")
                    .on_hover_text("Angle between arc interpolation segments");
                ui.add(egui::DragValue::new(&mut settings.arc_precision)
                    .speed(0.1)
                    .range(0.1..=10.0));
                ui.end_row();
                
                ui.label("Arc Segments:")
                    .on_hover_text("Number of line segments per arc");
                ui.add(egui::DragValue::new(&mut settings.arc_segments)
                    .speed(1)
                    .range(4..=100));
                ui.end_row();
                
                ui.label("Safe Z Height:")
                    .on_hover_text("Height to retract to before rapid moves");
                ui.add(egui::DragValue::new(&mut settings.safe_z)
                    .speed(0.1)
                    .range(0.0..=100.0)
                    .suffix(if settings.units_metric { " mm" } else { " in" }));
                ui.end_row();
            });
    }
    
    /// Show connection settings
    fn show_connection_settings(ui: &mut egui::Ui, settings: &mut crate::settings::ConnectionSettings) {
        ui.heading("Connection Settings");
        ui.add_space(5.0);
        
        egui::Grid::new("connection_settings_grid")
            .num_columns(2)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                ui.label("Port Name:");
                ui.text_edit_singleline(&mut settings.port_name);
                ui.end_row();
                
                ui.label("Baud Rate:");
                egui::ComboBox::from_id_source("baud_rate_combo")
                    .selected_text(format!("{}", settings.baud_rate))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut settings.baud_rate, 9600, "9600");
                        ui.selectable_value(&mut settings.baud_rate, 19200, "19200");
                        ui.selectable_value(&mut settings.baud_rate, 38400, "38400");
                        ui.selectable_value(&mut settings.baud_rate, 57600, "57600");
                        ui.selectable_value(&mut settings.baud_rate, 115200, "115200");
                        ui.selectable_value(&mut settings.baud_rate, 230400, "230400");
                    });
                ui.end_row();
                
                ui.label("Connection Timeout:");
                ui.add(egui::DragValue::new(&mut settings.timeout_ms)
                    .speed(100)
                    .range(100..=30000)
                    .suffix(" ms"));
                ui.end_row();
                
                ui.label("Command Timeout:");
                ui.add(egui::DragValue::new(&mut settings.command_timeout_ms)
                    .speed(100)
                    .range(100..=60000)
                    .suffix(" ms"));
                ui.end_row();
                
                ui.label("Status Query Interval:");
                ui.add(egui::DragValue::new(&mut settings.status_query_interval_ms)
                    .speed(10)
                    .range(50..=5000)
                    .suffix(" ms"));
                ui.end_row();
                
                ui.label("Auto-connect on Startup:");
                ui.checkbox(&mut settings.auto_connect, "");
                ui.end_row();
            });
    }
    
    /// Show visualization settings
    fn show_visualization_settings(ui: &mut egui::Ui, settings: &mut crate::settings::VisualizationSettings) {
        ui.heading("Visualization Settings");
        ui.add_space(5.0);
        
        egui::Grid::new("visualization_settings_grid")
            .num_columns(2)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                ui.label("Show Grid:");
                ui.checkbox(&mut settings.show_grid, "");
                ui.end_row();
                
                ui.label("Grid Size:");
                ui.add(egui::DragValue::new(&mut settings.grid_size)
                    .speed(1.0)
                    .range(1.0..=100.0));
                ui.end_row();
                
                ui.label("Show Tool:");
                ui.checkbox(&mut settings.show_tool, "");
                ui.end_row();
                
                ui.label("Show Origin:");
                ui.checkbox(&mut settings.show_origin, "");
                ui.end_row();
                
                ui.label("Show Bounds:");
                ui.checkbox(&mut settings.show_bounds, "");
                ui.end_row();
                
                ui.label("MSAA Samples:");
                egui::ComboBox::from_id_source("msaa_combo")
                    .selected_text(format!("{}x", settings.msaa_samples))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut settings.msaa_samples, 1, "1x");
                        ui.selectable_value(&mut settings.msaa_samples, 2, "2x");
                        ui.selectable_value(&mut settings.msaa_samples, 4, "4x");
                        ui.selectable_value(&mut settings.msaa_samples, 8, "8x");
                    });
                ui.end_row();
                
                ui.label("VSync:");
                ui.checkbox(&mut settings.vsync, "");
                ui.end_row();
                
                ui.label("Field of View:");
                ui.add(egui::Slider::new(&mut settings.fov, 30.0..=120.0)
                    .suffix("°"));
                ui.end_row();
                
                ui.label("Camera Speed:");
                ui.add(egui::Slider::new(&mut settings.camera_speed, 0.1..=5.0));
                ui.end_row();
            });
    }
    
    /// Show jog settings
    fn show_jog_settings(ui: &mut egui::Ui, settings: &mut crate::settings::JogSettings) {
        ui.heading("Jog Settings");
        ui.add_space(5.0);
        
        egui::Grid::new("jog_settings_grid")
            .num_columns(2)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                ui.label("XY Feed Rate:");
                ui.add(egui::DragValue::new(&mut settings.xy_feed_rate)
                    .speed(10.0)
                    .range(1.0..=10000.0)
                    .suffix(" mm/min"));
                ui.end_row();
                
                ui.label("Z Feed Rate:");
                ui.add(egui::DragValue::new(&mut settings.z_feed_rate)
                    .speed(10.0)
                    .range(1.0..=5000.0)
                    .suffix(" mm/min"));
                ui.end_row();
                
                ui.label("Continuous Mode:");
                ui.checkbox(&mut settings.continuous_mode, "");
                ui.end_row();
            });
        
        ui.add_space(10.0);
        ui.label("Step Sizes:");
        
        // Show step sizes as editable list
        let mut i = 0;
        while i < settings.step_sizes.len() {
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut settings.step_sizes[i])
                    .speed(0.1)
                    .range(0.001..=1000.0));
                
                if ui.button("🗑").clicked() {
                    settings.step_sizes.remove(i);
                    if settings.default_step_index >= settings.step_sizes.len() {
                        settings.default_step_index = settings.step_sizes.len().saturating_sub(1);
                    }
                } else {
                    i += 1;
                }
            });
        }
        
        if ui.button("➕ Add Step Size").clicked() {
            settings.step_sizes.push(1.0);
        }
    }
    
    /// Show UI settings
    fn show_ui_settings(ui: &mut egui::Ui, settings: &mut crate::settings::UiSettings) {
        ui.heading("UI Settings");
        ui.add_space(5.0);
        
        egui::Grid::new("ui_settings_grid")
            .num_columns(2)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                ui.label("Dark Mode:");
                ui.checkbox(&mut settings.dark_mode, "");
                ui.end_row();
                
                ui.label("Font Size:");
                ui.add(egui::Slider::new(&mut settings.font_size, 8.0..=24.0));
                ui.end_row();
                
                ui.label("Show Console:");
                ui.checkbox(&mut settings.show_console, "");
                ui.end_row();
                
                ui.label("Show State Panel:");
                ui.checkbox(&mut settings.show_state, "");
                ui.end_row();
                
                ui.label("Show Control Panel:");
                ui.checkbox(&mut settings.show_control, "");
                ui.end_row();
                
                ui.label("Console History Limit:");
                ui.add(egui::DragValue::new(&mut settings.console_history_limit)
                    .speed(10)
                    .range(100..=10000));
                ui.end_row();
            });
    }
    
    /// Apply a view preset to the camera
    fn apply_view_preset(&mut self, preset: ViewPreset) {
        if let Some(ref mut renderer) = self.renderer {
            // Calculate center and distance from bounds
            let bounds = renderer.calculate_bounds();
            let center = glam::Vec3::new(
                (bounds.0.x + bounds.1.x) / 2.0,
                (bounds.0.y + bounds.1.y) / 2.0,
                (bounds.0.z + bounds.1.z) / 2.0,
            );
            
            let size = glam::Vec3::new(
                (bounds.1.x - bounds.0.x).abs(),
                (bounds.1.y - bounds.0.y).abs(),
                (bounds.1.z - bounds.0.z).abs(),
            );
            let distance = size.max_element() * 2.0;
            
            // Apply the preset
            renderer.apply_view_preset(preset, center, distance);
            
            self.status_message = format!("Applied {:?} view", preset);
            self.console.info(format!("Camera set to {:?} view", preset));
        }
    }
    
    /// Execute a user command
    fn execute_user_command(&mut self, command_name: &str) {
        // Clone commands first to avoid borrowing issues
        let commands = self.user_command_library.get_command(command_name)
            .map(|c| c.commands.clone());
        
        if let Some(cmds) = commands {
            self.console.info(format!("Executing user command: {}", command_name));
            
            for cmd in cmds {
                self.send_command(GrblCommand::GCode(cmd));
            }
            
            self.status_message = format!("Executed: {}", command_name);
        } else {
            self.console.error(format!("User command not found: {}", command_name));
        }
    }
    
    /// Show script editor window
    fn show_script_editor_window(&mut self, ctx: &egui::Context) {
        let mut dialog_open = true;
        let mut action = None; // To store actions to perform after the UI
        
        egui::Window::new("Script Editor")
            .open(&mut dialog_open)
            .default_width(600.0)
            .default_height(400.0)
            .show(ctx, |ui| {
                if let Some(ref mut script) = self.editing_script {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut script.name);
                    });
                    
                    ui.separator();
                    
                    // Multi-line text editor for script content
                    ui.label("Script:");
                    egui::ScrollArea::vertical()
                        .max_height(250.0)
                        .show(ui, |ui| {
                            ui.add(egui::TextEdit::multiline(&mut script.code)
                                .code_editor()
                                .desired_width(f32::INFINITY)
                                .desired_rows(15));
                        });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut script.show_in_toolbar, "Show in Toolbar");
                    });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("💾 Save").clicked() {
                            action = Some(("save", script.name.clone()));
                        }
                        
                        if ui.button("▶ Test Run").clicked() {
                            action = Some(("test", script.name.clone()));
                        }
                        
                        if ui.button("❌ Cancel").clicked() {
                            action = Some(("cancel", String::new()));
                        }
                    });
                } else {
                    ui.label("No script loaded");
                    
                    if ui.button("Close").clicked() {
                        action = Some(("close", String::new()));
                    }
                }
            });
        
        // Process actions after the UI closure
        if let Some((act, name)) = action {
            match act {
                "save" => {
                    if let Some(script) = &self.editing_script {
                        self.script_library.add_script(script.clone());
                        self.console.info(format!("Saved script: {}", name));
                        self.status_message = format!("Script saved: {}", name);
                    }
                    self.show_script_editor = false;
                    self.editing_script = None;
                }
                "test" => {
                    self.console.info(format!("Testing script: {}", name));
                    // TODO: Execute script via script executor
                    self.status_message = format!("Testing: {}", name);
                }
                "cancel" | "close" => {
                    self.show_script_editor = false;
                    self.editing_script = None;
                }
                _ => {}
            }
        }
        
        if !dialog_open {
            self.show_script_editor = false;
            self.editing_script = None;
        }
    }
    
}

impl eframe::App for RCandleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for pending connection manager from async connection task
        let mut manager_to_store = None;
        let mut clear_pending = false;
        
        if let Some(pending_slot) = &self.pending_connection_manager {
            // Try to get the manager without blocking
            if let Ok(mut slot_guard) = pending_slot.try_lock() {
                if let Some(manager) = slot_guard.take() {
                    // We got the manager! Store it temporarily
                    manager_to_store = Some(manager);
                    clear_pending = true;
                }
            }
        }
        
        // Now update the fields outside the borrow
        if let Some(manager) = manager_to_store {
            // Subscribe to responses and status before storing the manager
            let manager_guard = tokio::runtime::Handle::current().block_on(manager.lock());
            let response_rx = manager_guard.subscribe_responses();
            let status_rx = manager_guard.subscribe_status();
            drop(manager_guard);
            
            self.response_receiver = Some(response_rx);
            self.status_receiver = Some(status_rx);
            self.connection_manager = Some(manager);
            self.status_message = "Connected".to_string();
            self.console.info("Connection established".to_string());
            tracing::info!("Connection manager stored successfully");
        }
        if clear_pending {
            self.pending_connection_manager = None;
        }
        
        // Check for responses from GRBL
        let mut responses = Vec::new();
        if let Some(ref mut rx) = self.response_receiver {
            // Try to receive responses without blocking
            while let Ok(response) = rx.try_recv() {
                responses.push(response);
            }
        }
        
        // Handle all received responses
        for response in responses {
            self.handle_grbl_response(response);
        }
        
        // Check for status updates from GRBL - Issue #1
        let mut status_updates = Vec::new();
        if let Some(ref mut rx) = self.status_receiver {
            // Try to receive status updates without blocking
            while let Ok(status) = rx.try_recv() {
                status_updates.push(status);
            }
        }
        
        // Handle all received status updates
        for status in status_updates {
            self.handle_grbl_status_update(status);
        }
        
        // Debug: Log that update is being called
        static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);
        let count = FRAME_COUNT.fetch_add(1, Ordering::Relaxed);
        if count % 60 == 0 {  // Log every 60 frames (~1 second)
            tracing::debug!("Update called: frame {}", count);
        }
        
        // Handle keyboard shortcuts
        ctx.input(|i| {
            // Ctrl+F to open find dialog
            if i.modifiers.command && i.key_pressed(egui::Key::F) {
                self.gcode_editor.toggle_find_replace();
            }
            // Ctrl+O to open file
            if i.modifiers.command && i.key_pressed(egui::Key::O) {
                self.open_file();
            }
            // Ctrl+S to save file
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                self.save_file();
            }
            // Ctrl+, to open settings (common shortcut)
            if i.modifiers.command && i.key_pressed(egui::Key::Comma) {
                self.show_settings_dialog = true;
                self.temp_settings = Some(self.settings.clone());
            }
        });
        
        // Top panel with menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("📂 Open G-Code...").clicked() {
                        tracing::info!("Open button clicked!");  // Debug
                        self.open_file();
                        ui.close_menu();
                    }
                    if ui.button("💾 Save").clicked() {
                        self.save_file();
                        ui.close_menu();
                    }
                    if ui.button("💾 Save As...").clicked() {
                        self.save_file_as();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🚪 Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("Connection", |ui| {
                    if ui.button("🔌 Connect").clicked() {
                        self.connect_to_grbl(ctx);
                        ui.close_menu();
                    }
                    if ui.button("⏸ Disconnect").clicked() {
                        self.disconnect_from_grbl();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🔄 Refresh Ports").clicked() {
                        self.refresh_ports();
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Edit", |ui| {
                    if ui.button("🔍 Find... (Ctrl+F)").clicked() {
                        self.gcode_editor.toggle_find_replace();
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("🎥 Reset Camera").clicked() {
                        if let Some(ref mut renderer) = self.renderer {
                            renderer.reset_camera();
                            self.status_message = "Camera reset".to_string();
                            self.console.info("Camera reset to default view".to_string());
                        }
                        ui.close_menu();
                    }
                    if ui.button("🔍 Zoom to Fit").clicked() {
                        if let Some(ref mut renderer) = self.renderer {
                            renderer.zoom_to_fit();
                            self.status_message = "Zoomed to fit".to_string();
                            self.console.info("Camera zoomed to fit toolpath".to_string());
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.checkbox(&mut self.show_console, "📟 Show Console").clicked() {
                        ui.close_menu();
                    }
                    if ui.checkbox(&mut self.show_user_commands, "🔧 Show User Commands").clicked() {
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Tools", |ui| {
                    if ui.button("⚙ Settings... (Ctrl+,)").clicked() {
                        self.show_settings_dialog = true;
                        self.temp_settings = Some(self.settings.clone());
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("📜 Script Editor...").clicked() {
                        self.show_script_editor = true;
                        self.editing_script = Some(UserScript::new("New Script".to_string(), "// Your script here\n".to_string()));
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("ℹ About").clicked() {
                        self.status_message = format!("rCandle v{}", crate::VERSION);
                        ui.close_menu();
                    }
                    if ui.button("📖 Documentation").clicked() {
                        self.status_message = "Opening documentation... (TODO)".to_string();
                        ui.close_menu();
                    }
                });
            });
        });

        // Bottom status bar
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label(&self.status_message);
                ui.separator();
                
                // Display current file
                if let Some(path) = &self.current_file {
                    ui.label(format!("📄 {}", path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown")));
                    ui.separator();
                }
                
                ui.label(format!("Units: {}", 
                    if self.settings.general.units_metric { "mm" } else { "inch" }));
                
                // Connection indicator
                ui.separator();
                let connected = self.app_state.is_connected();
                let color = if connected { 
                    egui::Color32::GREEN 
                } else { 
                    egui::Color32::RED 
                };
                ui.colored_label(color, if connected { "🟢 Connected" } else { "🔴 Disconnected" });
            });
        });

        // Left panel - controls
        egui::SidePanel::left("left_panel")
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Control Panel");
                ui.separator();
                
                // Connection section
                ui.group(|ui| {
                    ui.label("Connection");
                    
                    // Port selection
                    egui::ComboBox::from_label("Port")
                        .selected_text(&self.selected_port)
                        .show_ui(ui, |ui| {
                            for port in &self.available_ports {
                                ui.selectable_value(&mut self.selected_port, port.clone(), port);
                            }
                        });
                    
                    ui.horizontal(|ui| {
                        let is_connected = self.app_state.is_connected();
                        
                        if !is_connected {
                            if ui.button("🔌 Connect").clicked() {
                                tracing::info!("Connect button clicked");
                                self.connect_to_grbl(ctx);
                            }
                        } else {
                            if ui.button("⏹ Disconnect").clicked() {
                                tracing::info!("Disconnect button clicked");
                                self.disconnect_from_grbl();
                            }
                        }
                        
                        if ui.button("🔄").clicked() {
                            self.refresh_ports();
                        }
                    });
                    
                    // Connection status indicator
                    ui.horizontal(|ui| {
                        let (status_text, status_color) = if self.app_state.is_connected() {
                            ("● Connected", egui::Color32::GREEN)
                        } else {
                            ("○ Disconnected", egui::Color32::GRAY)
                        };
                        ui.colored_label(status_color, status_text);
                    });
                });
                
                ui.add_space(10.0);
                
                // Machine state section - Enhanced real-time display
                ui.group(|ui| {
                    ui.label("Machine State");
                    
                    // Extract data from machine_state before UI rendering
                    let (status, machine_pos_x, machine_pos_y, machine_pos_z, 
                         feed_rate, spindle_speed, feed_override, rapid_override, spindle_override) = {
                        let machine_state = self.app_state.machine.read();
                        (
                            machine_state.status.clone(),
                            machine_state.machine_position.x,
                            machine_state.machine_position.y,
                            machine_state.machine_position.z,
                            machine_state.feed_rate,
                            machine_state.spindle_speed,
                            machine_state.feed_override,
                            machine_state.rapid_override,
                            machine_state.spindle_override,
                        )
                    };
                    
                    // Status with color coding
                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        let status_color = match status {
                            crate::state::MachineStatus::Idle => egui::Color32::GREEN,
                            crate::state::MachineStatus::Run => egui::Color32::LIGHT_BLUE,
                            crate::state::MachineStatus::Hold => egui::Color32::YELLOW,
                            crate::state::MachineStatus::Alarm => egui::Color32::RED,
                            _ => egui::Color32::GRAY,
                        };
                        ui.colored_label(status_color, format!("{:?}", status));
                    });
                    
                    ui.separator();
                    
                    // Machine position
                    ui.label("Machine Position:");
                    ui.label(format!("  X: {:.3}", machine_pos_x));
                    ui.label(format!("  Y: {:.3}", machine_pos_y));
                    ui.label(format!("  Z: {:.3}", machine_pos_z));
                    
                    ui.add_space(3.0);
                    
                    // Feed and spindle display
                    if feed_rate > 0.0 {
                        ui.label(format!("Feed: {:.0} mm/min", feed_rate));
                    }
                    if spindle_speed > 0.0 {
                        ui.label(format!("Spindle: {:.0} RPM", spindle_speed));
                    }
                    
                    ui.add_space(3.0);
                    
                    // Override values
                    ui.label("Overrides:");
                    ui.label(format!("  Feed: {:.0}%", feed_override));
                    ui.label(format!("  Rapid: {:.0}%", rapid_override));
                    ui.label(format!("  Spindle: {:.0}%", spindle_override));
                });
                
                ui.add_space(10.0);
                
                // Jog controls - Enhanced with button grid
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Jog Controls");
                        
                        // Machine lock status indicator
                        let machine_status = self.app_state.machine.read().status;
                        let is_alarm = matches!(machine_status, MachineStatus::Alarm);
                        
                        if is_alarm {
                            ui.add_space(10.0);
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 100, 100), // Red
                                "🔒 LOCKED"
                            );
                        } else {
                            ui.add_space(10.0);
                            ui.colored_label(
                                egui::Color32::from_rgb(100, 255, 100), // Green
                                "🔓 READY"
                            );
                        }
                        
                        // Show the current machine status
                        ui.add_space(5.0);
                        ui.label(format!("({})", machine_status));
                    });
                    
                    ui.add_space(5.0);
                    
                    // Jog step size selector
                    ui.horizontal(|ui| {
                        ui.label("Step:");
                        if ui.selectable_label(self.jog_step_size == 0.1, "0.1").clicked() {
                            self.jog_step_size = 0.1;
                        }
                        if ui.selectable_label(self.jog_step_size == 1.0, "1").clicked() {
                            self.jog_step_size = 1.0;
                        }
                        if ui.selectable_label(self.jog_step_size == 10.0, "10").clicked() {
                            self.jog_step_size = 10.0;
                        }
                        if ui.selectable_label(self.jog_step_size == 100.0, "100").clicked() {
                            self.jog_step_size = 100.0;
                        }
                    });
                    
                    ui.add_space(5.0);
                    
                    // XY Jog grid
                    ui.horizontal(|ui| {
                        ui.add_space(35.0); // Indent for alignment
                        if ui.button("↑ Y+").clicked() {
                            self.send_jog_command(0.0, self.jog_step_size, 0.0);
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("← X-").clicked() {
                            self.send_jog_command(-self.jog_step_size, 0.0, 0.0);
                        }
                        if ui.button("🏠").clicked() {
                            self.send_home_command();
                        }
                        if ui.button("X+ →").clicked() {
                            self.send_jog_command(self.jog_step_size, 0.0, 0.0);
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add_space(35.0); // Indent for alignment
                        if ui.button("🔓 Unlock").clicked() {
                            self.send_unlock_command();
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add_space(35.0); // Indent for alignment
                        if ui.button("↓ Y-").clicked() {
                            self.send_jog_command(0.0, -self.jog_step_size, 0.0);
                        }
                    });
                    
                    ui.add_space(5.0);
                    
                    // Z Jog controls
                    ui.horizontal(|ui| {
                        ui.label("Z:");
                        if ui.button("↑ Z+").clicked() {
                            self.send_jog_command(0.0, 0.0, self.jog_step_size);
                        }
                        if ui.button("Z- ↓").clicked() {
                            self.send_jog_command(0.0, 0.0, -self.jog_step_size);
                        }
                    });
                    
                    ui.add_space(5.0);
                    
                    // Zero buttons
                    ui.horizontal(|ui| {
                        if ui.button("Zero X").clicked() {
                            self.send_zero_axis('X');
                        }
                        if ui.button("Zero Y").clicked() {
                            self.send_zero_axis('Y');
                        }
                        if ui.button("Zero Z").clicked() {
                            self.send_zero_axis('Z');
                        }
                    });
                    
                    if ui.button("Zero All").clicked() {
                        self.send_zero_all();
                    }
                });
                
                ui.add_space(10.0);
                
                // Work coordinate system display
                ui.group(|ui| {
                    ui.label("Work Coordinates");
                    
                    // Extract data from machine_state before closures
                    let (coord_system, work_pos_x, work_pos_y, work_pos_z) = {
                        let machine_state = self.app_state.machine.read();
                        (
                            machine_state.coordinate_system.clone(),
                            machine_state.work_position.x,
                            machine_state.work_position.y,
                            machine_state.work_position.z,
                        )
                    };
                    
                    // Display active coordinate system
                    ui.label(format!("System: {:?}", coord_system));
                    
                    // Display work position (with work offsets applied)
                    ui.label(format!("X: {:.3}", work_pos_x));
                    ui.label(format!("Y: {:.3}", work_pos_y));
                    ui.label(format!("Z: {:.3}", work_pos_z));
                    
                    ui.add_space(5.0);
                    
                    // Quick WCS buttons
                    ui.horizontal(|ui| {
                        for i in 54..=59 {
                            if ui.button(format!("G{}", i)).clicked() {
                                self.send_wcs_command(i);
                            }
                        }
                    });
                });
                
                ui.add_space(10.0);
                
                // Spindle controls with slider
                ui.group(|ui| {
                    ui.label("Spindle");
                    
                    // Spindle speed slider
                    ui.horizontal(|ui| {
                        ui.label("Speed:");
                        ui.add(egui::Slider::new(&mut self.spindle_speed, 0.0..=24000.0)
                            .suffix(" RPM")
                            .clamp_to_range(true));
                    });
                    
                    ui.label(format!("{:.0} RPM", self.spindle_speed));
                    
                    ui.add_space(5.0);
                    
                    // Spindle override
                    ui.horizontal(|ui| {
                        ui.label("Override:");
                        if ui.add(egui::Slider::new(&mut self.spindle_override, 0.0..=200.0)
                            .suffix("%")
                            .clamp_to_range(true)).changed() {
                            self.send_spindle_override(self.spindle_override);
                        }
                    });
                    
                    ui.add_space(5.0);
                    
                    // Spindle control buttons
                    ui.horizontal(|ui| {
                        if ui.button("🗘 CW").clicked() {
                            self.send_spindle_command(true, false);
                        }
                        if ui.button("🗙 CCW").clicked() {
                            self.send_spindle_command(false, true);
                        }
                        if ui.button("⏹ Off").clicked() {
                            self.send_spindle_command(false, false);
                        }
                    });
                });
                
                ui.add_space(10.0);
                
                // Feed rate override
                ui.group(|ui| {
                    ui.label("Feed Rate Override");
                    
                    ui.horizontal(|ui| {
                        if ui.add(egui::Slider::new(&mut self.feed_override, 0.0..=200.0)
                            .suffix("%")
                            .clamp_to_range(true)).changed() {
                            self.send_feed_override(self.feed_override);
                        }
                    });
                    
                    // Quick preset buttons
                    ui.horizontal(|ui| {
                        if ui.button("50%").clicked() {
                            self.feed_override = 50.0;
                            self.send_feed_override(self.feed_override);
                        }
                        if ui.button("100%").clicked() {
                            self.feed_override = 100.0;
                            self.send_feed_override(self.feed_override);
                        }
                        if ui.button("150%").clicked() {
                            self.feed_override = 150.0;
                            self.send_feed_override(self.feed_override);
                        }
                    });
                    
                    ui.label(format!("Active: {:.0}%", self.feed_override));
                });
                
                ui.add_space(10.0);
                
                // Rapid override
                ui.group(|ui| {
                    ui.label("Rapid Override");
                    
                    ui.horizontal(|ui| {
                        if ui.add(egui::Slider::new(&mut self.rapid_override, 25.0..=100.0)
                            .suffix("%")
                            .clamp_to_range(true)).changed() {
                            self.send_rapid_override(self.rapid_override);
                        }
                    });
                    
                    // Quick preset buttons
                    ui.horizontal(|ui| {
                        if ui.button("25%").clicked() {
                            self.rapid_override = 25.0;
                            self.send_rapid_override(self.rapid_override);
                        }
                        if ui.button("50%").clicked() {
                            self.rapid_override = 50.0;
                            self.send_rapid_override(self.rapid_override);
                        }
                        if ui.button("100%").clicked() {
                            self.rapid_override = 100.0;
                            self.send_rapid_override(self.rapid_override);
                        }
                    });
                    
                    ui.label(format!("Active: {:.0}%", self.rapid_override));
                });
                
                ui.add_space(10.0);
                
                // Program execution controls
                ui.group(|ui| {
                    ui.heading("Program Execution");
                    
                    // Status indicator with color
                    let program_state = self.app_state.program.read();
                    let status_text = match program_state.state {
                        ExecutionState::NotLoaded => "No Program",
                        ExecutionState::Loaded => "Ready",
                        ExecutionState::Running => "Running",
                        ExecutionState::Paused => "Paused",
                        ExecutionState::Completed => "Complete",
                        ExecutionState::Error => "Error",
                    };
                    
                    let status_color = match program_state.state {
                        ExecutionState::NotLoaded => egui::Color32::DARK_GRAY,
                        ExecutionState::Loaded => egui::Color32::GRAY,
                        ExecutionState::Running => egui::Color32::LIGHT_BLUE,
                        ExecutionState::Paused => egui::Color32::YELLOW,
                        ExecutionState::Completed => egui::Color32::LIGHT_GREEN,
                        ExecutionState::Error => egui::Color32::RED,
                    };
                    
                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        ui.colored_label(status_color, status_text);
                    });
                    
                    drop(program_state);
                    
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                    
                    // Main control buttons in a grid
                    ui.horizontal(|ui| {
                        if ui.button("▶ Run").clicked() {
                            self.start_program();
                        }
                        if ui.button("⏸ Pause").clicked() {
                            self.pause_program();
                        }
                        if ui.button("⏹ Stop").clicked() {
                            self.stop_program();
                        }
                        if ui.button("🔄 Reset").clicked() {
                            self.reset_program();
                        }
                    });
                    
                    ui.add_space(5.0);
                    
                    // Progress bar
                    let program_state = self.app_state.program.read();
                    let progress_percent = if program_state.total_lines > 0 {
                        (program_state.current_line as f64 / program_state.total_lines as f64) * 100.0
                    } else {
                        0.0
                    };
                    let progress = progress_percent / 100.0;
                    let progress_text = format!("{:.1}%", progress_percent);
                    drop(program_state);
                    
                    ui.horizontal(|ui| {
                        ui.label("Progress:");
                        ui.add(egui::ProgressBar::new(progress as f32).text(progress_text));
                    });
                    
                    ui.add_space(5.0);
                    
                    // Line tracking
                    let program_state = self.app_state.program.read();
                    let total_lines = program_state.total_lines;
                    drop(program_state);
                    
                    ui.horizontal(|ui| {
                        ui.label("Line:");
                        ui.label(format!("{} / {}", self.current_line + 1, total_lines));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Completed:");
                        ui.label(format!("{}", self.current_line));
                    });
                    
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                    
                    // Time tracking
                    let (elapsed_text, remaining_text) = self.calculate_time_estimates();
                    
                    ui.horizontal(|ui| {
                        ui.label("Elapsed:");
                        ui.label(elapsed_text);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Remaining:");
                        ui.label(remaining_text);
                    });
                    
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                    
                    // Step mode controls
                    ui.checkbox(&mut self.step_mode, "Step Mode");
                    
                    if self.step_mode {
                        if ui.button("⏭ Single Step").clicked() {
                            self.execute_single_step();
                        }
                    }
                    
                    ui.add_space(5.0);
                    
                    // Execution speed override
                    ui.horizontal(|ui| {
                        ui.label("Speed:");
                        ui.add(egui::Slider::new(&mut self.execution_speed, 0.0..=200.0)
                            .suffix("%")
                            .clamp_to_range(true));
                    });
                    
                    ui.label(format!("Active: {:.0}%", self.execution_speed));
                });
                
                ui.add_space(10.0);
                
                // View Presets - Phase 8
                ui.group(|ui| {
                    ui.label("View Presets");
                    
                    // Top row of view buttons
                    ui.horizontal(|ui| {
                        if ui.button("⬆ Top").clicked() {
                            self.apply_view_preset(ViewPreset::Top);
                        }
                        if ui.button("⬅ Front").clicked() {
                            self.apply_view_preset(ViewPreset::Front);
                        }
                        if ui.button("➡ Right").clicked() {
                            self.apply_view_preset(ViewPreset::Right);
                        }
                    });
                    
                    // Bottom row of view buttons
                    ui.horizontal(|ui| {
                        if ui.button("⬇ Bottom").clicked() {
                            self.apply_view_preset(ViewPreset::Bottom);
                        }
                        if ui.button("◀ Back").clicked() {
                            self.apply_view_preset(ViewPreset::Back);
                        }
                        if ui.button("◄ Left").clicked() {
                            self.apply_view_preset(ViewPreset::Left);
                        }
                    });
                    
                    // Isometric default view
                    if ui.button("🔲 Isometric").clicked() {
                        self.apply_view_preset(ViewPreset::Isometric);
                    }
                });
                
                ui.add_space(10.0);
                
                // User Commands Panel - Phase 8
                if self.show_user_commands {
                    // Store clicked command outside of borrowing scope
                    let mut clicked_command: Option<String> = None;
                    
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("User Commands");
                            if ui.button("➕").clicked() {
                                self.show_script_editor = true;
                                self.editing_script = Some(UserScript::new("New Command".to_string(), String::new()));
                            }
                        });
                        
                        ui.separator();
                        
                        // Display user commands by category
                        let categories = self.user_command_library.categories();
                        for category in categories {
                            ui.label(category.clone());
                            
                            let commands = self.user_command_library.commands_by_category(&category);
                            for command in commands {
                                if ui.button(&command.name).clicked() {
                                    clicked_command = Some(command.name.clone());
                                }
                            }
                            
                            ui.add_space(3.0);
                        }
                    });
                    
                    // Execute clicked command after UI closure
                    if let Some(cmd_name) = clicked_command {
                        self.execute_user_command(&cmd_name);
                    }
                }

            });

        // Right panel - G-Code editor/viewer
        egui::SidePanel::right("right_panel")
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.heading("G-Code");
                ui.separator();
                
                // Use the custom GCodeEditor widget
                self.gcode_editor.show(ui, &mut self.gcode_content);
            });

        // Console panel (bottom, before central panel)
        if self.show_console {
            egui::TopBottomPanel::bottom("console_panel")
                .default_height(200.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.heading("Console");
                    ui.separator();
                    
                    // Show console widget and handle command submission
                    if let Some(command) = self.console.show(ui) {
                        // Handle command submission
                        self.handle_console_command(&command);
                    }
                });
        }

        // Central panel - 3D viewport
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Toolpath Viewer");
            
            let available_size = ui.available_size();
            // Use hover sense instead of click_and_drag to avoid consuming events
            let (rect, _response) = ui.allocate_exact_size(
                available_size,
                egui::Sense::hover()
            );
            
            // Draw background
            ui.painter().rect_filled(
                rect,
                0.0,
                egui::Color32::from_rgb(25, 25, 35)
            );
            
            // Draw toolpath if we have segments
            if !self.segments.is_empty() {
                self.draw_toolpath_2d(ui, rect);
            } else {
                // Show placeholder text
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Load a G-Code file to view toolpath\n(File > Open G-Code...)",
                    egui::FontId::proportional(18.0),
                    egui::Color32::from_rgb(150, 150, 150),
                );
            }
            
            // Show instructions in corner
            if !self.segments.is_empty() {
                let instructions = format!(
                    "Segments: {} | Use View menu for camera controls",
                    self.segments.len()
                );
                ui.painter().text(
                    rect.left_top() + egui::vec2(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    instructions,
                    egui::FontId::monospace(12.0),
                    egui::Color32::from_rgb(180, 180, 180),
                );
            }
        });
        
        // Settings dialog
        if self.show_settings_dialog {
            self.show_settings_window(ctx);
        }
        
        // Show script editor dialog - Phase 8
        if self.show_script_editor {
            self.show_script_editor_window(ctx);
        }
        
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save settings to default location
        if let Err(e) = self.settings.save_default() {
            tracing::error!("Failed to save settings: {}", e);
        }
    }
}

/// Format a duration in HH:MM:SS format
fn format_duration(duration: std::time::Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
