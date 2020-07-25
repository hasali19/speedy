use anyhow::Result;
use serde::Deserialize;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
pub struct TestResult {
    pub timestamp: String,
    pub ping: Ping,
    pub download: Download,
    pub upload: Upload,
    #[serde(rename = "packetLoss")]
    pub packet_loss: Option<i32>,
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

pub struct TestClient {
    options: TestClientBuilder,
}

pub struct TestClientBuilder {
    path: String,
    accept_license: bool,
    accept_gdpr: bool,
}

impl TestClientBuilder {
    pub fn with_path(path: &str) -> Self {
        TestClientBuilder {
            path: path.to_owned(),
            accept_license: false,
            accept_gdpr: false,
        }
    }

    pub fn accept_license(mut self) -> Self {
        self.accept_license = true;
        self
    }

    pub fn accept_gdpr(mut self) -> Self {
        self.accept_gdpr = true;
        self
    }

    pub fn build_client(self) -> TestClient {
        TestClient::new(self)
    }
}

impl TestClient {
    pub fn new(options: TestClientBuilder) -> TestClient {
        TestClient { options }
    }

    pub async fn run_test(&self) -> Result<TestResult> {
        let mut command = Command::new(&self.options.path);

        command.arg("--format=json");

        if self.options.accept_license {
            command.arg("--accept-license");
        }

        if self.options.accept_gdpr {
            command.arg("--accept-gdpr");
        }

        let output = command.output().await.unwrap();

        if log::log_enabled!(log::Level::Trace) {
            log::trace!(
                "test output: {}",
                String::from_utf8(output.stdout.clone()).unwrap()
            );
        }

        Ok(serde_json::from_slice(&output.stdout)?)
    }
}
