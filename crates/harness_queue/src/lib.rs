//! Queue module - High-performance queues for heliosHarness

use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Error types for queues
#[derive(Debug, Error)]
pub enum QueueError {
    #[error("Channel is closed")]
    Closed,

    #[error("Channel is full")]
    Full,

    #[error("Channel is empty")]
    Empty,

    #[error("Send error: {0}")]
    Send(String),

    #[error("Receive error: {0}")]
    Receive(String),
}

/// MPSC (Multiple Producer Single Consumer) channel
pub struct Channel<T> {
    buffer: Arc<Mutex<VecDeque<T>>>,
    capacity: usize,
    size: Arc<AtomicUsize>,
    closed: Arc<Mutex<bool>>,
}

impl<T> Channel<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            capacity,
            size: Arc::new(AtomicUsize::new(0)),
            closed: Arc::new(Mutex::new(false)),
        }
    }

    pub fn send(&self, item: T) -> Result<(), String> {
        let is_closed = {
            let closed = self.closed.lock().map_err(|e| e.to_string())?;
            *closed
        };
        if is_closed {
            return Err("Channel closed".into());
        }

        let mut buffer = self.buffer.lock().map_err(|e| e.to_string())?;
        if buffer.len() >= self.capacity {
            return Err("Channel full".into());
        }

        buffer.push_back(item);
        self.size.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn recv(&self) -> Option<T> {
        let mut buffer = self.buffer.lock().ok()?;
        if buffer.is_empty() {
            return None;
        }
        self.size.fetch_sub(1, Ordering::Relaxed);
        buffer.pop_front()
    }

    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn is_full(&self) -> bool {
        self.len() >= self.capacity
    }

    pub fn close(&self) {
        if let Ok(mut closed) = self.closed.lock() {
            *closed = true;
        }
    }
}

/// Ring buffer for single producer/consumer
#[allow(dead_code)]
pub struct RingBuffer<T> {
    data: Vec<T>,
    read: usize,
    write: usize,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            read: 0,
            write: 0,
            capacity,
        }
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.data.len() >= self.capacity {
            return false;
        }
        self.data.push(item);
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.read >= self.data.len() {
            return None;
        }
        let item = self.data.remove(self.read);
        self.read += 1;
        Some(item)
    }

    pub fn len(&self) -> usize {
        self.data.len().saturating_sub(self.read)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Work-stealing queue for parallel processing
pub struct WorkQueue<T> {
    local: Mutex<VecDeque<T>>,
    global: Arc<Mutex<VecDeque<T>>>,
}

impl<T> WorkQueue<T> {
    pub fn new() -> Self {
        Self {
            local: Mutex::new(VecDeque::new()),
            global: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn push(&self, item: T) {
        if let Ok(mut q) = self.local.lock() {
            q.push_back(item);
        }
    }

    pub fn pop(&self) -> Option<T> {
        if let Ok(mut q) = self.local.lock() {
            if let Some(item) = q.pop_front() {
                return Some(item);
            }
        }
        if let Ok(mut g) = self.global.lock() {
            return g.pop_back();
        }
        None
    }

    pub fn steal(&self) -> Option<T> {
        if let Ok(mut g) = self.global.lock() {
            return g.pop_back();
        }
        None
    }
}

impl<T> Default for WorkQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
