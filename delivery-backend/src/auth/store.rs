use std::sync::Arc;

use super::jwt::RefreshToken;
use dashmap::{mapref::one::Ref, DashMap};

/// A simple in-memory store for holding [`RefreshToken`]s
#[derive(Debug, Default)]
pub struct Store {
    inner: DashMap<String, RefreshToken>
}

impl Store {
    /// Get a [`RefreshToken`] by `key` from the [`Store`]
    pub fn get(&self, key: &str) -> Option<Ref<String, RefreshToken>> {
        self.inner.get(key)
    }

    /// Insert a [`RefreshToken`] into the [`Store`]
    pub fn insert(&self, key: String, value: RefreshToken) -> Option<RefreshToken> {
        self.inner.insert(key, value)
    }

    /// Removes a [`RefreshToken`] from the map, returning the key and value if they existed in the map.
    pub fn remove(&self, key: &str) -> Option<(String, RefreshToken)> {
        self.inner.remove(key)
    }
}

/// Initialize the in-memory store
pub fn setup_store() -> Arc<Store> {
    Arc::new(Store::default())
}
