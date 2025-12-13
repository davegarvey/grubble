use crate::config::Config;
use crate::error::BumperResult;
use crate::versioner::Version;

pub trait Strategy {
    fn get_current_version(&self) -> BumperResult<Version>;
    fn update_files(&self, new_version: &Version) -> BumperResult<Vec<String>>;
}

pub mod git;
pub mod node;
pub mod rust;

pub fn load_strategy(config: &Config) -> Box<dyn Strategy> {
    if config.raw {
        return Box::new(git::GitStrategy::new(config.clone()));
    }

    match config.preset.as_str() {
        "node" => Box::new(node::NodeStrategy::new(config.clone())),
        "git" => Box::new(git::GitStrategy::new(config.clone())),
        "rust" => Box::new(rust::RustStrategy::new(config.clone())),
        _ => Box::new(git::GitStrategy::new(config.clone())),
    }
}
