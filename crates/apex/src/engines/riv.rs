
pub mod parse_helper;
pub mod pipe_metrics;
pub mod riv_parser;

/// Messages that the UI layer can send to the background worker.
/// 
#[derive(Debug, Clone)]
pub enum RivCommand {
    Parse { file: std::path::PathBuf },
    Publish,
    Quit,
}
