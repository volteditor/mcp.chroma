use anyhow::Result;
use serde_json::json;
use std::sync::{Arc, Mutex, MutexGuard};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChromaClient {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
}

impl ChromaClient {
    pub fn new(
        host: &str,
        port: u16,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            username: username.map(|s| s.to_string()),
            password: password.map(|s| s.to_string()),
        }
    }

    pub fn list_collections(&self, _limit: Option<usize>, _offset: Option<usize>) -> Result<Vec<String>> {
        Ok(vec!["test_collection".to_string()])
    }

    pub fn create_collection(&self, name: &str, _metadata: Option<serde_json::Value>) -> Result<String> {
        Ok(format!("Created collection: {}", name))
    }

    pub fn get_collection(&self, name: &str) -> Result<Collection> {
        Ok(Collection {
            name: name.to_string(),
        })
    }

    pub fn delete_collection(&self, _name: &str) -> Result<()> {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Collection {
    pub name: String,
}

impl Collection {
    pub fn add(
        &self,
        _documents: Vec<String>,
        _metadatas: Option<Vec<serde_json::Value>>,
        _ids: Vec<String>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn query(
        &self,
        _query_texts: Vec<String>,
        _n_results: usize,
        _where_filter: Option<serde_json::Value>,
        _where_document: Option<serde_json::Value>,
        _include: Vec<String>,
    ) -> Result<serde_json::Value> {
        Ok(json!({
            "ids": [["doc1", "doc2"]],
            "documents": [["document1", "document2"]],
            "metadatas": [[{"source": "test1"}, {"source": "test2"}]],
            "distances": [[0.1, 0.2]],
        }))
    }

    pub fn get(
        &self,
        _ids: Option<Vec<String>>,
        _where_filter: Option<serde_json::Value>,
        _where_document: Option<serde_json::Value>,
        _include: Vec<String>,
        _limit: Option<usize>,
        _offset: Option<usize>,
    ) -> Result<serde_json::Value> {
        Ok(json!({
            "ids": ["doc1", "doc2"],
            "documents": ["document1", "document2"],
            "metadatas": [{"source": "test1"}, {"source": "test2"}]
        }))
    }

    pub fn update(
        &self,
        _ids: Vec<String>,
        _embeddings: Option<Vec<Vec<f32>>>,
        _metadatas: Option<Vec<serde_json::Value>>,
        _documents: Option<Vec<String>>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn delete(&self, _ids: Vec<String>) -> Result<()> {
        Ok(())
    }

    pub fn count(&self) -> Result<usize> {
        Ok(3)
    }

    pub fn peek(&self, _limit: usize) -> Result<serde_json::Value> {
        Ok(json!({
            "ids": ["doc1", "doc2"],
            "documents": ["document1", "document2"],
            "metadatas": [{"source": "test1"}, {"source": "test2"}]
        }))
    }

    pub fn modify(
        &self,
        _name: Option<String>,
        _metadata: Option<serde_json::Value>,
    ) -> Result<()> {
        Ok(())
    }
}

static CLIENT: Mutex<Option<ChromaClient>> = Mutex::new(None);

pub fn initialize_client() -> Result<()> {
    let host = std::env::var("CHROMA_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("CHROMA_PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .unwrap_or(8000);
    let username = std::env::var("CHROMA_USERNAME").ok();
    let password = std::env::var("CHROMA_PASSWORD").ok();

    let client = ChromaClient::new(
        &host, 
        port,
        username.as_deref(),
        password.as_deref(),
    );
    
    let mut global_client = CLIENT.lock().unwrap();
    *global_client = Some(client);
    
    Ok(())
}

pub fn get_client() -> Arc<ChromaClient> {
    let client_guard: MutexGuard<Option<ChromaClient>> = CLIENT.lock().unwrap();
    
    if client_guard.is_none() {
        drop(client_guard);
        initialize_client().expect("Failed to initialize client");
        return get_client();
    }
    
    Arc::new(client_guard.as_ref().unwrap().clone())
}
