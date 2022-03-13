use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use color_eyre::eyre::eyre;
use futures::{future::LocalBoxFuture, FutureExt};
use sqlx::Encode;

use crate::{server::MyError, session::SessionMap};

#[derive(Debug, Clone, Encode)]
pub struct UserIdPart(pub i64);

impl<'a> FromRequest for UserIdPart {
    type Error = MyError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        return async { Ok(UserIdPart(1)) }.boxed();
        let ssid = req.cookie("ssid");
        let value = web::Data::<SessionMap>::from_request(req, payload)
            .then(|session_map| async move {
                let session = ssid.ok_or_else(|| eyre!("Need to have a SSID cookie"))?;
                let session = session.value();
                let sessions_data = session_map.map_err(|_| eyre!("Could not extract sessions"))?;
                let mut sessions = sessions_data.lock().await;
                let user_id = sessions
                    .get(&session.to_string())
                    .ok_or_else(|| eyre!("No cookie in sessions"))?;

                Ok(Self(user_id.user_id()))
            })
            .map(|x| x.map_err(MyError::NotLoggedIn));
        Box::pin(value)
    }
}
