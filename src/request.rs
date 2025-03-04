#![allow(dead_code)]
use anyhow::Result;
use std::sync::LazyLock;

use reqwest::{Client, IntoUrl, Response};

/// DNS resolve with `trust_dns`
static HTTP_CLIENT: LazyLock<reqwest::Result<Client>> =
    LazyLock::new(|| reqwest::ClientBuilder::new().use_rustls_tls().build());

pub async fn get_bytes<U: IntoUrl>(url: U) -> Result<Vec<u8>> {
    let c = HTTP_CLIENT.as_ref()?;
    let bytes: Vec<u8> = c.get(url).send().await?.bytes().await?.to_vec();
    Ok(bytes)
}

pub async fn get_text<U: IntoUrl>(url: U) -> Result<String> {
    let c = HTTP_CLIENT.as_ref()?;
    let text = c.get(url).send().await?.text().await?;
    Ok(text)
}

pub async fn get<U: IntoUrl>(url: U) -> Result<Response> {
    let c = HTTP_CLIENT.as_ref()?;
    let bs = c.get(url).send().await?;
    Ok(bs)
}
