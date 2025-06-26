#[cfg(feature = "generate")]
use crate::error::Error;
#[cfg(feature = "generate")]
use serde::de::DeserializeOwned;
#[cfg(feature = "generate")]
pub trait FromBackend {
    fn pull(url: &str) -> Result<Self, Error> 
    where 
        Self: Sized + DeserializeOwned
    {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .map_err(Error::Reqwest)?
            .text()
            .map_err(Error::Reqwest)?;
        let instance = serde_json::from_str(&response).map_err(Error::SerdeJson)?;
        Ok(instance)
    }
}