use std::net::SocketAddr;

use reqwest::{Body, ClientBuilder};
use serde::de::DeserializeOwned;
use serde_json::json;
use url::Url;

use crate::{
    error::{Error, Result},
    http_client::HttpClient,
    models::*,
};

pub enum NanoleafState {
    On,
    Off,
}

pub struct Client {
    inner: HttpClient,
}

impl Client {
    pub fn with_socketaddr(sa: SocketAddr) -> Result<Self> {
        let base_url = Url::parse(&format!("http://{}/api/v1/", sa)).unwrap();
        let client = ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let http = HttpClient::new(base_url, client);
        Ok(Client { inner: http })
    }

    // ====================
    // Users
    // ====================

    pub async fn add_user(&self) -> Result<Authorization> {
        self.post_value("new", "").await
    }

    pub async fn delete_user(&self, token: &str) -> Result<()> {
        self.delete_value(token).await
    }

    // ====================
    // Panel Info
    // ====================

    pub async fn get_all_info(&self, token: &str) -> Result<PanelInfo> {
        self.get_value(token).await
    }

    // ====================
    // Panel State
    // ====================

    pub async fn get_state(&self, token: &str) -> Result<State> {
        self.get_value(&format!("{}/state", token)).await
    }

    // ====================
    // On
    // ====================

    pub async fn get_on(&self, token: &str) -> Result<On> {
        self.get_value(&format!("{}/state/on", token)).await
    }

    pub async fn set_on(&self, token: &str, on: On) -> Result<()> {
        self.put_value(&format!("{}/state", token), json!({ "on": on }).to_string())
            .await
    }

    // ====================
    // Brightness
    // ====================
    pub async fn get_brightness(&self, token: &str) -> Result<Range> {
        self.get_value(&format!("{}/state/brightness", token)).await
    }

    pub async fn set_brightness(&self, token: &str, brightness: Brightness) -> Result<()> {
        self.put_value(
            &format!("{}/state", token),
            json!({ "brightness": brightness }).to_string(),
        )
        .await
    }

    // ====================
    // Hue
    // ====================
    pub async fn get_hue(&self, token: &str) -> Result<Range> {
        self.get_value(&format!("{}/state/hue", token)).await
    }

    pub async fn set_hue(&self, token: &str, hue: SetRange) -> Result<()> {
        self.put_value(
            &format!("{}/state", token),
            json!({ "hue": hue }).to_string(),
        )
        .await
    }

    // ====================
    // Saturation
    // ====================
    pub async fn get_saturation(&self, token: &str) -> Result<Range> {
        self.get_value(&format!("{}/state/sat", token)).await
    }

    pub async fn set_saturation(&self, token: &str, sat: SetRange) -> Result<()> {
        self.put_value(
            &format!("{}/state", token),
            json!({ "sat": sat }).to_string(),
        )
        .await
    }

    // ====================
    // Color Temperature
    // ====================
    pub async fn get_ct(&self, token: &str) -> Result<Range> {
        self.get_value(&format!("{}/state/ct", token)).await
    }

    pub async fn set_ct(&self, token: &str, ct: SetRange) -> Result<()> {
        self.put_value(&format!("{}/state", token), json!({ "ct": ct }).to_string())
            .await
    }

    // ====================
    // Color Mode
    // ====================

    pub async fn get_color_mode(&self, token: &str) -> Result<String> {
        self.get_value(&format!("{}/effects/select", token)).await
    }

    // ====================
    // Effects
    // ====================

    pub async fn get_effect(&self, token: &str) -> Result<String> {
        self.get_value(&format!("{}/effects/select", token)).await
    }

    pub async fn set_effect(&self, token: &str, effect: &str) -> Result<()> {
        self.put_value(
            &format!("{}/effects/select", token),
            json!({ "select": effect }).to_string(),
        )
        .await
    }

    pub async fn get_all_effects(&self, token: &str) -> Result<Vec<String>> {
        self.get_value(&format!("{}/effects/effectsList", token))
            .await
    }

    pub async fn start_external_streaming(&self, token: &str) -> Result<()> {
        self.put_value(
            &format!("{}/effects", token),
            json!({"write":{"command":"display","animType":"extControl","extControlVersion":"v2"}})
                .to_string(),
        )
        .await
    }

    // ====================
    // Helpers
    // ====================
    pub async fn get_value<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let res = self.inner.get(path).await?;
        let res = res.error_for_status().map_err(Error::from)?;
        res.json::<T>().await.map_err(Error::from)
    }

    pub async fn delete_value(&self, path: &str) -> Result<()> {
        let res = self.inner.delete(path).await?;
        let res = res.error_for_status().map_err(Error::from)?;
        res.json::<()>().await.map_err(Error::from)
    }

    pub async fn post_value<B, T>(&self, path: &str, body: B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Into<Body>,
    {
        let res = self.inner.post(path, body).await?;
        let res = res.error_for_status().map_err(Error::from)?;
        res.json::<T>().await.map_err(Error::from)
    }

    pub async fn put_value<B: Into<Body>>(&self, path: &str, body: B) -> Result<()> {
        let res = self.inner.put(path, body).await?;
        res.error_for_status().map_err(Error::from)?;

        // API returns 204 No Content or a 4xx error only
        Ok(())
    }
}
