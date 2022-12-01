use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Strategy {
    SustainedLoad,
    OnDemand,
    Powersave,
}