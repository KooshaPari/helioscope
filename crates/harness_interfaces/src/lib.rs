//! Interfaces module - Protocol definitions for heliosHarness

use std::collections::HashMap;

/// Request context
#[derive(Debug, Clone)]
pub struct Request {
    pub id: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(method: &str, path: &str) -> Self {
        Self {
            id: uuid_v4(),
            method: method.to_string(),
            path: path.to_string(),
            headers: HashMap::new(),
            body: None,
        }
    }
    
    pub fn with_header(mut self, key: &str, val: &str) -> Self {
        self.headers.insert(key.to_string(), val.to_string());
        self
    }
    
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }
}

/// Response
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Response {
    pub fn ok() -> Self { Self { status: 200, headers: HashMap::new(), body: None } }
    pub fn created() -> Self { Self { status: 201, headers: HashMap::new(), body: None } }
    pub fn error(status: u16) -> Self { Self { status, headers: HashMap::new(), body: None } }
    
    pub fn with_header(mut self, key: &str, val: &str) -> Self {
        self.headers.insert(key.to_string(), val.to_string());
        self
    }
    
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }
}

/// Event for pub/sub
#[derive(Debug, Clone)]
pub struct Event {
    pub topic: String,
    pub payload: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

impl Event {
    pub fn new(topic: &str, payload: Vec<u8>) -> Self {
        Self { topic: topic.to_string(), payload, metadata: HashMap::new() }
    }
}

/// Simple UUID v4 generator
fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:032x}-{:016x}", now.as_nanos(), now.as_secs())
}

/// Handler trait for request processing
pub trait Handler: Send + Sync {
    fn handle(&self, request: Request) -> Response;
}

/// Publisher trait for event systems
pub trait Publisher: Send + Sync {
    fn publish(&self, event: Event) -> Result<(), String>;
}

/// Subscriber trait for event systems
#[allow(async_fn_in_trait)]
pub trait Subscriber: Send + Sync {
    async fn on_event(&self, event: Event);
}
