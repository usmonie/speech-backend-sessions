use std::{net::IpAddr, sync::Arc};
use rsmgclient::Connection;
use sled::Db;
use tokio::sync::Mutex;
use uuid::Uuid;
use async_trait::async_trait;
use speech_backend_common::{result::ApiResult, data::DataHolder};
use crate::models::{Device, Session};

#[async_trait]
pub(crate) trait SessionRepository {
    async fn  create_session(
        &mut self,
        device: Device,
        ip_addr: IpAddr, 
        session_key: Vec<u8>
    ) -> ApiResult<Session>;

    async fn add_session_to_user(
        &mut self,
        session_id: Uuid,
        latest_ip_address: IpAddr,
        user_id: Uuid,
        session_key: Vec<u8>,
        user_password_hash: Vec<u8>,
    ) -> ApiResult<Session>;

    async fn update_session_ip(
        &mut self,
        session_id: Uuid,
        latest_ip_address: IpAddr,
        session_key: Vec<u8>,
    ) -> ApiResult<Session>;

    async fn get_session(&self, id: &str) -> ApiResult<Session>;

    fn clear_session(
        &mut self,
        session_id: Uuid,
        session_key: Vec<u8>,
    );
}

#[async_trait]
impl SessionRepository for DataHolder {
    async fn create_session(&mut self, device: Device, ip_addr: IpAddr, session_key: Vec<u8>) -> ApiResult<Session> {
        todo!()
    }

    async fn add_session_to_user(&mut self, session_id: Uuid, latest_ip_address: IpAddr, user_id: Uuid, session_key: Vec<u8>, user_password_hash: Vec<u8>) -> ApiResult<Session> {
        todo!()
    }

    async fn update_session_ip(&mut self, session_id: Uuid, latest_ip_address: IpAddr, session_key: Vec<u8>) -> ApiResult<Session> {
        todo!()
    }

    async fn get_session(&mut self, id: &str) -> ApiResult<Session> {
        todo!()
    }

    fn clear_session(&self, session_id: Uuid, latest_ip_address: IpAddr, session_key: Vec<u8>) {
        todo!()
    }
}