use std::{sync::Arc, net::IpAddr};

use async_trait::async_trait;
use speech_backend_common::domain::UseCase;
use speech_backend_common::result::ApiResult;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{data::SessionRepository, models::{Session, Device}};

pub struct GetSessionUseCase {
    sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>,
}

/// Get session request from repository by session id
/// * `id` — session id
/// returns `Session`
pub struct GetSessionRequest {
    id: String,
}

pub struct CreateSessionRequest {
    device: Device,
    ip_addr: IpAddr,
    session_key: Vec<u8>,
}

#[async_trait]
impl UseCase<GetSessionRequest, Session> for GetSessionUseCase {
    async fn execute(&self, request: GetSessionRequest) -> ApiResult<Session> {
        let session = self.sessions_repository
            .lock()
            .await
            .get_session(request.id.as_str())
            .await;

        return session;
    }
}

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
    fn new(sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>) -> Self {
        Self { sessions_repository }
    }
}

pub struct AddUserToSessionUseCase {
    sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>,
}

pub struct AddUserToSessionRequest {
    session_id: Uuid,
    latest_ip_address: IpAddr,
    user_id: Uuid,
    session_key: Vec<u8>,
    user_password_hash: Vec<u8>,
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
    fn new(sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>) -> Self {
        Self { sessions_repository }
    }
}

pub struct UpdateSessionIpUseCase {
    sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>,
}

pub struct UpdateSessionIpRequest {
    session_id: Uuid,
    latest_ip_address: IpAddr,
    session_key: Vec<u8>,
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
    fn new(sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>) -> Self {
        Self { sessions_repository }
    }
}

pub struct ClearSessionUseCase {
    sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>,
}

pub struct ClearSessionRequest {
    session_id: Uuid,
    session_key: Vec<u8>,
}

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
    fn new(sessions_repository: Arc<Mutex<dyn SessionRepository + Send + Sync>>) -> Self {
        Self { sessions_repository }
    }
}