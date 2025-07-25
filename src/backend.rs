use crate::device::{Device, Model};
use crate::error::Error;
use crate::libra::Config;
use reqwest;
use reqwest::StatusCode;

pub const CONFIG_BACKEND_URL: &str =
    "https://us-west1-back-of-house-backend.cloudfunctions.net/mise/";
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
#[derive(Clone)]
pub struct ConfigBackend {
    path: String,
    auth_token: String,
}
impl ConfigBackend {
    pub fn new(path: String, auth_token: String) -> Self {
        Self { path, auth_token }
    }
    pub fn make_new_device(&self, model: Model, config: Config) -> Result<Device, Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/{:?}", self.path, model);
        let response = client
            .post(url)
            .bearer_auth(&self.auth_token)
            .json(&config)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .map_err(Error::Reqwest)?;
        if response.status() == StatusCode::CREATED {
            let device: Device = response.json().map_err(Error::Reqwest)?;
            Ok(device)
        } else {
            Err(Error::Backend(response.status()))
        }
    }
    pub fn get_config(&self, device: Device) -> Result<Config, Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/{:?}/{}", self.path, device.model, device.number);
        let response = client
            .get(url)
            .bearer_auth(&self.auth_token)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .map_err(Error::Reqwest)?;
        if response.status() == StatusCode::OK {
            let config: Config = response.json().map_err(Error::Reqwest)?;
            Ok(config)
        } else {
            Err(Error::Backend(response.status()))
        }
    }
    pub fn edit_config(&self, device: Device, config: Config) -> Result<(), Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/{:?}/{}", self.path, device.model, device.number);
        let response = client
            .put(url)
            .bearer_auth(&self.auth_token)
            .json(&config)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .map_err(Error::Reqwest)?;
        if response.status() == StatusCode::OK {
            Ok(())
        } else {
            Err(Error::Backend(response.status()))
        }
    }
    pub async fn make_new_device_async(
        &self,
        model: Model,
        config: Config,
    ) -> Result<Device, Error> {
        let client = reqwest::Client::new();
        let url = format!("{}/{:?}", self.path, model);
        let response = client
            .post(url)
            .bearer_auth(&self.auth_token)
            .json(&config)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await
            .map_err(Error::Reqwest)?;
        if response.status() == StatusCode::CREATED {
            let device: Device = response.json().await.map_err(Error::Reqwest)?;
            Ok(device)
        } else {
            Err(Error::Backend(response.status()))
        }
    }
    pub async fn get_config_async(&self, device: Device) -> Result<Config, Error> {
        let client = reqwest::Client::new();
        let url = format!("{}/{:?}/{}", self.path, device.model, device.number);
        let response = client
            .get(url)
            .bearer_auth(&self.auth_token)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await
            .map_err(Error::Reqwest)?;
        if response.status() == StatusCode::OK {
            let config: Config = response.json().await.map_err(Error::Reqwest)?;
            Ok(config)
        } else {
            Err(Error::Backend(response.status()))
        }
    }
    pub async fn edit_config_async(&self, device: Device, config: Config) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let url = format!("{}/{:?}/{}", self.path, device.model, device.number);
        let response = client
            .put(url)
            .bearer_auth(&self.auth_token)
            .json(&config)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await
            .map_err(Error::Reqwest)?;
        if response.status() == StatusCode::OK {
            Ok(())
        } else {
            Err(Error::Backend(response.status()))
        }
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
