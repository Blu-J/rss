use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::LocalBoxFuture;

use crate::server::MyError;

pub const USER_PREFERENCE: &str = "user_preferences";

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum FilterItems {
    All,
    Id(i64),
    Title(String),
}
impl FilterItems {
    pub fn as_items(&self) -> (Option<i64>, Option<String>) {
        let id = match self {
            FilterItems::Id(id) => Some(*id),
            _ => None,
        };
        let title = match self {
            FilterItems::Title(id) => Some(id.clone()),
            _ => None,
        };
        (id, title)
    }
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum ShowUnreads {
    ShowEverything,
    ShowUnreads,
}
impl ShowUnreads {
    pub fn query_value(&self) -> Option<bool> {
        match self {
            ShowUnreads::ShowEverything => None,
            ShowUnreads::ShowUnreads => Some(true),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UserPreferences {
    pub filter_items: FilterItems,
    pub sidebar_collapsed: bool,
    pub show_unreads: ShowUnreads,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            filter_items: FilterItems::All,
            sidebar_collapsed: false,
            show_unreads: ShowUnreads::ShowUnreads,
        }
    }
}

impl<'a> FromRequest for UserPreferences {
    type Error = MyError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let value: UserPreferences = req
            .cookie(USER_PREFERENCE)
            .and_then(|x| serde_json::from_str(x.value()).ok())
            .unwrap_or_default();
        Box::pin(async { Ok(value) })
    }
}
