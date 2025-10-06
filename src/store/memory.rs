use crate::{models::LinkRecord, store::store::Store};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct MemoryStore {
    links: RwLock<HashMap<String, LinkRecord>>,
    hits: RwLock<HashMap<String, u64>>,
}

#[async_trait]
impl Store for MemoryStore {
    async fn get(&self, id: &str) -> anyhow::Result<Option<LinkRecord>> {
        Ok(self.links.read().await.get(id).cloned())
    }

    async fn put(&self, record: LinkRecord) -> anyhow::Result<()> {
        self.links.write().await.insert(record.id.clone(), record);
        Ok(())
    }

    async fn delete(&self, id: &str) -> anyhow::Result<bool> {
        Ok(self.links.write().await.remove(id).is_some())
    }

    async fn incr_hit(&self, id: &str) -> anyhow::Result<()> {
        if let Some(r) = self.links.write().await.get_mut(id) {
            r.hits += 1;
        }
        Ok(())
    }

    async fn incr_hit(&self, id: &str) -> anyhow::Result<u64> {
        if let Some(r) = self.map.write().await.get_mut(id) {
            r.hits += 1;
        }
        Ok(0)
    }

    // async fn purge_expired(&self, now: OffsetDateTime) -> anyhow::Result<usize> {
    //     let mut w = self.map.write();
    //     let before = w.len();
    //     w.retain(|_, v| !v.is_expired(now));
    //     Ok(before - w.len())
    // }
}
