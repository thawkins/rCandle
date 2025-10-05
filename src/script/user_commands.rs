//! User command system
//!
//! Allows users to define custom command buttons with GRBL commands.

use serde::{Deserialize, Serialize};

/// A user-defined command button
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCommand {
    /// Command name (displayed on button)
    pub name: String,
    
    /// Command description/tooltip
    pub description: String,
    
    /// GRBL commands to execute (one per line)
    pub commands: Vec<String>,
    
    /// Icon name (optional)
    pub icon: Option<String>,
    
    /// Keyboard shortcut (optional, e.g., "Ctrl+Shift+C")
    pub shortcut: Option<String>,
    
    /// Category/group for organization
    pub category: String,
    
    /// Whether to confirm before execution
    pub confirm: bool,
    
    /// Whether the command requires a connection
    pub requires_connection: bool,
}

impl UserCommand {
    /// Create a new user command
    pub fn new(name: String, commands: Vec<String>) -> Self {
        Self {
            name,
            description: String::new(),
            commands,
            icon: None,
            shortcut: None,
            category: "General".to_string(),
            confirm: false,
            requires_connection: true,
        }
    }
    
    /// Builder: set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
    
    /// Builder: set icon
    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }
    
    /// Builder: set shortcut
    pub fn with_shortcut(mut self, shortcut: String) -> Self {
        self.shortcut = Some(shortcut);
        self
    }
    
    /// Builder: set category
    pub fn with_category(mut self, category: String) -> Self {
        self.category = category;
        self
    }
    
    /// Builder: set confirmation requirement
    pub fn with_confirm(mut self, confirm: bool) -> Self {
        self.confirm = confirm;
        self
    }
    
    /// Builder: set connection requirement
    pub fn requires_connection(mut self, requires: bool) -> Self {
        self.requires_connection = requires;
        self
    }
}

/// User command library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCommandLibrary {
    /// User-defined commands
    pub commands: Vec<UserCommand>,
}

impl UserCommandLibrary {
    /// Create a new user command library
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create with default commands
    pub fn with_defaults() -> Self {
        let mut library = Self::new();
        
        // Add some useful default commands
        library.add_command(UserCommand::new(
            "Safe Z".to_string(),
            vec!["G91".to_string(), "G0 Z5".to_string(), "G90".to_string()],
        )
        .with_description("Raise Z by 5mm in relative mode".to_string())
        .with_category("Safety".to_string()));
        
        library.add_command(UserCommand::new(
            "Spindle On".to_string(),
            vec!["M3 S1000".to_string()],
        )
        .with_description("Start spindle at 1000 RPM".to_string())
        .with_category("Spindle".to_string()));
        
        library.add_command(UserCommand::new(
            "Spindle Off".to_string(),
            vec!["M5".to_string()],
        )
        .with_description("Stop spindle".to_string())
        .with_category("Spindle".to_string()));
        
        library.add_command(UserCommand::new(
            "Coolant On".to_string(),
            vec!["M8".to_string()],
        )
        .with_description("Turn on coolant".to_string())
        .with_category("Coolant".to_string()));
        
        library.add_command(UserCommand::new(
            "Coolant Off".to_string(),
            vec!["M9".to_string()],
        )
        .with_description("Turn off coolant".to_string())
        .with_category("Coolant".to_string()));
        
        library.add_command(UserCommand::new(
            "Check Mode On".to_string(),
            vec!["$C".to_string()],
        )
        .with_description("Enable check mode (simulate without moving)".to_string())
        .with_category("Safety".to_string()));
        
        library.add_command(UserCommand::new(
            "Check Mode Off".to_string(),
            vec!["$C".to_string()],
        )
        .with_description("Disable check mode".to_string())
        .with_category("Safety".to_string()));
        
        library
    }
    
    /// Add a command
    pub fn add_command(&mut self, command: UserCommand) {
        self.commands.push(command);
    }
    
    /// Remove a command by name
    pub fn remove_command(&mut self, name: &str) -> bool {
        if let Some(index) = self.commands.iter().position(|c| c.name == name) {
            self.commands.remove(index);
            true
        } else {
            false
        }
    }
    
    /// Get a command by name
    pub fn get_command(&self, name: &str) -> Option<&UserCommand> {
        self.commands.iter().find(|c| c.name == name)
    }
    
    /// Get a mutable command by name
    pub fn get_command_mut(&mut self, name: &str) -> Option<&mut UserCommand> {
        self.commands.iter_mut().find(|c| c.name == name)
    }
    
    /// List all command names
    pub fn list_commands(&self) -> Vec<String> {
        self.commands.iter().map(|c| c.name.clone()).collect()
    }
    
    /// Get commands by category
    pub fn commands_by_category(&self, category: &str) -> Vec<&UserCommand> {
        self.commands.iter().filter(|c| c.category == category).collect()
    }
    
    /// Get all categories
    pub fn categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self.commands
            .iter()
            .map(|c| c.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        categories
    }
}
