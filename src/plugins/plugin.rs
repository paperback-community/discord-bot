use std::collections::HashMap;
use std::path::PathBuf;
use wasmer::{imports, Function, Instance, Store};

struct Plugin {
    name: String,
    version: String,
    description: String,
    commands: HashMap<String, Box<Function>>
}
pub trait PluginLike {
    fn new(name: String, version: String, description: String, commands: HashMap<String, Box<Function>>) -> Self;
    fn import(name: String, version: String, description: String, command_names: HashMap<String, String>, wasm_path: PathBuf) -> Self {
        let engine = wasmer::Cranelift::new();
        // TODO: Could simplify this to directly iterate over exports() from module
        let mut store = Store::default();
        let imports = imports! {};

        let module = wasmer::Module::from_file(&engine, &wasm_path).unwrap();
        let instance = Instance::new(&mut store, &module, &imports).unwrap();

        let mut commands = HashMap::new();
        for (command_name, function_name) in command_names {
            let func = instance.exports.get_function(&function_name).unwrap();
            commands.insert(command_name, Box::new(func));
        }

        Self {
            name,
            version,
            description,
            commands
        }
    }
    fn name(&self) -> &str;
    fn run_command(&self, command: String, args: Vec<String>) -> Result<String, String>;
}

impl PluginLike for Plugin {
    fn new(name: String, version: String, description: String, commands: HashMap<String, Box<Function>>) -> Self {
        Self {
            name,
            version,
            description,
            commands
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn run_command(&self, command: String, args: Vec<String>) -> Result<String, String> {
        let args: Vec<wasmer::Value> = args.iter().map(|arg| {
            wasmer::Value::I32(arg.parse().unwrap())
        }).collect();

        match self.commands.get(&command) {
            Some(func) => {
                let mut store = Store::default();
                let result = func.call(&mut store, &args);
                match result {
                    Ok(value) => Ok(value.to_string()),
                    Err(err) => Err(err.to_string())
                }
            }
            None => Err("Command not found".to_string())
        }
    }
}