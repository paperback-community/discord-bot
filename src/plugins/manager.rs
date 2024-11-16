use wasmer::Instance;
pub use crate::plugins::plugin::{PluginLike};

struct Manager {
    plugins: Vec<Box<dyn PluginLike>>,
}
trait ManagerLike {
    fn new() -> Self;
    fn add_plugin(&mut self, plugin: Box<dyn PluginLike>);
    fn remove_plugin(&mut self, plugin: Box<dyn PluginLike>);
    fn run_command(&self, command: String, args: Vec<String>) -> Result<String, String>;
}

impl ManagerLike for Manager {
    fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    fn add_plugin(&mut self, plugin: Box<dyn PluginLike>) {
        for p in &self.plugins {
            if p.name() == plugin.name() {
                panic!("Plugin with the same name has already been registered. Consider removing it first with remove_plugin()");
            }
            for (command, _) in &plugin.commands() {
                if p.commands().contains_key(command) {
                    panic!(format!("Command with the same name has already been registered for plugin '{}'. Consider removing it first with remove_plugin()", p.name()));
                }
            }
        }

        self.plugins.push(plugin);
    }

    fn remove_plugin(&mut self, plugin: Box<dyn PluginLike>) {
        self.plugins.retain(|p| p != &plugin);
    }

    fn run_command(&self, command: String, args: Vec<String>) -> Result<String, String> {
        for plugin in &self.plugins {
            return match plugin.run_command(command.clone(), args.clone()) {
                Ok(value) => Ok(value),
                Err(err) => Err(err),
            }
        }
        Err("Command not found".to_string())
    }
}