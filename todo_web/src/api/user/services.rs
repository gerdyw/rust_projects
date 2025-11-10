use sqlx::Error;
use tower_sessions::Session;
use uuid::Uuid;

use crate::{
    api::user::{models::User, repository::UserRepository},
    domain::errors::ServiceError,
};

const USER_KEY: &str = "user";

#[derive(Clone)]
pub struct UserService {
    pub repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn get_email_by_id(&self, user_id: &Uuid) -> Result<Option<String>, Error> {
        let user = self.repo.find_user_by_id(user_id).await?;
        Ok(user.map(|u| u.email))
    }

    pub async fn sign_in(
        &self,
        session: &mut Session,
        email: &String,
    ) -> Result<Option<User>, ServiceError> {
        let user = self.repo.find_user_by_email(email).await?;

        match user {
            Some(user) => {
                session.insert(USER_KEY, user.meta.id).await?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    pub async fn sign_up(
        &self,
        session: &mut Session,
        email: &String,
    ) -> Result<User, ServiceError> {
        let user = self.repo.create_user(email).await?;
        session.insert(USER_KEY, user.meta.id).await?;
        Ok(user)
    }
}
