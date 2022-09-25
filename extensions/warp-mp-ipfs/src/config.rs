use ipfs::Multiaddr;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Bootstrap {
    #[default]
    Ipfs,
    Experimental,
    Custom(Vec<Multiaddr>),
    None,
}

impl Bootstrap {
    pub fn address(&self) -> Vec<Multiaddr> {
        match self {
            Bootstrap::Ipfs => vec![
                "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
                "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
                "/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
                "/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
            ]
            .iter()
            .filter_map(|s| Multiaddr::from_str(s).ok())
            .collect::<Vec<_>>(),
            Bootstrap::Experimental => vec!["/ip4/67.205.175.147/tcp/5000/p2p/12D3KooWDC7igsZ9Yaheip77ejALmjG6AZm2auuVmMDj1AkC2o7B"]
            .iter()
            .filter_map(|s| Multiaddr::from_str(s).ok())
            .collect::<Vec<_>>(),
            Bootstrap::Custom(address) => address.clone(),
            Bootstrap::None => vec![]
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Mdns {
    pub enable: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RelayClient {
    pub enable: bool,
    pub dcutr: bool,
    pub single: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub relay_address: Vec<Multiaddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swarm {
    pub dial_factor: u8,
    pub notify_buffer_size: usize,
    pub connection_buffer_size: usize,
    pub limit: Option<ConnectionLimit>,
}

impl Default for Swarm {
    fn default() -> Self {
        Self {
            dial_factor: 8, //Same dial factor as default for libp2p
            notify_buffer_size: 32,
            connection_buffer_size: 1024,
            limit: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionLimit {
    pub max_pending_incoming: Option<u32>,
    pub max_pending_outgoing: Option<u32>,
    pub max_established_incoming: Option<u32>,
    pub max_established_outgoing: Option<u32>,
    pub max_established: Option<u32>,
    pub max_established_per_peer: Option<u32>,
}

impl Default for ConnectionLimit {
    fn default() -> Self {
        Self {
            max_pending_incoming: None,
            max_pending_outgoing: Some(1024),
            max_established_incoming: None,
            max_established_outgoing: Some(10000),
            max_established: None,
            max_established_per_peer: None,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RelayServer {
    pub enable: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IpfsSetting {
    pub mdns: Mdns,
    pub relay_client: RelayClient,
    pub relay_server: RelayServer,
    pub swarm: Swarm,
    pub bootstrap: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StoreSetting {
    pub broadcast_interval: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_name: Option<String>,
    pub discovery: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sync: Vec<Multiaddr>,
    pub sync_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpIpfsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    pub bootstrap: Bootstrap,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub listen_on: Vec<Multiaddr>,
    pub ipfs_setting: IpfsSetting,
    pub store_setting: StoreSetting,
    pub debug: bool,
}

impl Default for MpIpfsConfig {
    fn default() -> Self {
        MpIpfsConfig {
            path: None,
            bootstrap: Bootstrap::Ipfs,
            listen_on: vec!["/ip4/0.0.0.0/tcp/0", "/ip6/::/tcp/0"]
                .iter()
                .filter_map(|s| Multiaddr::from_str(s).ok())
                .collect::<Vec<_>>(),
            ipfs_setting: IpfsSetting {
                mdns: Mdns { enable: true },
                bootstrap: false,
                ..Default::default()
            },
            store_setting: StoreSetting {
                broadcast_interval: 100,
                ..Default::default()
            },
            debug: false,
        }
    }
}

impl MpIpfsConfig {
    pub fn development() -> MpIpfsConfig {
        MpIpfsConfig {
            path: None,
            ipfs_setting: IpfsSetting {
                mdns: Mdns { enable: true },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn testing(experimental: bool) -> MpIpfsConfig {
        MpIpfsConfig {
            bootstrap: match experimental {
                true => Bootstrap::Experimental,
                false => Bootstrap::Ipfs,
            },
            ipfs_setting: IpfsSetting {
                bootstrap: true,
                mdns: Mdns { enable: true },
                relay_client: RelayClient {
                    enable: true,
                    dcutr: true,
                    single: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            store_setting: StoreSetting {
                discovery: true,
                broadcast_interval: 100,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn production<P: AsRef<std::path::Path>>(path: P, experimental: bool) -> MpIpfsConfig {
        MpIpfsConfig {
            bootstrap: match experimental {
                true => Bootstrap::Experimental,
                false => Bootstrap::Ipfs,
            },
            path: Some(path.as_ref().to_path_buf()),
            ipfs_setting: IpfsSetting {
                mdns: Mdns { enable: true },
                bootstrap: true,
                relay_client: RelayClient {
                    enable: true,
                    dcutr: true,
                    single: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            store_setting: StoreSetting {
                broadcast_interval: 100,
                discovery: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<MpIpfsConfig> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;
        let config = serde_json::from_slice(&bytes)?;
        Ok(config)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path = path.as_ref();
        let bytes = serde_json::to_vec(self)?;
        std::fs::write(path, bytes)?;
        Ok(())
    }

    pub fn from_string<S: AsRef<str>>(data: S) -> anyhow::Result<MpIpfsConfig> {
        let data = data.as_ref();
        let config = serde_json::from_str(data)?;
        Ok(config)
    }
}

pub mod ffi {
    use crate::MpIpfsConfig;
    use std::ffi::CStr;
    use std::os::raw::c_char;
    use warp::error::Error;
    use warp::ffi::{FFIResult, FFIResult_Null};

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn mp_ipfs_config_from_file(
        file: *const c_char,
    ) -> FFIResult<MpIpfsConfig> {
        if file.is_null() {
            return FFIResult::err(Error::Any(anyhow::anyhow!("file cannot be null")));
        }

        let file = CStr::from_ptr(file).to_string_lossy().to_string();

        MpIpfsConfig::from_file(file).map_err(Error::from).into()
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn mp_ipfs_config_to_file(
        config: *const MpIpfsConfig,
        file: *const c_char,
    ) -> FFIResult_Null {
        if config.is_null() {
            return FFIResult_Null::err(Error::NullPointerContext {
                pointer: "config".into(),
            });
        }

        if file.is_null() {
            return FFIResult_Null::err(Error::NullPointerContext {
                pointer: "file".into(),
            });
        }

        let file = CStr::from_ptr(file).to_string_lossy().to_string();

        MpIpfsConfig::to_file(&*config, file)
            .map_err(Error::from)
            .into()
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn mp_ipfs_config_from_str(
        config: *const c_char,
    ) -> FFIResult<MpIpfsConfig> {
        if config.is_null() {
            return FFIResult::err(Error::Any(anyhow::anyhow!("config cannot be null")));
        }

        let data = CStr::from_ptr(config).to_string_lossy().to_string();

        MpIpfsConfig::from_string(data).map_err(Error::from).into()
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn mp_ipfs_config_development() -> *mut MpIpfsConfig {
        Box::into_raw(Box::new(MpIpfsConfig::development()))
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn mp_ipfs_config_testing(experimental: bool) -> *mut MpIpfsConfig {
        Box::into_raw(Box::new(MpIpfsConfig::testing(experimental)))
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn mp_ipfs_config_production(
        path: *const c_char,
        experimental: bool,
    ) -> FFIResult<MpIpfsConfig> {
        if path.is_null() {
            return FFIResult::err(Error::Any(anyhow::anyhow!("config cannot be null")));
        }

        let path = CStr::from_ptr(path).to_string_lossy().to_string();

        FFIResult::ok(MpIpfsConfig::production(path, experimental))
    }
}
