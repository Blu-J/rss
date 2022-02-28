use std::{sync::Arc, time::SystemTime};

use futures::lock::Mutex;
use lru_time_cache::LruCache;

// use crate::dto::UserId;

pub type SessionMap = Arc<Mutex<LruCache<String, Session>>>;

#[derive(Debug, Clone)]
pub struct Session {
    user_id: String,
    date: SystemTime,
}

impl Session {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            date: SystemTime::now(),
        }
    }

    /// Get a reference to the session's user id.
    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    /// Get a reference to the session's date.
    pub fn date(&self) -> &SystemTime {
        &self.date
    }
}
