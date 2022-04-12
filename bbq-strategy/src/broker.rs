use anyhow::Result;
use async_trait::async_trait;
use bbq_core::Entrust;

#[async_trait]
pub trait Broker: Send + Sync {
    fn name(&self) -> String;
    async fn on_init(
        &mut self
    ) -> Result<()> {
        Ok(())
    }
    async fn on_destroy(&self) -> Result<()> {
        Ok(())
    }

    async fn on_entrust(&mut self, _entrust: &Entrust) -> Result<()> {
        Ok(())
    }
}
