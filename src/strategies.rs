use crate::utils::Percentage;
use clap::builder::Str;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Strategy {
    SustainedLoad,
    OnDemand,
    Powersave,
}

impl Default for Strategy {
    fn default() -> Self {
        Strategy::OnDemand
    }
}

trait PowerStrategy {
    fn get_fan_percent(temp: i32) -> Percentage {
        todo!()
    }
}
