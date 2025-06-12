
pub mod riv_parser;
mod parse_helper;

/// Messages that the UI layer can send to the background worker.
/// 
#[derive(Debug)]
pub enum RivCommand {
    Parse { file: std::path::PathBuf },
    Publish,
    Quit,
}
