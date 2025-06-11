use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Instant;

use riv::model::ir::atom::Atom;
use crate::engines::riv::RivCommand;

/// Background parser: owns the command receiver and publishes parsed `Atom`s.
pub struct RivParser 
{
    cmd_rx:  Receiver<RivCommand>,
    atom_tx: Sender<Atom>,
}

impl RivParser {
    /* ───────── construction & spawning ─────────────────────────────── */
    pub fn new(cmd_rx: Receiver<RivCommand>, atom_tx: Sender<Atom>) -> Self {
        Self {cmd_rx, atom_tx}
    }

    /// Spawn the worker thread and detach it.
    ///
    /// ```rust
    /// use apex::engines::riv::RivCommand;
    /// use apex::engines::riv::riv_parser::RivParser;
    /// use riv::model::ir::atom::Atom;
    /// use std::sync::mpsc;
    /// let (cmd_tx, cmd_rx)   = mpsc::channel::<RivCommand>();
    /// let (atom_tx, atom_rx) = mpsc::channel::<Atom>();
    ///
    /// RivParser::spawn(cmd_rx, atom_tx);
    /// // keep `cmd_tx` to send parse jobs; poll `atom_rx` for results
    /// ```
    pub fn spawn(cmd_rx: Receiver<RivCommand>, atom_tx: Sender<Atom>) {
        thread::Builder::new()
            .name("riv-parser".into())
            .spawn(move || {
                let worker = RivParser::new(cmd_rx, atom_tx);
                worker.run();
            })
            .expect("failed to spawn riv-parser thread");
    }

    /* ───────── main loop ───────────────────────────────────────────── */

    fn run(self) {
        while let Ok(cmd) = self.cmd_rx.recv() {
            match cmd {
                RivCommand::Parse { file } => {
                    if let Err(e) = self.handle_parse(file) {
                        eprintln!("[RivParser] parse error: {e}");
                    }
                }
                RivCommand::Quit => break,
                // add more commands here
            }
        }
        eprintln!("[RivParser] shutting down");
    }

    /* ───────── command handlers ───────────────────────────────────── */

    fn handle_parse(&self, file: PathBuf) -> io::Result<()> {
        let started    = Instant::now();
        let f          = File::open(&file)?;
        let mut reader = BufReader::new(f);

        // example: read file line-by-line and convert to Atoms
        let mut line = String::new();
        while reader.read_line(&mut line)? != 0 {
            // business-specific: turn the string into your IR
            if let Some(atom) = line_to_atom(&line) {
                // failure to send means UI side went away → give up
                if self.atom_tx.send(atom).is_err() {
                    break;
                }
            }
            line.clear();
        }

        let elapsed = started.elapsed();
        eprintln!(
            "[RivParser] parsed {:?} in {:.2?}",
            file.file_name().unwrap_or_default(),
            elapsed
        );
        Ok(())
    }
}

/* ───────── helpers / stubs ────────────────────────────────────────── */

/// Very small example IR conversion.
/// FIXME: Replace this with real parsing logic.
/// 
fn line_to_atom(line: &str) -> Option<Atom> {
    if line.trim().is_empty() {
        None
    } else {
    	let atom = Atom::EndTask;
        Some(atom)
    }
}
