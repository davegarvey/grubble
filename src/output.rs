use clap::ValueEnum;

#[derive(ValueEnum, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Output {
    #[default]
    Text,
    Json,
}
