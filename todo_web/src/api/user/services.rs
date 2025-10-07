use tower_sessions::Session;
use tracing::error;
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

    pub async fn get_email_by_id(&self, user_id: Uuid) -> Result<String, ServiceError> {
        let user = self.repo.find_user_by_id(user_id).await?;
        Ok(user.email)
    }

    pub async fn sign_in(
        &self,
        session: &mut Session,
        email: &String,
    ) -> Result<User, ServiceError> {
        let user = self
            .repo
            .find_user_by_email(email)
            .await
            .inspect_err(|e| error!("Error signing in user: {:#?}", e))?;

        session.insert(USER_KEY, user.meta.id).await?;
        session.save().await?;
        Ok(user)
    }

    pub async fn sign_up(
        &self,
        session: &mut Session,
        email: &String,
    ) -> Result<User, ServiceError> {
        let user = self.repo.create_user(email).await?;
        session.insert(USER_KEY, user.meta.id).await?;
        session.save().await?;
        Ok(user)
    }

    pub async fn sign_out(&self, session: &mut Session) -> Result<(), ServiceError> {
        session.delete().await?;
        session.save().await?;
        Ok(())
    }

    pub async fn sign_up_or_sign_in(
        &self,
        mut session: &mut Session,
        email: &String,
    ) -> Result<User, ServiceError> {
        let existing_sign_in = self.sign_in(&mut session, email).await;

        if let Ok(user) = existing_sign_in {
            Ok(user)
        } else {
            let user = self.sign_up(&mut session, email).await?;
            Ok(user)
        }
    }
}
