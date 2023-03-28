pub mod results {
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
}

pub mod request {
    use std::net::IpAddr;
    use uuid::Uuid;
    use crate::models::results::Device;

    pub struct GetSessionRequest {
        pub id: String,
    }

    pub struct CreateSessionRequest {
        pub device: Device,
        pub ip_addr: IpAddr,
        pub session_key: Vec<u8>,
    }


    pub struct UpdateSessionIpRequest {
        pub session_id: Uuid,
        pub latest_ip_address: IpAddr,
        pub session_key: Vec<u8>,
    }

    pub struct AddUserToSessionRequest {
        pub session_id: Uuid,
        pub latest_ip_address: IpAddr,
        pub user_id: Uuid,
        pub session_key: Vec<u8>,
        pub user_password_hash: Vec<u8>,
    }

    pub struct ClearSessionRequest {
        pub session_id: Uuid,
        pub session_key: Vec<u8>,
    }
}