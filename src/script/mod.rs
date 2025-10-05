//! Script module
//!
//! Provides scripting support using the Rhai scripting engine.
//! Allows users to automate tasks and extend application functionality.

use rhai::{Engine, Scope, Dynamic};
use std::sync::Arc;
use crate::utils::{Error, Result};

mod api;
mod executor;
mod user_commands;

pub use api::{ScriptApi, ScriptCommand};
pub use executor::{ScriptExecutor, UserScript, ScriptLibrary};
pub use user_commands::{UserCommand, UserCommandLibrary};

/// Script context containing application state and API access
pub struct ScriptContext {
    engine: Engine,
    api: Arc<ScriptApi>,
}

impl ScriptContext {
    /// Create a new script context
    pub fn new(api: Arc<ScriptApi>) -> Self {
        let mut engine = Engine::new();
        
        // Configure engine
        engine.set_max_expr_depths(64, 32);
        
        // Register API functions
        Self::register_api(&mut engine, api.clone());
        
        Self { engine, api }
    }
    
    /// Register API functions with the engine
    fn register_api(engine: &mut Engine, api: Arc<ScriptApi>) {
        // Machine control functions
        let api_clone = api.clone();
        engine.register_fn("send_command", move |cmd: &str| {
            api_clone.send_command(cmd.to_string())
        });
        
        let api_clone = api.clone();
        engine.register_fn("jog", move |axis: &str, distance: f64| {
            api_clone.jog(axis.to_string(), distance)
        });
        
        let api_clone = api.clone();
        engine.register_fn("home", move || {
            api_clone.home()
        });
        
        let api_clone = api.clone();
        engine.register_fn("zero_axis", move |axis: &str| {
            api_clone.zero_axis(axis.to_string())
        });
        
        // Status query functions
        let api_clone = api.clone();
        engine.register_fn("get_position", move |axis: &str| {
            api_clone.get_position(axis.to_string())
        });
        
        let api_clone = api.clone();
        engine.register_fn("is_connected", move || {
            api_clone.is_connected()
        });
        
        let api_clone = api.clone();
        engine.register_fn("get_state", move || {
            api_clone.get_state()
        });
        
        // Program control
        let api_clone = api.clone();
        engine.register_fn("start_program", move || {
            api_clone.start_program()
        });
        
        let api_clone = api.clone();
        engine.register_fn("pause_program", move || {
            api_clone.pause_program()
        });
        
        let api_clone = api.clone();
        engine.register_fn("stop_program", move || {
            api_clone.stop_program()
        });
        
        // Utility functions
        let api_clone = api.clone();
        engine.register_fn("log", move |msg: &str| {
            api_clone.log(msg.to_string())
        });
        
        let api_clone = api.clone();
        engine.register_fn("sleep", move |ms: i64| {
            api_clone.sleep(ms as u64)
        });
    }
    
    /// Execute a script
    pub fn execute(&mut self, script: &str) -> Result<Dynamic> {
        self.engine.eval(script)
            .map_err(|e| Error::Script(format!("Script error: {}", e)))
    }
    
    /// Execute a script with a custom scope
    pub fn execute_with_scope(&mut self, script: &str, scope: &mut Scope) -> Result<Dynamic> {
        self.engine.eval_with_scope(scope, script)
            .map_err(|e| Error::Script(format!("Script error: {}", e)))
    }
}
