use crate::RespFrame;
use dashmap::DashMap;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Backend(Arc<BackendInner>);

#[derive(Debug, Clone)]
pub struct BackendInner {
    map: DashMap<String, RespFrame>,
    hmap: DashMap<String, DashMap<String, RespFrame>>,
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Backend {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self(Arc::new(BackendInner::default()))
    }
}

impl Default for BackendInner {
    fn default() -> Self {
        Self::new()
    }
}

impl BackendInner {
    pub fn new() -> Self {
        Self {
            map: DashMap::new(),
            hmap: DashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: RespFrame) {
        self.map.insert(key, value);
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<RespFrame> {
        self.hmap
            .get(key)
            .and_then(|v| v.get(field).map(|v| v.value().clone()))
    }

    pub fn hset(&self, key: String, field: String, value: RespFrame) {
        let hmap = self.hmap.entry(key).or_default();
        hmap.insert(field, value);
    }

    pub fn hget_all(&self, key: &str) -> Option<DashMap<String, RespFrame>> {
        self.hmap.get(key).map(|v| v.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BulkString;
    use anyhow::Result;

    #[test]
    fn test_backend_map() -> Result<()> {
        let backend = Backend::new();

        // test set key
        backend.set("hello".into(), BulkString::new("world").into());

        // test exist key
        let v = backend.get("hello");
        assert_eq!(v, Some(BulkString::new(b"world").into()));

        // test not exist key
        let v_not_exist = backend.get("not_exist_key");
        assert_eq!(v_not_exist, None);
        Ok(())
    }

    #[test]
    fn test_backend_hmap() -> Result<()> {
        let backend = Backend::new();
        backend.hset(
            "key".into(),
            "field".into(),
            BulkString::new("value").into(),
        );

        let value = backend.hget("key", "field");
        assert_eq!(value, Some(BulkString::new("value").into()));

        let value = backend.hget("key", "not_exist_field");
        assert_eq!(value, None);
        Ok(())
    }
}
