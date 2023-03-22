use std::net::IpAddr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Session {
    pub id: String,
    pub device: Device,
    pub ip_address: IpAddr,
    pub session_key: Vec<u8>,
    pub user_id: Option<String>,
}

unsafe impl Send for Session {}
unsafe impl Sync for Session {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Device {
    PC { name: String },
    Mac { os_version: String, name: String },
    IPhone { os_version: String, iphone: String },
    Android { os_version: String, name: String },
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}