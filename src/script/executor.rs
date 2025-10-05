//! Script executor
//!
//! Manages script execution and lifecycle.

use super::{ScriptContext, ScriptApi, ScriptCommand};
use crate::utils::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
use rhai::Scope;

/// Script executor that manages script lifecycle
pub struct ScriptExecutor {
    context: ScriptContext,
    command_rx: mpsc::UnboundedReceiver<ScriptCommand>,
}

impl ScriptExecutor {
    /// Create a new script executor
    pub fn new(api: Arc<ScriptApi>, command_rx: mpsc::UnboundedReceiver<ScriptCommand>) -> Self {
        let context = ScriptContext::new(api);
        Self { context, command_rx }
    }
    
    /// Execute a script
    pub fn execute(&mut self, script: &str) -> Result<String> {
        match self.context.execute(script) {
            Ok(result) => Ok(format!("{}", result)),
            Err(e) => Err(e),
        }
    }
    
    /// Execute a script with variables
    pub fn execute_with_vars(&mut self, script: &str, vars: Vec<(String, rhai::Dynamic)>) -> Result<String> {
        let mut scope = Scope::new();
        
        // Add variables to scope
        for (name, value) in vars {
            scope.push(name, value);
        }
        
        match self.context.execute_with_scope(script, &mut scope) {
            Ok(result) => Ok(format!("{}", result)),
            Err(e) => Err(e),
        }
    }
    
    /// Get the command receiver (for processing commands from scripts)
    pub fn command_receiver(&mut self) -> &mut mpsc::UnboundedReceiver<ScriptCommand> {
        &mut self.command_rx
    }
}

/// User-defined script
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserScript {
    /// Script name
    pub name: String,
    
    /// Script description
    pub description: String,
    
    /// Script code
    pub code: String,
    
    /// Whether to show in toolbar
    pub show_in_toolbar: bool,
    
    /// Keyboard shortcut (optional)
    pub shortcut: Option<String>,
}

impl UserScript {
    /// Create a new user script
    pub fn new(name: String, code: String) -> Self {
        Self {
            name,
            description: String::new(),
            code,
            show_in_toolbar: false,
            shortcut: None,
        }
    }
}

/// Script library managing user scripts
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ScriptLibrary {
    /// User scripts
    pub scripts: Vec<UserScript>,
}

impl ScriptLibrary {
    /// Create a new script library
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a script
    pub fn add_script(&mut self, script: UserScript) {
        self.scripts.push(script);
    }
    
    /// Remove a script by name
    pub fn remove_script(&mut self, name: &str) -> bool {
        if let Some(index) = self.scripts.iter().position(|s| s.name == name) {
            self.scripts.remove(index);
            true
        } else {
            false
        }
    }
    
    /// Get a script by name
    pub fn get_script(&self, name: &str) -> Option<&UserScript> {
        self.scripts.iter().find(|s| s.name == name)
    }
    
    /// Get a mutable script by name
    pub fn get_script_mut(&mut self, name: &str) -> Option<&mut UserScript> {
        self.scripts.iter_mut().find(|s| s.name == name)
    }
    
    /// List all script names
    pub fn list_scripts(&self) -> Vec<String> {
        self.scripts.iter().map(|s| s.name.clone()).collect()
    }
    
    /// Get scripts to show in toolbar
    pub fn toolbar_scripts(&self) -> Vec<&UserScript> {
        self.scripts.iter().filter(|s| s.show_in_toolbar).collect()
    }
}
