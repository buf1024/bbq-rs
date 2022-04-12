use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Risk: Send + Sync {
    fn name(&self) -> String;
    async fn on_init(&mut self) -> Result<()> {
        Ok(())
    }
    async fn on_destroy(&self) -> Result<()> {
        Ok(())
    }

    async fn on_risk(&mut self) -> Result<()> {
        Ok(())
    }
}
