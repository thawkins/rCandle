//! State management module
//!
//! Provides thread-safe state tracking for the machine and application.

use std::sync::{Arc, RwLock};

mod machine;
mod program;
mod app;

pub use machine::{MachineState, MachineStatus, Position, CoordinateSystem};
pub use program::{ProgramState, ExecutionState};
pub use app::AppState;

/// Shared state wrapper for thread-safe access
#[derive(Clone)]
pub struct SharedState<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> SharedState<T> {
    /// Create a new shared state
    pub fn new(value: T) -> Self {
        SharedState {
            inner: Arc::new(RwLock::new(value)),
        }
    }

    /// Read access to the state
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, T> {
        self.inner.read().unwrap()
    }

    /// Write access to the state
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, T> {
        self.inner.write().unwrap()
    }

    /// Update the state with a closure
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut state = self.write();
        f(&mut state);
    }
}

impl<T: Default> Default for SharedState<T> {
    fn default() -> Self {
        SharedState::new(T::default())
    }
}
