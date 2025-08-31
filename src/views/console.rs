use crate::prelude::*;
use std::io::{self, Write};

pub struct ConsoleView;

impl ConsoleView {
    pub fn new() -> Self {
        Self
    }
    
    pub fn print_header(&self, title: &str) {
        let width = 60;
        println!("\n{}", "=".repeat(width));
        println!("{:^width$}", title);
        println!("{}", "=".repeat(width));
    }
    
    pub fn print_subheader(&self, title: &str) {
        let width = 50;
        println!("\n{}", "-".repeat(width));
        println!("{:^width$}", title);
        println!("{}", "-".repeat(width));
    }
    
    pub fn print_success(&self, message: &str) {
        println!("✅ {}", message);
    }
    
    pub fn print_error(&self, message: &str) {
        eprintln!("❌ {}", message);
    }
    
    pub fn print_warning(&self, message: &str) {
        println!("⚠️ {}", message);
    }
    
    pub fn print_info(&self, message: &str) {
        println!("ℹ️ {}", message);
    }
    
    pub fn print_progress(&self, current: usize, total: usize, description: &str) {
        println!("🔄 {}/{}: {}", current, total, description);
    }
    
    pub fn print_separator(&self) {
        println!("{}", "=".repeat(80));
    }
    
    pub fn print_menu_options(&self, options: &[(&str, &str)]) {
        for (key, description) in options {
            println!("{}. {}", key, description);
        }
    }
    
    pub fn print_goodbye(&self) {
        println!("👋 Goodbye!");
    }
    
    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }
    
    pub fn pause_for_input(&self, message: &str) -> Result<()> {
        println!("\n{}", message);
        self.get_input("")?;
        Ok(())
    }
    
    pub fn get_input(&self, prompt: &str) -> Result<String> {
        if !prompt.is_empty() {
            print!("{}", prompt);
            io::stdout().flush().map_err(Error::Io)?;
        }
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(Error::Io)?;
        
        Ok(input.trim().to_string())
    }
    
    pub fn get_number<T>(&self, prompt: &str, default: Option<T>) -> Result<T>
    where
        T: std::str::FromStr + std::fmt::Display + Copy,
        T::Err: std::fmt::Display,
    {
        let full_prompt = if let Some(default_val) = default {
            format!("{} (default: {}): ", prompt, default_val)
        } else {
            format!("{}: ", prompt)
        };
        
        let input = self.get_input(&full_prompt)?;
        
        if input.is_empty() {
            if let Some(default_val) = default {
                Ok(default_val)
            } else {
                Err(Error::input("No input provided and no default value"))
            }
        } else {
            input.parse().map_err(|e| Error::validation(format!("Invalid number: {}", e)))
        }
    }
    
    pub fn get_string(&self, prompt: &str, default: Option<&str>) -> Result<String> {
        let full_prompt = if let Some(default_val) = default {
            format!("{} (default: {}): ", prompt, default_val)
        } else {
            format!("{}: ", prompt)
        };
        
        let input = self.get_input(&full_prompt)?;
        
        if input.is_empty() {
            if let Some(default_val) = default {
                Ok(default_val.to_string())
            } else {
                Err(Error::input("No input provided and no default value"))
            }
        } else {
            Ok(input)
        }
    }
    
    pub fn confirm(&self, message: &str, default: bool) -> Result<bool> {
        let suffix = if default { " [Y/n]" } else { " [y/N]" };
        let prompt = format!("{}{}: ", message, suffix);
        
        let input = self.get_input(&prompt)?.to_lowercase();
        
        match input.as_str() {
            "" => Ok(default),
            "y" | "yes" | "true" => Ok(true),
            "n" | "no" | "false" => Ok(false),
            _ => Err(Error::validation("Please enter y/yes or n/no")),
        }
    }
    
    pub fn wait_for_enter(&self, message: &str) {
        println!("{}", message);
        let _ = self.get_input("");
    }
}

impl Default for ConsoleView {
    fn default() -> Self {
        Self::new()
    }
}
