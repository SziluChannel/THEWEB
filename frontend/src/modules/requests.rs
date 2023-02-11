use reqwest::{Response, Client, Method, Error};
use serde::{Serialize, de::DeserializeOwned};
use web_sys::window;
use models::{HttpAnswer};
use crate::BACKEND;

async fn request<B>(method: Method, path: &str, body: B) -> Response
where B: Serialize {
    let session_storage = window().unwrap().session_storage().unwrap().unwrap();
    let jwt = session_storage.get("jwt").unwrap_or_default().unwrap_or_default();
    let allow_body = method == Method::POST || method == Method::PUT;
    let mut builder = Client::new()
        .request(method, &format!("{BACKEND}{path}"))
        .header("Content-Type", "application/json");
    if jwt != "".to_string() {
        builder = builder.header("Authorization", format!("bearer {jwt}"));
    }
    if allow_body {
        builder =   builder.json(&body);
    }
    builder.send().await.unwrap()
}

pub async fn delete_request<B>(path: &str, body: B) -> Result<u16, Error>
where B: Serialize {
    Ok(
        request::<B>(
            Method::DELETE,
            path,
            body)
            .await
            .status()
            .as_u16()
    )
}

pub async fn put_request<B>(path: &str, body: B) -> Result<u16, Error>
where B: Serialize {
    Ok(
        request::<B>(
            Method::PUT,
            path,
            body)
            .await
            .status()
            .as_u16()
    )
}

pub async fn post_request<B, T>(path: &str, body: B) -> Result<HttpAnswer<T>, Error>
where
    T: DeserializeOwned,
    B: Serialize {
    Ok(
        request::<B>(
            Method::POST,
            path,
            body)
            .await
            .json::<HttpAnswer<T>>()
            .await?
    )
}

pub async fn get_request<B>(path: &str) -> Result<HttpAnswer<B>, Error>
where B: DeserializeOwned {
    Ok(
        request::<()>(
            Method::GET,
            path,
            ())
            .await
            .json::<HttpAnswer<B>>()
            .await?
        )
}
