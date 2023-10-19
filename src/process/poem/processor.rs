use super::{args::PoemArgs, lib::handle};
use crate::AsyncProcessor;
use anyhow::{Ok, Result};
use async_trait::async_trait;

#[async_trait]
impl AsyncProcessor for PoemArgs {
    type Item = ();

    async fn run(&self) -> Result<()> {
        handle(self.target.clone());
        Ok(())
    }
}
