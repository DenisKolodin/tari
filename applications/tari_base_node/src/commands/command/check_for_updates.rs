use std::fmt;

use anyhow::Error;
use async_trait::async_trait;
use clap::Parser;
use tari_p2p::auto_update::{SoftwareUpdate, SoftwareUpdaterHandle};

use super::TypedCommandPerformer;
use crate::builder::BaseNodeContext;

pub struct CheckForUpdatesCommand {
    software_updater: SoftwareUpdaterHandle,
}

impl CheckForUpdatesCommand {
    pub fn new(ctx: &BaseNodeContext) -> Self {
        Self {
            software_updater: ctx.software_updater(),
        }
    }
}

#[async_trait]
impl<'t> TypedCommandPerformer<'t> for CheckForUpdatesCommand {
    type Args = CheckForUpdatesArgs;
    type Report = CheckForUpdatesReport;

    fn command_name(&self) -> &'static str {
        "check-for-updates"
    }

    async fn perform_command(&'t mut self, args: Self::Args) -> Result<Self::Report, Error> {
        // TODO: `Checking for updates banner?`
        let update = self.software_updater.check_for_updates().await;
        Ok(Self::Report { update })
    }
}

#[derive(Parser, Debug)]
pub struct CheckForUpdatesArgs {}

pub struct CheckForUpdatesReport {
    update: Option<SoftwareUpdate>,
}

impl fmt::Display for CheckForUpdatesReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(update) = self.update.as_ref() {
            writeln!(
                f,
                "Version {} of the {} is available: {} (sha: {})",
                update.version(),
                update.app(),
                update.download_url(),
                update.to_hash_hex()
            )
        } else {
            writeln!(f, "No updates found.")
        }
    }
}
