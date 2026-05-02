pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    ctx: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ctx)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self { ctx: value }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            ctx: value.to_string(),
        }
    }
}

impl From<flagge::Error> for Error {
    fn from(value: flagge::Error) -> Self {
        Self {
            ctx: value.to_string(),
        }
    }
}

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

macro_rules! log {
    ($loglevel:ident, $($arg:tt)*) => {
        use std::io::IsTerminal;
        let is_tty = std::io::stdout().is_terminal(); // To ensure ansi escape codes are supported.

        let (ansi_red, ansi_yellow, ansi_green, ansi_reset) = if is_tty {
            ("\x1b[0;31m", "\x1b[0;33m", "\x1b[0;32m", "\x1b[0m")
        } else {
            ("", "", "", "")
        };

        match LogLevel::$loglevel {
            LogLevel::Info => {
                print!("{ansi_green}INFO{ansi_reset}: ");
                println!($($arg)*);
            }
            LogLevel::Warning => {
                print!("{ansi_yellow}WARNING{ansi_reset}: ");
                println!($($arg)*);
            }
            LogLevel::Error => {
                eprint!("{ansi_red}ERROR{ansi_reset}: ");
                eprintln!($($arg)*);
            }
        }
    };
}
