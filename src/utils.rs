use std::process::Command;
use nix::sys::ptrace::cont;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Percentage(u8);

impl Percentage {
    fn new(value: u8) -> Result<Percentage, String> {
        if value > 100 {
            return Err(format!("Percentage must be between 0 and 100, got {}", value));
        }

        Ok(Percentage(value))
    }

    fn value(&self) -> u8 {
        self.0
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Temperature(f64);

impl Temperature {
    fn new(value: f64) -> Temperature {
        Temperature(value)
    }

    fn value(&self) -> f64 {
        self.0
    }
}


pub fn get_current_temp() -> Temperature {
    // Run the "sensors -j" command and capture its output
    let output = Command::new("sensors")
        .arg("-j")
        .output()
        .expect("Failed to run 'sensors -j' command");

    // Parse the output into a JSON value
    let json: Value = serde_json::from_slice(output.stdout.as_slice()).unwrap();

    // Check if the "coretemp-isa-0000" entry exists in the JSON
    let Some(coretemp) = json["coretemp-isa-0000"].as_object() else {
        panic!("Error: 'coretemp-isa-0000' entry not found in JSON output");
    };

    let mut temps = Vec::new();

    for (k, v) in coretemp {
        if !k.starts_with("Core ") { continue; };

        for (temp_name, temp_val) in v.as_object().unwrap() {
            if !temp_name.ends_with("_input") { continue; }
            let Some(it) = temp_val.as_f64() else { continue; };

            temps.push(it);
        }
    };

    // TODO: Should this really be the average or rather the max?
    Temperature::new(temps.iter().sum::<f64>() / temps.len() as f64)
}