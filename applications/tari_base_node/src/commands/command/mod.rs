mod state_info;
use std::fmt::Display;

use anyhow::Error;
use async_trait::async_trait;
use clap::Parser;
pub use state_info::StateInfoCommand;

#[async_trait]
pub trait TypedCommandPerformer<'t>: Send + Sync + 'static {
    type Args: Parser + Send;
    type Report: Display + 't;

    fn command_name(&self) -> &'static str;
    async fn perform_command(&'t mut self, args: Self::Args) -> Result<Self::Report, Error>;
}
