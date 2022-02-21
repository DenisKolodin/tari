mod check_for_updates;
mod get_chain_meta;
mod print_version;
mod state_info;

use std::fmt::Display;

use anyhow::Error;
use async_trait::async_trait;
pub use check_for_updates::CheckForUpdatesCommand;
use clap::Parser;
pub use get_chain_meta::GetChainMetaCommand;
pub use print_version::PrintVersionCommand;
pub use state_info::StateInfoCommand;

#[async_trait]
pub trait TypedCommandPerformer<'t>: Send + Sync + 'static {
    type Args: Parser + Send;
    type Report: Display + 't;

    fn command_name(&self) -> &'static str;
    async fn perform_command(&'t mut self, args: Self::Args) -> Result<Self::Report, Error>;
}
