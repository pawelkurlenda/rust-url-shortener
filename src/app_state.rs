use std::sync::Arc;

use axum::extract::FromRef;

use crate::{cuckoo_filter::CuckooFilter, store::store::Store};

//#[derive(Clone)]
pub struct AppState<S: Store> {
    pub store: Arc<S>,
    pub cuckoo_filter: Arc<CuckooFilter>,
}

impl<S: Store> Clone for AppState<S> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            cuckoo_filter: self.cuckoo_filter.clone(),
        }
    }
} //todo derive clone not working?

impl<S: Store> FromRef<AppState<S>> for Arc<S> {
    fn from_ref(state: &AppState<S>) -> Self {
        state.store.clone()
    }
}

impl<S: Store> FromRef<AppState<S>> for Arc<dyn Store> {
    fn from_ref(state: &AppState<S>) -> Self {
        state.store.clone() as Arc<dyn Store>
    }
}

impl<S: Store> FromRef<AppState<S>> for Arc<CuckooFilter> {
    fn from_ref(state: &AppState<S>) -> Self {
        state.cuckoo_filter.clone()
    }
}
