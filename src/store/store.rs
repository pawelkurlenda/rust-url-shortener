use async_trait::async_trait;

use crate::models::LinkRecord;

#[async_trait]
pub trait Store: Send + Sync + 'static {
    async fn get(&self, id: &str) -> anyhow::Result<Option<LinkRecord>>;
    async fn put(&self, record: LinkRecord) -> anyhow::Result<()>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
    //async fn purge_expired(&self, now: OffsetDateTime) -> anyhow::Result<usize>;

    async fn incr_hit(&self, id: &str) -> anyhow::Result<()>;
    async fn get_hits(&self, id: &str) -> anyhow::Result<u64>;
}
