//! State updater - processes GRBL responses and updates state
//!
//! This module provides the logic to update application state based on GRBL responses.

use super::{
    AppState, CoordinateSystem, ExecutionState, MachineStatus, Position,
};
use crate::grbl::{GrblResponse, GrblStatus};
use crate::state::events::{StateEvent, StateEventBroadcaster};

/// State updater that processes GRBL responses
pub struct StateUpdater {
    /// Application state
    app_state: AppState,
    /// Event broadcaster
    event_broadcaster: StateEventBroadcaster,
}

impl StateUpdater {
    /// Create a new state updater
    pub fn new(app_state: AppState, event_broadcaster: StateEventBroadcaster) -> Self {
        StateUpdater {
            app_state,
            event_broadcaster,
        }
    }

    /// Process a GRBL response and update state accordingly
    pub fn process_response(&self, response: &GrblResponse) {
        match response {
            GrblResponse::Status(status) => {
                self.update_from_status_report(status);
            }
            GrblResponse::Ok => {
                // Command completed successfully
                self.increment_program_progress();
            }
            GrblResponse::Error(code) => {
                let msg = format!("GRBL Error: {}", code);
                self.handle_error(msg);
            }
            GrblResponse::Alarm(code) => {
                let msg = format!("GRBL Alarm: {}", code);
                self.handle_error(msg);
            }
            GrblResponse::Setting { number, value } => {
                tracing::debug!("Received setting: ${}={}", number, value);
                // Settings could be stored in app state if needed
            }
            GrblResponse::Feedback(msg) => {
                tracing::debug!("Feedback: {}", msg);
            }
            GrblResponse::Message(msg) => {
                tracing::info!("Message: {}", msg);
            }
            GrblResponse::Welcome { version } => {
                tracing::info!("GRBL {} startup message received", version);
                self.app_state.set_connected(true);
                self.event_broadcaster.send(StateEvent::ConnectionChanged {
                    connected: true,
                });
            }
        }
    }

    /// Update state from a status report
    fn update_from_status_report(&self, status: &GrblStatus) {
        let mut machine = self.app_state.machine.write();
        let old_status = machine.status;

        // Update machine status
        let new_status = convert_grbl_machine_state(&status.state);
        if old_status != new_status {
            machine.status = new_status;
            self.event_broadcaster
                .send(StateEvent::MachineStatusChanged {
                    old: old_status,
                    new: new_status,
                });
        }

        // Update positions if available
        if let Some(mpos) = &status.mpos {
            let new_pos = Position::new(mpos.x, mpos.y, mpos.z);
            machine.update_machine_position(new_pos);
            self.event_broadcaster
                .send(StateEvent::MachinePositionChanged {
                    machine_pos: machine.machine_position,
                    work_pos: machine.work_position,
                });
        }

        // Update work position if available (takes precedence over calculated)
        if let Some(wpos) = &status.wpos {
            machine.work_position = Position::new(wpos.x, wpos.y, wpos.z);
        }

        // Update feed rate
        if let Some(feed) = status.feed_rate {
            if (machine.feed_rate - feed).abs() > 0.01 {
                machine.feed_rate = feed;
                self.event_broadcaster
                    .send(StateEvent::FeedRateChanged { feed_rate: feed });
            }
        }

        // Update spindle speed
        if let Some(speed) = status.spindle_speed {
            if (machine.spindle_speed - speed).abs() > 0.01 {
                machine.spindle_speed = speed;
                machine.spindle_enabled = speed > 0.0;
                self.event_broadcaster
                    .send(StateEvent::SpindleStateChanged {
                        enabled: machine.spindle_enabled,
                        speed,
                    });
            }
        }

        // Update overrides
        let mut overrides_changed = false;
        if let Some(feed_override) = status.feed_override {
            let feed_override_f64 = feed_override as f64;
            if (machine.feed_override - feed_override_f64).abs() > 0.01 {
                machine.feed_override = feed_override_f64;
                overrides_changed = true;
            }
        }
        if let Some(rapid_override) = status.rapid_override {
            let rapid_override_f64 = rapid_override as f64;
            if (machine.rapid_override - rapid_override_f64).abs() > 0.01 {
                machine.rapid_override = rapid_override_f64;
                overrides_changed = true;
            }
        }
        if let Some(spindle_override) = status.spindle_override {
            let spindle_override_f64 = spindle_override as f64;
            if (machine.spindle_override - spindle_override_f64).abs() > 0.01 {
                machine.spindle_override = spindle_override_f64;
                overrides_changed = true;
            }
        }

        if overrides_changed {
            self.event_broadcaster
                .send(StateEvent::OverridesChanged {
                    feed: machine.feed_override,
                    rapid: machine.rapid_override,
                    spindle: machine.spindle_override,
                });
        }

        // Update buffer state
        if let Some((planner, _rx)) = status.buffer {
            machine.buffer_state = planner as u32;
        }
    }

    /// Handle an error response
    fn handle_error(&self, message: String) {
        let mut machine = self.app_state.machine.write();
        machine.last_error = Some(message.clone());

        let mut program = self.app_state.program.write();
        if program.state == ExecutionState::Running {
            program.error();
            self.event_broadcaster
                .send(StateEvent::ProgramStateChanged {
                    old: ExecutionState::Running,
                    new: ExecutionState::Error,
                });
        }

        self.event_broadcaster
            .send(StateEvent::ErrorOccurred { message });
    }

    /// Increment program progress (called on successful command completion)
    fn increment_program_progress(&self) {
        let mut program = self.app_state.program.write();
        if program.state == ExecutionState::Running {
            program.increment_completed();
            self.event_broadcaster
                .send(StateEvent::ProgramProgressChanged {
                    current_line: program.current_line,
                    total_lines: program.total_lines,
                    progress: program.progress(),
                });
        }
    }

    /// Update coordinate system
    pub fn set_coordinate_system(&self, system: CoordinateSystem) {
        let mut machine = self.app_state.machine.write();
        let old = machine.coordinate_system;
        if old != system {
            machine.coordinate_system = system;
            self.event_broadcaster
                .send(StateEvent::CoordinateSystemChanged {
                    old,
                    new: system,
                });
        }
    }

    /// Update work offset for a coordinate system
    pub fn set_work_offset(&self, system: CoordinateSystem, offset: Position) {
        let mut machine = self.app_state.machine.write();
        machine.set_work_offset(system, offset);
        self.event_broadcaster
            .send(StateEvent::WorkOffsetChanged { system, offset });
    }

    /// Start program execution
    pub fn start_program(&self) {
        let mut program = self.app_state.program.write();
        let old_state = program.state;
        program.start();
        self.event_broadcaster
            .send(StateEvent::ProgramStateChanged {
                old: old_state,
                new: program.state,
            });
    }

    /// Pause program execution
    pub fn pause_program(&self) {
        let mut program = self.app_state.program.write();
        let old_state = program.state;
        program.pause();
        self.event_broadcaster
            .send(StateEvent::ProgramStateChanged {
                old: old_state,
                new: program.state,
            });
    }

    /// Stop program execution
    pub fn stop_program(&self) {
        let mut program = self.app_state.program.write();
        let old_state = program.state;
        program.stop();
        self.event_broadcaster
            .send(StateEvent::ProgramStateChanged {
                old: old_state,
                new: program.state,
            });
    }

    /// Complete program execution
    pub fn complete_program(&self) {
        let mut program = self.app_state.program.write();
        let old_state = program.state;
        program.complete();
        self.event_broadcaster
            .send(StateEvent::ProgramStateChanged {
                old: old_state,
                new: program.state,
            });
    }

    /// Load a program
    pub fn load_program(&self, file_path: String, total_lines: usize) {
        let mut program = self.app_state.program.write();
        let old_state = program.state;
        program.load(file_path, total_lines);
        self.event_broadcaster
            .send(StateEvent::ProgramStateChanged {
                old: old_state,
                new: program.state,
            });
    }
}

/// Convert GRBL machine state to application machine status
fn convert_grbl_machine_state(grbl_state: &crate::grbl::MachineState) -> MachineStatus {
    match grbl_state {
        crate::grbl::MachineState::Idle => MachineStatus::Idle,
        crate::grbl::MachineState::Run => MachineStatus::Run,
        crate::grbl::MachineState::Hold => MachineStatus::Hold,
        crate::grbl::MachineState::Jog => MachineStatus::Jog,
        crate::grbl::MachineState::Alarm => MachineStatus::Alarm,
        crate::grbl::MachineState::Door => MachineStatus::Door,
        crate::grbl::MachineState::Check => MachineStatus::Check,
        crate::grbl::MachineState::Home => MachineStatus::Home,
        crate::grbl::MachineState::Sleep => MachineStatus::Sleep,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_grbl_machine_state() {
        assert_eq!(
            convert_grbl_machine_state(&crate::grbl::MachineState::Idle),
            MachineStatus::Idle
        );
        assert_eq!(
            convert_grbl_machine_state(&crate::grbl::MachineState::Run),
            MachineStatus::Run
        );
        assert_eq!(
            convert_grbl_machine_state(&crate::grbl::MachineState::Hold),
            MachineStatus::Hold
        );
        assert_eq!(
            convert_grbl_machine_state(&crate::grbl::MachineState::Jog),
            MachineStatus::Jog
        );
        assert_eq!(
            convert_grbl_machine_state(&crate::grbl::MachineState::Alarm),
            MachineStatus::Alarm
        );
    }

    #[tokio::test]
    async fn test_state_updater_error_handling() {
        let app_state = AppState::new();
        let broadcaster = StateEventBroadcaster::new(10);
        let updater = StateUpdater::new(app_state.clone(), broadcaster.clone());

        updater.handle_error("Test error".to_string());

        let machine = app_state.machine.read();
        assert_eq!(machine.last_error, Some("Test error".to_string()));
    }

    #[tokio::test]
    async fn test_state_updater_coordinate_system() {
        let app_state = AppState::new();
        let broadcaster = StateEventBroadcaster::new(10);
        let updater = StateUpdater::new(app_state.clone(), broadcaster.clone());
        let mut receiver = broadcaster.subscribe();

        updater.set_coordinate_system(CoordinateSystem::G55);

        let machine = app_state.machine.read();
        assert_eq!(machine.coordinate_system, CoordinateSystem::G55);

        // Check event was broadcast
        let event = receiver.recv().await.unwrap();
        match event {
            StateEvent::CoordinateSystemChanged { old, new } => {
                assert_eq!(old, CoordinateSystem::G54);
                assert_eq!(new, CoordinateSystem::G55);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[tokio::test]
    async fn test_state_updater_program_lifecycle() {
        let app_state = AppState::new();
        let broadcaster = StateEventBroadcaster::new(10);
        let updater = StateUpdater::new(app_state.clone(), broadcaster.clone());

        updater.load_program("test.gcode".to_string(), 100);
        {
            let program = app_state.program.read();
            assert_eq!(program.state, ExecutionState::Loaded);
        }

        updater.start_program();
        {
            let program = app_state.program.read();
            assert_eq!(program.state, ExecutionState::Running);
        }

        updater.pause_program();
        {
            let program = app_state.program.read();
            assert_eq!(program.state, ExecutionState::Paused);
        }

        updater.start_program();
        {
            let program = app_state.program.read();
            assert_eq!(program.state, ExecutionState::Running);
        }

        updater.complete_program();
        {
            let program = app_state.program.read();
            assert_eq!(program.state, ExecutionState::Completed);
        }
    }
}
