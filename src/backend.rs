pub const BACKEND_URL: &str =
    "https://us-west1-back-of-house-backend.cloudfunctions.net/mise/";
// "http://localhost:8080/";
#[derive(Clone)]
pub struct ConfigBackend {
    path: String,
    auth_token: String,
}
impl ConfigBackend {
    pub fn new(path: String, auth_token: String) -> Self {
        Self { path, auth_token }
    }
}

#[cfg(feature = "write")]
pub mod write {
    use reqwest::StatusCode;
    pub(crate) use crate::backend::ConfigBackend;
    use crate::device::{Device, Model};
    use crate::error::Error;
    use crate::libra::Config;

    impl ConfigBackend {
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
}

#[cfg(feature = "address")]
pub mod address {
    use reqwest::StatusCode;
    use serde::{Deserialize, Serialize};
    use crate::backend::ConfigBackend;
    use crate::device::Device;
    use crate::error::Error;

    impl ConfigBackend {
        pub fn get_address(&self, device: Device) -> Result<String, Error> {
            let client = reqwest::blocking::Client::new();
            let url = format!("{}/{}/{:?}/{}", self.path, "address", device.model, device.number);
            let response = client
                .get(url)
                .bearer_auth(&self.auth_token)
                .timeout(std::time::Duration::from_secs(60))
                .send()
                .map_err(Error::Reqwest)?;
            if response.status() == StatusCode::OK {
                let response: AddressResponse = response.json().map_err(Error::Reqwest)?;
                Ok(response.address)
            } else {
                Err(Error::Backend(response.status()))
            }
        }
        pub fn put_address(&self, device: Device, address: String) -> Result<(), Error> {
            let client = reqwest::blocking::Client::new();
            let url = format!("{}/{}/{:?}/{}", self.path, "address", device.model, device.number);
            let response = client
                .put(url)
                .json(&AddressResponse { address })
                .bearer_auth(&self.auth_token)
                .timeout(std::time::Duration::from_secs(60))
                .send()
                .map_err(Error::Reqwest)?;
            if response.status() == StatusCode::OK {
                Ok(())
            } else {
                Err(Error::Backend(response.status()))
            }
        }
    }
    #[derive(Deserialize, Serialize)]
    struct AddressResponse {
        address: String
    }
}


// pub struct CalibrationBackend {
//     path: String,
// }
// impl CalibrationBackend {
//     pub fn new(path: String) -> Self {
//         Self { path }
//     }
//
//     pub fn get_config(&self, phidget_id: isize) -> Result<String, Error> {
//         let client = reqwest::blocking::Client::new();
//         let url = format!("{}/{}", self.path, phidget_id);
//         response_from_client(client, url)
//     }
// }
//
//
// fn response_from_client(client: reqwest::blocking::Client, url: String) -> Result<String, Error> {
//     client
//         .get(url)
//         .timeout(std::time::Duration::from_secs(60))
//         .send()
//         .map_err(Error::Reqwest)?
//         .text()
//         .map_err(Error::Reqwest)
// }

#[cfg(any(feature = "write", feature = "address"))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{Device, Model};
    use crate::libra::Config;
    use mockito;
    use reqwest::StatusCode;
    use serde_json;
    use crate::error::Error;

    #[test]
    fn test_config_backend_make_new_device_success() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let model = Model::LibraV0;
        let config = Config::default();
        let token = "test-token";

        let expected_device = Device::new(model.clone(), 1);
        let mock = server
            .mock("POST", &format!("/{:?}", model)[..])
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_header("authorization", "Bearer test-token")
            .with_body(serde_json::to_string(&expected_device).unwrap())
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(&config).unwrap(),
            ))
            .create();

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.make_new_device(model, config);

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_device);
    }

    #[test]
    fn test_config_backend_make_new_device_error() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let model = Model::LibraV0;
        let config = Config::default();
        let token = "test-token";

        let mock = server
            .mock("POST", &format!("/{:?}", model)[..])
            .with_status(400)
            .with_header("authorization", "Bearer test-token")
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(&config).unwrap(),
            ))
            .create();

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.make_new_device(model, config);

        mock.assert();
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Backend(status) => assert_eq!(status, StatusCode::BAD_REQUEST),
            other_err => panic!("Expected Error::Backend, got {:?}", other_err),
        }
    }

    #[test]
    fn test_config_backend_get_config_success() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let device = Device::new(Model::LibraV0, 1);
        let token = "test-token";
        let expected_config = Config::default();

        let mock = server
            .mock("GET", &format!("/{:?}/{}", device.model, device.number)[..])
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("authorization", "Bearer test-token")
            .with_body(serde_json::to_string(&expected_config).unwrap())
            .create();

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.get_config(device);

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_config);
    }

    #[test]
    fn test_config_backend_get_config_error() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let device = Device::new(Model::LibraV0, 1);
        let token = "test-token";

        let mock = server
            .mock("GET", &format!("/{:?}/{}", device.model, device.number)[..])
            .with_status(404)
            .with_header("authorization", "Bearer test-token")
            .create();

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.get_config(device);

        mock.assert();
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Backend(status) => assert_eq!(status, StatusCode::NOT_FOUND),
            other_err => panic!("Expected Error::Backend, got {:?}", other_err),
        }
    }

    #[test]
    fn test_config_backend_edit_config_success() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let device = Device::new(Model::LibraV0, 1);
        let config = Config::default();
        let token = "test-token";

        let mock = server
            .mock("PUT", &format!("/{:?}/{}", device.model, device.number)[..])
            .with_status(200)
            .with_header("authorization", "Bearer test-token")
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(&config).unwrap(),
            ))
            .create();

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.edit_config(device, config);

        mock.assert();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_backend_edit_config_error() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let device = Device::new(Model::LibraV0, 1);
        let config = Config::default();
        let token = "test-token";

        let mock = server
            .mock("PUT", &format!("/{:?}/{}", device.model, device.number)[..])
            .with_status(500)
            .with_header("authorization", "Bearer test-token")
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(&config).unwrap(),
            ))
            .create();

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.edit_config(device, config);

        mock.assert();
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Backend(status) => assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR),
            other_err => panic!("Expected Error::Backend, got {:?}", other_err),
        }
    }

    #[tokio::test]
    async fn test_config_backend_make_new_device_async_success() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let model = Model::LibraV0;
        let config = Config::default();
        let token = "test-token";

        let expected_device = Device::new(model.clone(), 1);
        let mock = server
            .mock("POST", &format!("/{:?}", model)[..])
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_header("authorization", "Bearer test-token")
            .with_body(serde_json::to_string(&expected_device).unwrap())
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(&config).unwrap(),
            ))
            .create_async()
            .await;

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.make_new_device_async(model, config).await;

        mock.assert_async().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_device);
    }

    #[tokio::test]
    async fn test_config_backend_make_new_device_async_error() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let model = Model::LibraV0;
        let config = Config::default();
        let token = "test-token";

        let mock = server
            .mock("POST", &format!("/{:?}", model)[..])
            .with_status(400)
            .with_header("authorization", "Bearer test-token")
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(&config).unwrap(),
            ))
            .create_async()
            .await;

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.make_new_device_async(model, config).await;

        mock.assert_async().await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Backend(status) => assert_eq!(status, StatusCode::BAD_REQUEST),
            other_err => panic!("Expected Error::Backend, got {:?}", other_err),
        }
    }

    #[tokio::test]
    async fn test_config_backend_get_config_async_success() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let device = Device::new(Model::LibraV0, 1);
        let token = "test-token";
        let expected_config = Config::default();

        let mock = server
            .mock("GET", &format!("/{:?}/{}", device.model, device.number)[..])
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("authorization", "Bearer test-token")
            .with_body(serde_json::to_string(&expected_config).unwrap())
            .create_async()
            .await;

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.get_config_async(device).await;

        mock.assert_async().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_config);
    }

    #[tokio::test]
    async fn test_config_backend_get_config_async_error() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let device = Device::new(Model::LibraV0, 1);
        let token = "test-token";

        let mock = server
            .mock("GET", &format!("/{:?}/{}", device.model, device.number)[..])
            .with_status(404)
            .with_header("authorization", "Bearer test-token")
            .create_async()
            .await;

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.get_config_async(device).await;

        mock.assert_async().await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Backend(status) => assert_eq!(status, StatusCode::NOT_FOUND),
            other_err => panic!("Expected Error::Backend, got {:?}", other_err),
        }
    }

    #[tokio::test]
    async fn test_config_backend_edit_config_async_success() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let device = Device::new(Model::LibraV0, 1);
        let config = Config::default();
        let token = "test-token";

        let mock = server
            .mock("PUT", &format!("/{:?}/{}", device.model, device.number)[..])
            .with_status(200)
            .with_header("authorization", "Bearer test-token")
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(&config).unwrap(),
            ))
            .create_async()
            .await;

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.edit_config_async(device, config).await;

        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_config_backend_edit_config_async_error() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let device = Device::new(Model::LibraV0, 1);
        let config = Config::default();
        let token = "test-token";

        let mock = server
            .mock("PUT", &format!("/{:?}/{}", device.model, device.number)[..])
            .with_status(500)
            .with_header("authorization", "Bearer test-token")
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(&config).unwrap(),
            ))
            .create_async()
            .await;

        let backend = ConfigBackend::new(url.to_string(), token.to_string());
        let result = backend.edit_config_async(device, config).await;

        mock.assert_async().await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Backend(status) => assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR),
            other_err => panic!("Expected Error::Backend, got {:?}", other_err),
        }
    }
}
