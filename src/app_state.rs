use std::sync::Arc;

use crate::store::store::Store;

pub struct AppState<S: Store> {
    pub store: Arc<S>,
    //pub cuckoo_filter:
}

impl<S: Store> Clone for AppState<S> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}
