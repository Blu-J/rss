use std::{collections::HashMap, sync::Arc, time::SystemTime};

use futures::lock::Mutex;

use crate::dto::UserId;

pub type SessionMap = Arc<Mutex<HashMap<String, Session>>>;

#[derive(Debug, Clone)]
pub struct Session {
    user_id: UserId,
    date: SystemTime,
}

impl Session {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            date: SystemTime::now(),
        }
    }

    /// Get a reference to the session's user id.
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    /// Get a reference to the session's date.
    pub fn date(&self) -> &SystemTime {
        &self.date
    }
}
