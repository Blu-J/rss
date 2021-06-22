use color_eyre::eyre::bail;
use color_eyre::Result;
use sqlx::{query, Executor, Sqlite};
use tiny_keccak::Hasher;
use tiny_keccak::Sha3;
use tracing::instrument;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub i64);

impl UserId {
    #[instrument(skip(executor, password))]
    pub async fn fetch<'a>(
        executor: impl Executor<'a, Database = Sqlite>,
        user: &str,
        password: &str,
    ) -> Result<Self> {
        let record = query!(
            r#"SELECT id as 'id:UserId', salt, salted_password 
            FROM users
            WHERE username = $1"#,
            user,
        )
        .fetch_one(executor)
        .await?;

        let mut sha3 = Sha3::v256();
        let mut output = [0u8; 32];

        sha3.update(record.salt.as_bytes());
        sha3.update(password.as_bytes());
        sha3.finalize(&mut output);

        let calculated_salted_password = hex::encode(output);

        if calculated_salted_password != record.salted_password {
            bail!("Invalid password");
        }
        Ok(record.id)
    }
}
