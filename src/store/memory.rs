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

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.links.write().await.remove(id).is_some();
        Ok(())
    }

    async fn incr_hit(&self, id: &str) -> anyhow::Result<()> {
        if let Some(h) = self.hits.write().await.get_mut(id) {
            *h += 1;
        } else {
            self.hits.write().await.insert(id.to_string(), 1);
        }
        Ok(())
    }

    async fn get_hits(&self, id: &str) -> anyhow::Result<u64> {
        Ok(*self.hits.read().await.get(id).unwrap_or(&0))
    }

    // async fn purge_expired(&self, now: OffsetDateTime) -> anyhow::Result<usize> {
    //     let mut w = self.map.write();
    //     let before = w.len();
    //     w.retain(|_, v| !v.is_expired(now));
    //     Ok(before - w.len())
    // }
}
