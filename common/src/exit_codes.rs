use std::fmt;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub struct ExitError {
    pub exit_code: ExitCode,
    pub details: Option<String>,
}

impl ExitError {
    pub fn new(exit_code: ExitCode, details: impl ToString) -> Self {
        let details = Some(details.to_string());
        Self { exit_code, details }
    }
}

impl From<ExitCode> for ExitError {
    fn from(exit_code: ExitCode) -> Self {
        Self {
            exit_code,
            details: None,
        }
    }
}

impl fmt::Display for ExitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let details = self.details.as_ref().map(String::as_ref).unwrap_or("");
        write!(f, "{} {}", self.exit_code, details)
    }
}

impl From<ExitCodes> for ExitError {
    fn from(codes: ExitCodes) -> Self {
        use ExitCodes::*;
        match codes {
            ConfigError(s) => Self::new(ExitCode::ConfigError, s),
            UnknownError(s) => Self::new(ExitCode::UnknownError, s),
            InterfaceError => Self::from(ExitCode::InterfaceError),
            WalletError(s) => Self::new(ExitCode::WalletError, s),
            GrpcError(s) => Self::new(ExitCode::GrpcError, s),
            InputError(s) => Self::new(ExitCode::InputError, s),
            CommandError(s) => Self::new(ExitCode::CommandError, s),
            IOError(s) => Self::new(ExitCode::IOError, s),
            RecoveryError(s) => Self::new(ExitCode::RecoveryError, s),
            NetworkError(s) => Self::new(ExitCode::NetworkError, s),
            ConversionError(s) => Self::new(ExitCode::ConversionError, s),
            IncorrectPassword => Self::from(ExitCode::IncorrectOrEmptyPassword),
            NoPassword => Self::from(ExitCode::IncorrectOrEmptyPassword),
            TorOffline => Self::from(ExitCode::TorOffline),
            DatabaseError(s) => Self::new(ExitCode::DatabaseError, s),
            DbInconsistentState(s) => Self::new(ExitCode::DbInconsistentState, s),
        }
    }
}

const TOR_HINT: &str = r#"\
Unable to connect to the Tor control port.

Please check that you have the Tor proxy running and \
that access to the Tor control port is turned on.

If you are unsure of what to do, use the following \
command to start the Tor proxy:
tor --allow-missing-torrc --ignore-missing-torrc \
--clientonly 1 --socksport 9050 --controlport \
127.0.0.1:9051 --log \"warn stdout\" --clientuseipv6 1
"#;

impl ExitCode {
    pub fn hint(&self) -> &str {
        use ExitCode::*;
        match self {
            TorOffline => TOR_HINT,
            _ => "",
        }
    }
}

/// Enum to show failure information
#[derive(Debug, Clone, Copy, Error)]
pub enum ExitCode {
    #[error("There is an error in the configuration.")]
    ConfigError = 101,
    #[error("The application exited because an unknown error occurred. Check the logs for more details.")]
    UnknownError = 102,
    #[error("The application exited because an interface error occurred. Check the logs for details.")]
    InterfaceError = 103,
    #[error("The application exited.")]
    WalletError = 104,
    #[error("The application was not able to start the GRPC server.")]
    GrpcError = 105,
    #[error("The application did not accept the command input.")]
    InputError = 106,
    #[error("Invalid command.")]
    CommandError = 107,
    #[error("IO error.")]
    IOError = 108,
    #[error("Recovery failed.")]
    RecoveryError = 109,
    #[error("The application exited because of an internal network error.")]
    NetworkError = 110,
    #[error("The application exited because it received a message it could not interpret.")]
    ConversionError = 111,
    #[error("Your password was incorrect or required, but not provided.")]
    IncorrectOrEmptyPassword = 112,
    #[error("Tor connection is offline")]
    TorOffline = 113,
    #[error("The application encountered a database error.")]
    DatabaseError = 114,
    #[error("Database is in an inconsistent state!")]
    DbInconsistentState = 115,
}

/// Enum to show failure information
#[derive(Debug, Clone, Error)]
pub enum ExitCodes {
    #[error("There is an error in the configuration: {0}")]
    ConfigError(String),
    #[error("The application exited because an unknown error occurred: {0}. Check the logs for more details.")]
    UnknownError(String),
    #[error("The application exited because an interface error occurred. Check the logs for details.")]
    InterfaceError,
    #[error("The application exited. {0}")]
    WalletError(String),
    #[error("The application was not able to start the GRPC server. {0}")]
    GrpcError(String),
    #[error("The application did not accept the command input: {0}")]
    InputError(String),
    #[error("Invalid command: {0}")]
    CommandError(String),
    #[error("IO error: {0}")]
    IOError(String),
    #[error("Recovery failed: {0}")]
    RecoveryError(String),
    #[error("The application exited because of an internal network error: {0}")]
    NetworkError(String),
    #[error("The application exited because it received a message it could not interpret: {0}")]
    ConversionError(String),
    #[error("Your password was incorrect.")]
    IncorrectPassword,
    #[error("Your wallet is encrypted but no password was provided.")]
    NoPassword,
    #[error("The application encountered a database error: {0}")]
    DatabaseError(String),
    #[error("Tor connection is offline")]
    TorOffline,
    #[error("Database is in an inconsistent state!: {0}")]
    DbInconsistentState(String),
}

impl ExitCodes {
    pub fn as_i32(&self) -> i32 {
        match self {
            Self::ConfigError(_) => 101,
            Self::UnknownError(_) => 102,
            Self::InterfaceError => 103,
            Self::WalletError(_) => 104,
            Self::GrpcError(_) => 105,
            Self::InputError(_) => 106,
            Self::CommandError(_) => 107,
            Self::IOError(_) => 108,
            Self::RecoveryError(_) => 109,
            Self::NetworkError(_) => 110,
            Self::ConversionError(_) => 111,
            Self::IncorrectPassword | Self::NoPassword => 112,
            Self::TorOffline => 113,
            Self::DatabaseError(_) => 114,
            Self::DbInconsistentState(_) => 115,
        }
    }

    pub fn eprint_details(&self) {
        use ExitCodes::*;
        match self {
            TorOffline => {
                eprintln!("Unable to connect to the Tor control port.");
                eprintln!(
                    "Please check that you have the Tor proxy running and that access to the Tor control port is \
                     turned on.",
                );
                eprintln!("If you are unsure of what to do, use the following command to start the Tor proxy:");
                eprintln!(
                    "tor --allow-missing-torrc --ignore-missing-torrc --clientonly 1 --socksport 9050 --controlport \
                     127.0.0.1:9051 --log \"warn stdout\" --clientuseipv6 1",
                );
            },
            e => {
                eprintln!("{}", e);
            },
        }
    }
}

impl From<super::ConfigError> for ExitError {
    fn from(err: super::ConfigError) -> Self {
        // TODO: Move it out
        // error!(target: LOG_TARGET, "{}", err);
        Self::new(ExitCode::ConfigError, err)
    }
}

impl From<crate::ConfigurationError> for ExitError {
    fn from(err: crate::ConfigurationError) -> Self {
        Self::new(ExitCode::ConfigError, err)
    }
}

impl From<multiaddr::Error> for ExitError {
    fn from(err: multiaddr::Error) -> Self {
        Self::new(ExitCode::ConfigError, err)
    }
}

impl From<std::io::Error> for ExitError {
    fn from(err: std::io::Error) -> Self {
        Self::new(ExitCode::IOError, err)
    }
}
