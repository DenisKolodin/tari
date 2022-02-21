use std::fmt;

use anyhow::Error;
use async_trait::async_trait;
use clap::Parser;
use tari_core::base_node::state_machine_service::states::StatusInfo;
use tokio::sync::watch;

use super::TypedCommandPerformer;
use crate::builder::BaseNodeContext;

pub struct StateInfoCommand {
    state_machine_info: watch::Receiver<StatusInfo>,
}

impl StateInfoCommand {
    pub fn new(ctx: &BaseNodeContext) -> Self {
        Self {
            state_machine_info: ctx.get_state_machine_info_channel(),
        }
    }
}

#[async_trait]
impl<'t> TypedCommandPerformer<'t> for StateInfoCommand {
    type Args = StateInfoArgs;
    type Report = StateInfoReport<'t>;

    fn command_name(&self) -> &'static str {
        "state-info"
    }

    async fn perform_command(&'t mut self, args: Self::Args) -> Result<Self::Report, Error> {
        Ok(Self::Report {
            status_info: self.state_machine_info.borrow(),
        })
    }
}

#[derive(Parser, Debug)]
pub struct StateInfoArgs {}

pub struct StateInfoReport<'t> {
    status_info: watch::Ref<'t, StatusInfo>,
}

impl<'t> fmt::Display for StateInfoReport<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Current state machine state:\n{}", *self.status_info)
    }
}
