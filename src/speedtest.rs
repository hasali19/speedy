use std::process::Command;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestResult {
    pub timestamp: String,
    pub ping: Ping,
    pub download: Download,
    pub upload: Upload,
    #[serde(rename = "packetLoss")]
    pub packet_loss: i32,
    pub isp: String,
    pub interface: Interface,
    pub server: Server,
    pub result: WebResult,
}

#[derive(Debug, Deserialize)]
pub struct Ping {
    pub jitter: f32,
    pub latency: f32,
}

#[derive(Debug, Deserialize)]
pub struct Download {
    pub bandwidth: i32,
    pub bytes: i32,
    pub elapsed: i32,
}

#[derive(Debug, Deserialize)]
pub struct Upload {
    pub bandwidth: i32,
    pub bytes: i32,
    pub elapsed: i32,
}

#[derive(Debug, Deserialize)]
pub struct Interface {
    #[serde(rename = "internalIp")]
    pub internal_ip: String,
    pub name: String,
    #[serde(rename = "macAddr")]
    pub mac_addr: String,
    #[serde(rename = "isVpn")]
    pub is_vpn: bool,
    #[serde(rename = "externalIp")]
    pub external_ip: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub country: String,
    pub host: String,
    pub port: i32,
    pub ip: String,
}

#[derive(Debug, Deserialize)]
pub struct WebResult {
    pub id: String,
    pub url: String,
}

pub struct Client {
    path: String,
}

impl Client {
    pub fn from_path(path: &str) -> Client {
        Client {
            path: path.to_owned(),
        }
    }

    pub fn run_test(&self) -> Result<TestResult> {
        let output = Command::new(&self.path).arg("--format=json").output()?;
        Ok(serde_json::from_slice(&output.stdout)?)
    }
}
