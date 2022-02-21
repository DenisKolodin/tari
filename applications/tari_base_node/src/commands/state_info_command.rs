use super::{args::Args, performer::TypedCommandPerformer};
use anyhow::Error;
use async_trait::async_trait;
use std::fmt;
use tari_core::base_node::state_machine_service::states::StatusInfo;
use tokio::sync::watch;

pub struct StateInfoCommand {
    state_machine_info: watch::Receiver<StatusInfo>,
}

#[async_trait]
impl<'t> TypedCommandPerformer<'t> for StateInfoCommand {
    type Report = StateInfoReport<'t>;

    fn command_name(&self) -> &'static str {
        "state-info"
    }

    async fn perform_command(&'t mut self, args: Args<'t>) -> Result<Self::Report, Error> {
        Ok(Self::Report {
            status_info: self.state_machine_info.borrow(),
        })
    }
}

pub struct StateInfoReport<'a> {
    status_info: watch::Ref<'a, StatusInfo>,
}

impl<'a> fmt::Display for StateInfoReport<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current state machine state:\n{}\n", *self.status_info)
    }
}
