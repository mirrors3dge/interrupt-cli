#[cfg(feature = "clear-output")]
use crate::term_utils::clear_terminal;
use crate::{Command, Interrupt};
#[cfg(feature = "term-utils")]
use crate::{println_red, println_yellow};
#[cfg(not(feature = "term-utils"))]
use println as println_red;
#[cfg(not(feature = "term-utils"))]
use println as println_yellow;

use std::cell::LazyCell;
use std::io::{self, Write as _};
use std::sync::mpsc::{
    self, Receiver, RecvError, RecvTimeoutError, Sender, SyncSender, TryRecvError, TrySendError,
};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

static INPUT_READER: Mutex<LazyCell<InputReader>> = Mutex::new(LazyCell::new(InputReader::new));

struct InputReader {
    request_tx: SyncSender<()>,
    input_rx: Receiver<String>,
}

impl InputReader {
    fn new() -> Self {
        let (request_tx, request_rx): (SyncSender<()>, Receiver<()>) = mpsc::sync_channel(0);
        let (input_tx, input_rx): (Sender<String>, Receiver<String>) = mpsc::channel();

        let input_reader = Self {
            request_tx,
            input_rx,
        };

        // the idea is to spawn a background thread handling stdin through a rendez-vous channel
        // so it allows cancelling blocking reads with a timeout
        std::thread::spawn(move || {
            let stdin = io::stdin();

            // block until receiving next input request
            while request_rx.recv().is_ok() {
                // block until receiving a new line from stdin
                let mut buf = String::new();
                if 0 == stdin.read_line(&mut buf).unwrap_or(0) {
                    return; // EOF
                }

                let line_len = buf.trim_end_matches(['\r', '\n']).len();
                buf.truncate(line_len);

                // send new line
                if input_tx.send(buf).is_err() {
                    return; // input receiver disconnected
                }
            }
        });

        // before allowing to send non-blocking requests to the input thread from other functions
        // we must wait for it to be ready (otherwise, non-blocking requests are sent to the void)
        // that's why the first read request must be blocking, which is okay to already send here
        // because this constructor is called on the first input read, meaning the user actually
        // already asked for a read
        input_reader
            .request_tx
            .send(())
            .expect("input thread disconnected");
        input_reader
    }

    /// Return the next line from stdin. Doesn't parse interrupt commands.
    ///
    /// `Err(RecvError)` is returned if stdin is closed.
    ///
    /// This will block until prior calls to `self.read()` or `self.read_timeout()` have returned.
    fn read(&self) -> Result<String, RecvError> {
        // try to get buffered input before sending read request
        match self.input_rx.try_recv() {
            Ok(buffered) => return Ok(buffered),
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => return Err(RecvError),
        }

        match self.request_tx.try_send(()) {
            Ok(()) => (),                      // input thread was waiting for a request -> ok!
            Err(TrySendError::Full(())) => (), // input thread is blocked on stdin -> no request needed
            Err(TrySendError::Disconnected(())) => return Err(RecvError),
        }

        self.input_rx.recv()
    }

    /// Return the next line from stdin. Doesn't parse interrupt commands.
    ///
    /// - `Err(RecvTimeoutError::Timeout)` is returned if the timeout was reached before any input was entered.
    /// - `Err(RecvTimeoutError::Disconnected)` is returned if stdin is closed.
    ///
    /// This will block until prior calls to `self.read()` or `self.read_timeout()` have returned.
    ///
    /// This function has the disadvantage to lock stdin until the next line read, even past the timeout.
    /// This is a known limitation arising from the inability to interrupt a blocking `read_line` call.
    fn read_timeout(&self, timeout: Duration) -> Result<String, RecvTimeoutError> {
        // try to get buffered input before sending read request
        match self.input_rx.try_recv() {
            Ok(buffered) => return Ok(buffered),
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => return Err(RecvTimeoutError::Disconnected),
        }

        match self.request_tx.try_send(()) {
            Ok(()) => (),                      // input thread was waiting for a request -> ok!
            Err(TrySendError::Full(())) => (), // input thread is blocked on stdin -> no request needed
            Err(TrySendError::Disconnected(())) => return Err(RecvTimeoutError::Disconnected),
        }

        self.input_rx.recv_timeout(timeout)
    }
}

const NO_HELP_MSG: &str = "No help available for this prompt.";
const FORMAT_WIDTH: usize = 80;
static HELP_KEYWORD_WIDTH: OnceLock<usize> = OnceLock::new();

/// Result of a prompt with timeout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputTimeout<T> {
    /// An input was provided.
    Input(T),
    /// No input was provided before timeout.
    Timeout,
}

/// Result of an optional prompt with timeout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OptionalInputTimeout<T> {
    /// An input was provided.
    Input(T),
    /// The user explicitly entered nothing.
    None,
    /// No input was provided before timeout.
    Timeout,
}

/// Prompts the user to confirm the exit of the program and return the response as a bool.
/// If stdin is closed, returns true in order to gracefully exit the program.
fn confirm_exit() -> bool {
    loop {
        print!("Confirm exit? (y/n): ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input: String = match INPUT_READER.lock().expect("mutex poisoned").read() {
            Ok(input) => input,
            Err(RecvError) => {
                //gracefully exit when stdin is closed
                println_yellow!("stdin is closed, exiting gracefully");
                return true;
            }
        };

        input.make_ascii_lowercase();
        match input.as_str() {
            "yes" | "y" | "" => return true,
            "no" | "n" => return false,
            _ => println_red!("'{input}' is not a recognized input, type y/n to confirm/deny"),
        }
    }
}

/// Assuming print width of 80 chars.
fn print_help<C: Command>(help: &str) {
    fn print_command_help(mut kw: &str, descr: &str, kw_width: usize) {
        let descr_width: usize = FORMAT_WIDTH - 2 - kw_width;

        // divide `descr` in multiple lines
        let lines = descr.lines().flat_map(|line| {
            // divide each line longer than `descr_width` into nested lines to fit in format
            let mut nested_lines: Vec<String> = vec![String::new()];
            for (idx, ch) in line.char_indices() {
                if idx != 0 && idx % descr_width == 0 {
                    nested_lines.push(String::new());
                }
                nested_lines.last_mut().unwrap().push(ch); // `nested_lines` is never empty
            }
            nested_lines
        });

        for line in lines {
            // each line is garanteed to fit in format
            println!("{:<kw_width$}  {:<descr_width$}", kw, line);
            kw = "";
        }
    }

    let kw_width: usize = *HELP_KEYWORD_WIDTH.get_or_init(|| {
        C::help()
            .iter()
            .map(|(kw, _)| kw.chars().count())
            .max()
            .unwrap_or_default()
            .min(FORMAT_WIDTH / 3)
    });

    // start delimiter
    println!("\n{:-^FORMAT_WIDTH$}", "[ HELP ]");

    // help message
    println!("{}", help);

    // commands help
    println!("\n - Commands:");
    print_command_help("exit", "exit the program", kw_width);
    print_command_help("help", "show this help", kw_width);
    for (kw, descr) in C::help() {
        print_command_help(kw, descr, kw_width);
    }

    // end delimiter
    println!("{:-^FORMAT_WIDTH$}\n", "");
}

/// Return the next line from stdin. Parses interrupt commands.
///
/// This will block until prior calls to `read_input` or `read_input_timeout` have returned.
///
/// ## Command parse order:
///
/// 1) default commands
/// 2) normal commands
/// 3) internal commands
pub(crate) fn read_input<C: Command>(
    msg: impl AsRef<str>,
    help: Option<&impl AsRef<str>>,
) -> Result<String, Interrupt<C>> {
    let msg: &str = msg.as_ref();
    let help: &str = help
        .as_ref()
        .map_or(NO_HELP_MSG, std::convert::AsRef::as_ref);

    loop {
        // prompt
        print!("{}: ", msg);

        io::stdout().flush().expect("failed to flush stdout");

        // read input
        let input: Result<String, RecvError> = INPUT_READER.lock().expect("mutex poisoned").read();

        #[cfg(feature = "clear-output")]
        clear_terminal();

        let input: String = match input {
            Ok(input) => input,
            Err(RecvError) => {
                //gracefully exit when stdin is closed
                println_yellow!("stdin is closed, exiting gracefully");
                return Err(Interrupt::Exit);
            }
        };

        // parse default command
        match input.to_lowercase().as_str() {
            "exit" => {
                if confirm_exit() {
                    return Err(Interrupt::Exit);
                }
                continue;
            }
            "help" => {
                print_help::<C>(help);
                continue;
            }
            _ => (),
        }

        // parse command
        if let Ok(cmd) = input.parse::<C>() {
            return Err(Interrupt::Cmd(cmd));
        }

        // parse internal command
        if let Ok(cmd) = input.parse::<C::Internal>() {
            C::internal(cmd);
            continue;
        }

        return Ok(input);
    }
}

/// Return the next line from stdin. Parses interrupt commands.
///
/// `Ok(InputTimeout::Timeout)` is returned if the timeout is reached before any input was entered.
///
/// This will block until prior calls to `read_input()` or `read_input_timeout()` have returned.
///
/// This function has the disadvantage to lock stdin until the next line read, even past the timeout.
///
/// ## Command parse order:
///
/// 1) default commands
/// 2) normal commands
/// 3) internal commands
pub(crate) fn read_input_timeout<C: Command>(
    msg: impl AsRef<str>,
    help: Option<&impl AsRef<str>>,
    timeout: Duration,
) -> Result<InputTimeout<String>, Interrupt<C>> {
    let msg: &str = msg.as_ref();
    let help: &str = help
        .as_ref()
        .map_or(NO_HELP_MSG, std::convert::AsRef::as_ref);

    loop {
        // prompt
        print!("{}: ", msg);
        io::stdout().flush().expect("failed to flush stdout");

        // read input
        let input: Result<String, RecvTimeoutError> = INPUT_READER
            .lock()
            .expect("mutex poisoned")
            .read_timeout(timeout);

        if let Err(RecvTimeoutError::Timeout) = &input {
            // since the user didn't press enter, print newline to remain consistent with other scenarios
            println!();
        }
        #[cfg(feature = "clear-output")]
        clear_terminal();

        let input: String = match input {
            Ok(input) => input,
            Err(RecvTimeoutError::Disconnected) => {
                //gracefully exit when stdin is closed
                println_yellow!("stdin is closed, exiting gracefully");
                return Err(Interrupt::Exit);
            }
            Err(RecvTimeoutError::Timeout) => return Ok(InputTimeout::Timeout),
        };

        // parse default command
        match input.to_lowercase().as_str() {
            "exit" => {
                if confirm_exit() {
                    return Err(Interrupt::Exit);
                }
                continue;
            }
            "help" => {
                print_help::<C>(help);
                continue;
            }
            _ => (),
        }

        // parse command
        if let Ok(cmd) = input.parse::<C>() {
            return Err(Interrupt::Cmd(cmd));
        }

        // parse internal command
        if let Ok(cmd) = input.parse::<C::Internal>() {
            C::internal(cmd);
            continue;
        }

        return Ok(InputTimeout::Input(input));
    }
}
