use super::Store;
use crate::models::LinkRecord;
use async_trait::async_trait;
use parking_lot::RwLock;
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Default)]
pub struct MemoryStore {
    map: RwLock<HashMap<String, LinkRecord>>,
}

#[async_trait]
impl Store for MemoryStore {
    async fn get(&self, id: &str) -> anyhow::Result<Option<LinkRecord>> {
        Ok(self.map.read().get(id).cloned())
    }
    async fn put(&self, record: LinkRecord) -> anyhow::Result<()> {
        self.map.write().insert(record.id.clone(), record);
        Ok(())
    }
    async fn delete(&self, id: &str) -> anyhow::Result<bool> {
        Ok(self.map.write().remove(id).is_some())
    }
    async fn incr_hits(&self, id: &str) -> anyhow::Result<()> {
        if let Some(r) = self.map.write().get_mut(id) {
            r.hits += 1;
        }
        Ok(())
    }
    async fn purge_expired(&self, now: OffsetDateTime) -> anyhow::Result<usize> {
        let mut w = self.map.write();
        let before = w.len();
        w.retain(|_, v| !v.is_expired(now));
        Ok(before - w.len())
    }
}
