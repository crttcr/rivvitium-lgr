
pub mod riv_parser;

/// Messages that the UI layer can send to the background worker.
/// 
#[derive(Debug)]
pub enum RivCommand {
    Parse { file: std::path::PathBuf },
    Quit,
}
