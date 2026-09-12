pub mod user_repositories {
    use sqlx::{PgPool, query_as};
    use crate::models::user::{CreateUserRequest, UserModel};
    use crate::errors::ApiError;

    pub struct UserRepository {
        pool: PgPool,
    }

    impl UserRepository {
        pub fn new(pool: PgPool) -> Self {
            Self { pool }
        }

        pub async fn insert(
            &self,
            data: &CreateUserRequest,
        ) -> Result<UserModel, ApiError> {
            log::debug!("Repository: Inserting user with email: {}", data.email);

            let row = query_as::<_, UserModel>(
                r#"
                INSERT INTO users (email, name)
                VALUES ($1, $2)
                RETURNING id, email, name
                "#,
            )
            .bind(&data.email)
            .bind(&data.name)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Repository: Database error on insert: {:?}", e);
                ApiError::from(e)
            })?;

            log::debug!("Repository: User inserted successfully with id: {}", row.id);
            Ok(row)
        }

        pub async fn get_all(&self) -> Result<Vec<UserModel>, ApiError> {
            log::debug!("Repository: Fetching all users");
            
            let rows = query_as::<_, UserModel>(
                "SELECT id, email, name FROM users"
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Repository: Database error on fetch_all: {:?}", e);
                ApiError::from(e)
            })?;

            log::debug!("Repository: Fetched {} users", rows.len());
            Ok(rows)
        }
    }
}
