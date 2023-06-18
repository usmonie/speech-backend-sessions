use std::sync::Arc;

use async_trait::async_trait;
use speech_backend_common::ApiResult;
use speech_backend_common::domain::UseCase;
use tokio::sync::Mutex;

use crate::{data::SessionRepository};
use crate::models::request::{AddUserToSessionRequest, ClearSessionRequest, CreateSessionRequest, GetSessionRequest, UpdateSessionIpRequest};
use crate::models::results::Session;

pub struct CreateSessionUseCase {
    sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>,
}

#[async_trait]
impl UseCase<CreateSessionRequest, Session> for CreateSessionUseCase {
    async fn execute(&self, request: CreateSessionRequest) -> ApiResult<Session> {
        let session = self.sessions_repository
            .lock()
            .await
            .create_session(request.device, request.ip_addr, request.session_key)
            .await;

        return session;
    }
}

impl CreateSessionUseCase {
    pub fn new(sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>) -> Self {
        Self { sessions_repository }
    }
}

pub struct GetSessionUseCase {
    sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>,
}

#[async_trait]
impl UseCase<GetSessionRequest, Session> for GetSessionUseCase {
    async fn execute(&self, request: GetSessionRequest) -> ApiResult<Session> {
        let session = self.sessions_repository
            .lock()
            .await
            .get_session(request.id.to_string().as_str())
            .await;

        return session;
    }
}

pub struct AddUserToSessionUseCase {
    sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>,
}

#[async_trait]
impl UseCase<AddUserToSessionRequest, Session> for AddUserToSessionUseCase {
    async fn execute(&self, request: AddUserToSessionRequest) -> ApiResult<Session> {
        let session = self.sessions_repository
            .lock()
            .await
            .add_session_to_user(request.session_id, request.latest_ip_address, request.user_id, request.session_key, request.user_password_hash)
            .await;

        return session;
    }
}

impl AddUserToSessionUseCase {
    pub fn new(sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>) -> Self {
        Self { sessions_repository }
    }
}

pub struct UpdateSessionIpUseCase {
    sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>,
}

#[async_trait]
impl UseCase<UpdateSessionIpRequest, Session> for UpdateSessionIpUseCase {
    async fn execute(&self, request: UpdateSessionIpRequest) -> ApiResult<Session> {
        let session = self.sessions_repository
            .lock()
            .await
            .update_session_ip(request.session_id, request.latest_ip_address, request.session_key)
            .await;

        return session;
    }
}

impl UpdateSessionIpUseCase {
    pub fn new(sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>) -> Self {
        Self { sessions_repository }
    }
}

pub struct ClearSessionUseCase {
    sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>,
}

#[async_trait]
impl UseCase<ClearSessionRequest, ()> for ClearSessionUseCase {
    async fn execute(&self, request: ClearSessionRequest) -> ApiResult<()> {
        self.sessions_repository
            .lock()
            .await
            .clear_session(request.session_id, request.session_key);

        ApiResult::Ok(())
    }
}

impl ClearSessionUseCase {
    pub fn new(sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>) -> Self {
        Self { sessions_repository }
    }
}