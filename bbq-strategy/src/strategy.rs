#[allow(dead_code)]

use anyhow::Result;
use async_trait::async_trait;
use bbq_core::{Event, QuotData};

#[async_trait]
pub trait Strategy: Send + Sync {
    fn name(&self) -> String;
    async fn on_init(
        &mut self
    ) -> Result<()> {
        Ok(())
    }
    async fn on_destroy(&self) -> Result<()> {
        Ok(())
    }
    async fn on_open(&mut self, _quot: &QuotData) -> Result<Option<Event>> {
        Ok(None)
    }
    async fn on_close(&mut self, _quot: &QuotData) -> Result<Option<Event>> {
        Ok(None)
    }
    async fn on_quot(&mut self, _quot: &QuotData) -> Result<Option<Event>> {
        Ok(None)
    }
}
