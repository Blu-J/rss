use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use color_eyre::eyre::eyre;
use futures::{future::LocalBoxFuture, FutureExt};

use crate::{dto, server::MyError, session::SessionMap};

#[derive(Debug, Clone)]
pub struct UserIdPart(pub dto::UserId);

impl<'a> FromRequest for UserIdPart {
    type Error = MyError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let ssid = req.cookie("ssid");
        let value = web::Data::<SessionMap>::from_request(req, payload)
            .then(|session_map| async move {
                let session = ssid.ok_or_else(|| eyre!("Need to have a SSID cookie"))?;
                let session = session.value();
                let sessions_data = session_map.map_err(|_| eyre!("Could not extract sessions"))?;
                let sessions = sessions_data.lock().await;
                let user_id = sessions
                    .get(&session.to_string())
                    .ok_or_else(|| eyre!("No cookie in sessions"))?;

                Ok(Self(user_id.user_id().clone()))
            })
            .map(|x| x.map_err(MyError::NotLoggedIn));
        Box::pin(value)
    }
}
