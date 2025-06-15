
pub mod engine;
pub mod parse_helper;
pub mod component_configuration;

#[cfg(test)]
mod config_tests;

/// Messages that the UI layer can send to the background worker.
/// 
#[derive(Debug, Clone)]
pub enum RivCommand {
    Parse,
    Analyze,
    Blueprint,
    Publish,
    Quit,
}
