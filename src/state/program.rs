//! Program execution state tracking

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Program execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionState {
    /// No program loaded
    NotLoaded,
    /// Program loaded but not started
    Loaded,
    /// Program is running
    Running,
    /// Program is paused
    Paused,
    /// Program completed successfully
    Completed,
    /// Program stopped with error
    Error,
}

impl Default for ExecutionState {
    fn default() -> Self {
        ExecutionState::NotLoaded
    }
}

impl std::fmt::Display for ExecutionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionState::NotLoaded => write!(f, "Not Loaded"),
            ExecutionState::Loaded => write!(f, "Loaded"),
            ExecutionState::Running => write!(f, "Running"),
            ExecutionState::Paused => write!(f, "Paused"),
            ExecutionState::Completed => write!(f, "Completed"),
            ExecutionState::Error => write!(f, "Error"),
        }
    }
}

/// Program state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramState {
    /// Current execution state
    pub state: ExecutionState,
    
    /// File path of loaded program
    pub file_path: Option<String>,
    
    /// Total number of lines in program
    pub total_lines: usize,
    
    /// Current line being executed
    pub current_line: usize,
    
    /// Number of lines sent to GRBL
    pub lines_sent: usize,
    
    /// Number of lines completed
    pub lines_completed: usize,
    
    /// Start time of execution
    #[serde(skip)]
    pub start_time: Option<Instant>,
    
    /// Elapsed time
    #[serde(skip)]
    pub elapsed_time: Duration,
    
    /// Estimated time remaining
    #[serde(skip)]
    pub estimated_remaining: Option<Duration>,
    
    /// Program bounding box (min corner)
    pub bounds_min: Option<[f64; 3]>,
    
    /// Program bounding box (max corner)
    pub bounds_max: Option<[f64; 3]>,
}

impl Default for ProgramState {
    fn default() -> Self {
        ProgramState {
            state: ExecutionState::default(),
            file_path: None,
            total_lines: 0,
            current_line: 0,
            lines_sent: 0,
            lines_completed: 0,
            start_time: None,
            elapsed_time: Duration::ZERO,
            estimated_remaining: None,
            bounds_min: None,
            bounds_max: None,
        }
    }
}

impl ProgramState {
    /// Create a new program state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a program
    pub fn load(&mut self, file_path: String, total_lines: usize) {
        self.state = ExecutionState::Loaded;
        self.file_path = Some(file_path);
        self.total_lines = total_lines;
        self.current_line = 0;
        self.lines_sent = 0;
        self.lines_completed = 0;
        self.start_time = None;
        self.elapsed_time = Duration::ZERO;
        self.estimated_remaining = None;
    }

    /// Start program execution
    pub fn start(&mut self) {
        if self.state == ExecutionState::Loaded || self.state == ExecutionState::Paused {
            self.state = ExecutionState::Running;
            if self.start_time.is_none() {
                self.start_time = Some(Instant::now());
            }
        }
    }

    /// Pause program execution
    pub fn pause(&mut self) {
        if self.state == ExecutionState::Running {
            self.state = ExecutionState::Paused;
            self.update_elapsed_time();
        }
    }

    /// Stop program execution
    pub fn stop(&mut self) {
        self.state = ExecutionState::NotLoaded;
        self.current_line = 0;
        self.lines_sent = 0;
        self.lines_completed = 0;
        self.start_time = None;
        self.elapsed_time = Duration::ZERO;
        self.estimated_remaining = None;
    }

    /// Mark program as completed
    pub fn complete(&mut self) {
        self.state = ExecutionState::Completed;
        self.update_elapsed_time();
        self.estimated_remaining = Some(Duration::ZERO);
    }

    /// Mark program as error
    pub fn error(&mut self) {
        self.state = ExecutionState::Error;
        self.update_elapsed_time();
    }

    /// Reset program to beginning
    pub fn reset(&mut self) {
        if matches!(self.state, ExecutionState::Completed | ExecutionState::Error) {
            self.state = ExecutionState::Loaded;
            self.current_line = 0;
            self.lines_sent = 0;
            self.lines_completed = 0;
            self.start_time = None;
            self.elapsed_time = Duration::ZERO;
            self.estimated_remaining = None;
        }
    }

    /// Update current line
    pub fn set_current_line(&mut self, line: usize) {
        self.current_line = line.min(self.total_lines);
    }

    /// Increment lines sent
    pub fn increment_sent(&mut self) {
        self.lines_sent += 1;
    }

    /// Increment lines completed
    pub fn increment_completed(&mut self) {
        self.lines_completed += 1;
        self.update_progress();
    }

    /// Get progress as percentage (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.total_lines == 0 {
            0.0
        } else {
            (self.lines_completed as f64) / (self.total_lines as f64)
        }
    }

    /// Update elapsed time
    pub fn update_elapsed_time(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed_time = start.elapsed();
        }
    }

    /// Update progress and estimate remaining time
    fn update_progress(&mut self) {
        if self.state == ExecutionState::Running {
            self.update_elapsed_time();
            
            let progress = self.progress();
            if progress > 0.0 && progress < 1.0 {
                let elapsed_secs = self.elapsed_time.as_secs_f64();
                let total_estimated = elapsed_secs / progress;
                let remaining = total_estimated - elapsed_secs;
                self.estimated_remaining = Some(Duration::from_secs_f64(remaining));
            }
        }
    }

    /// Set program bounds
    pub fn set_bounds(&mut self, min: [f64; 3], max: [f64; 3]) {
        self.bounds_min = Some(min);
        self.bounds_max = Some(max);
    }

    /// Get bounds as tuple
    pub fn get_bounds(&self) -> Option<([f64; 3], [f64; 3])> {
        match (self.bounds_min, self.bounds_max) {
            (Some(min), Some(max)) => Some((min, max)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_state_load() {
        let mut state = ProgramState::new();
        state.load("test.gcode".to_string(), 100);
        
        assert_eq!(state.state, ExecutionState::Loaded);
        assert_eq!(state.total_lines, 100);
        assert_eq!(state.file_path, Some("test.gcode".to_string()));
    }

    #[test]
    fn test_program_state_progress() {
        let mut state = ProgramState::new();
        state.load("test.gcode".to_string(), 100);
        
        for _ in 0..50 {
            state.increment_completed();
        }
        
        assert_eq!(state.progress(), 0.5);
    }

    #[test]
    fn test_program_state_lifecycle() {
        let mut state = ProgramState::new();
        assert_eq!(state.state, ExecutionState::NotLoaded);
        
        state.load("test.gcode".to_string(), 100);
        assert_eq!(state.state, ExecutionState::Loaded);
        
        state.start();
        assert_eq!(state.state, ExecutionState::Running);
        
        state.pause();
        assert_eq!(state.state, ExecutionState::Paused);
        
        state.start();
        assert_eq!(state.state, ExecutionState::Running);
        
        state.complete();
        assert_eq!(state.state, ExecutionState::Completed);
    }
}
