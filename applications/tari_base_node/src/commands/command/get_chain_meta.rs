use anyhow::Error;
use async_trait::async_trait;
use clap::Parser;
use tari_common_types::chain_metadata::ChainMetadata;
use tari_core::base_node::LocalNodeCommsInterface;

use super::TypedCommandPerformer;
use crate::builder::BaseNodeContext;

pub struct GetChainMetaCommand {
    node_service: LocalNodeCommsInterface,
}

impl GetChainMetaCommand {
    pub fn new(ctx: &BaseNodeContext) -> Self {
        Self {
            node_service: ctx.local_node(),
        }
    }
}

#[async_trait]
impl<'t> TypedCommandPerformer<'t> for GetChainMetaCommand {
    type Args = GetChainMetaArgs;
    type Report = ChainMetadata;

    fn command_name(&self) -> &'static str {
        "get-chain-meta"
    }

    async fn perform_command(&'t mut self, args: Self::Args) -> Result<Self::Report, Error> {
        self.node_service.get_metadata().await.map_err(Error::from)
    }
}

#[derive(Parser, Debug)]
pub struct GetChainMetaArgs {}
