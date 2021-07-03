use color_eyre::Result;
use sqlx::{Executor, Sqlite,  query_as};
use sha3::{Sha3_256, Digest};
use tracing::instrument;

use super::UserId;

#[derive(Debug, Clone)]
pub struct User{
    id: UserId,
    salt: String,
    username: String,
    salted_password: String,
}

impl std::ops::Deref for User {
    type Target = UserId;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl User {
    #[instrument(skip(executor))]
    pub async fn fetch<'a>(
        executor: impl Executor<'a, Database = Sqlite>,
        username: &str,
    ) -> Result<Self> {
        let record = query_as!(Self,
            r#"SELECT id as 'id:UserId', salt, salted_password, username
            FROM users
            WHERE username = $1"#,
            username,
        )
        .fetch_one(executor)
        .await?;
        Ok(record)
    }

    pub fn username(&self) -> &str{
        &self.username
    }

    pub fn passwords_match(&self, password: &str) -> bool {

        let mut hasher = Sha3_256::new();

        // write input message
        hasher.update(format!("{}{}", self.salt, password).as_bytes());

        // read hash digest
        let result = hasher.finalize();
        
        &self.salted_password == &hex::encode(result.as_slice())
    }

}
