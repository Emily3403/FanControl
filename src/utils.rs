use serde::{Deserialize, Serialize};

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