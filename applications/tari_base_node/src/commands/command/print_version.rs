use std::fmt;

use anyhow::Error;
use async_trait::async_trait;
use clap::Parser;
use tari_app_utilities::consts;
use tari_p2p::auto_update::{SoftwareUpdate, SoftwareUpdaterHandle};
use tokio::sync::watch;

use super::TypedCommandPerformer;
use crate::builder::BaseNodeContext;

pub struct PrintVersionCommand {
    software_updater: SoftwareUpdaterHandle,
}

impl PrintVersionCommand {
    pub fn new(ctx: &BaseNodeContext) -> Self {
        Self {
            software_updater: ctx.software_updater(),
        }
    }
}

#[async_trait]
impl<'t> TypedCommandPerformer<'t> for PrintVersionCommand {
    type Args = PrintVersionArgs;
    type Report = PrintVersionReport<'t>;

    fn command_name(&self) -> &'static str {
        "print-version"
    }

    async fn perform_command(&'t mut self, args: Self::Args) -> Result<Self::Report, Error> {
        let update = self.software_updater.new_update_notifier().borrow();
        Ok(Self::Report { update })
    }
}

#[derive(Parser, Debug)]
pub struct PrintVersionArgs {}

pub struct PrintVersionReport<'t> {
    update: watch::Ref<'t, Option<SoftwareUpdate>>,
}

impl<'t> fmt::Display for PrintVersionReport<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Version: {}", consts::APP_VERSION)?;
        writeln!(f, "Author: {}", consts::APP_AUTHOR)?;
        writeln!(f, "Avx2: {}", match cfg!(feature = "avx2") {
            true => "enabled",
            false => "disabled",
        })?;

        if let Some(update) = self.update.as_ref() {
            writeln!(
                f,
                "Version {} of the {} is available: {} (sha: {})",
                update.version(),
                update.app(),
                update.download_url(),
                update.to_hash_hex()
            )?;
        }
        Ok(())
    }
}
