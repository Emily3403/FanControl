use clap::ValueEnum;
use serde::{Serialize, Deserialize};


#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Strategy {
    SustainedLoad,
    OnDemand,
    Powersave,
}