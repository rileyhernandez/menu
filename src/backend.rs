use crate::device::Device;
use crate::error::Error;
use reqwest;

pub struct CalibrationBackend {
    path: String,
}
impl CalibrationBackend {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn get_config(&self, phidget_id: isize) -> Result<String, Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/{}", self.path, phidget_id);
        response_from_client(client, url)
    }
}

pub struct ConfigBackend {
    path: String,
}
impl ConfigBackend {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn get_config(&self, device: Device) -> Result<String, Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/{}", self.path, device);
        response_from_client(client, url)
    }
}

fn response_from_client(client: reqwest::blocking::Client, url: String) -> Result<String, Error> {
    client
        .get(url)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .map_err(Error::Reqwest)?
        .text()
        .map_err(Error::Reqwest)
}
