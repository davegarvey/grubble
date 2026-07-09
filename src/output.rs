use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Output {
    Text,
    Json,
}

impl Default for Output {
    fn default() -> Self {
        Output::Text
    }
}
