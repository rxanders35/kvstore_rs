use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hash;
use tokio::sync::Mutex;

enum OperationError {
    /*
    TODO:
    - Implement different error types
     */ 
    KeyAlreadyExists, 
    KeyNotFound,
}

struct KVDataStructure {
    data: Arc<Mutex<HashMap<K, V>>>,
}

impl<K, V> KVDataStructure<K, V> where
    K: Eq + Hash + Clone,
    V: Clone,
{

    pub fn new() -> Self {
        Self { data: Arc::new(Mutex::new(HashMap::new())), }
    }

    pub async fn put(&self, key: K, value: V) {
        let mut map = self.data.lock().await; 
        map.insert(key, value);
    }

    pub async fn get(&self, key: K) -> Option<V> {
        let map = self.data.lock().await;
        map.get(&key).cloned()
    }

    pub async fn delete(&self, key: K) -> Result<(), /* some error */> {
        let mut map = self.data.lock().await;
    }
}
