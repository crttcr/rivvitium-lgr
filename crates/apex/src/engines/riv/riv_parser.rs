use std::io;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Instant;
use tracing::info;
use riv::model::ir::atom::Atom;
use crate::engines::riv::parse_helper::open_source;
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
                RivCommand::Analyze => continue,
                RivCommand::Blueprint => continue,
                RivCommand::Publish => continue,
                RivCommand::Quit    => break,
                RivCommand::Parse   => {
                    if let Err(e) = self.handle_parse() {
                        eprintln!("[RivParser] parse error: {e}");
                    }
                }
            }
        }
        info!("[RivParser] shutting down");
    }

    /* ───────── command handlers ───────────────────────────────────── */

    //fn handle_parse(&self, file: PathBuf) -> io::Result<()> {
    fn handle_parse(&self) -> io::Result<()> {
        let started    = Instant::now();
        info!("Parse started: {:?}", started.elapsed());
        let file = unimplemented!();
        let mut source = open_source(&file).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
			info!("Start pulling atoms from source");
        for atom in &mut source {
        	match self.atom_tx.send(atom) {
        	Ok(_) => {},
        		Err(_) => {
        			info!("Failed to send atom to UI");
        			break;
        		}
        	}
        }
        let ended = started.elapsed();
        info!("Parse ended: {:?}", ended);
        Ok(())
    }
}
