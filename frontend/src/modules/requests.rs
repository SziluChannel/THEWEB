use reqwest::{Response, Client, Method, Error};
use serde::{Serialize, de::DeserializeOwned};
use crate::BACKEND;

async fn request<B>(method: Method, path: &str, body: B) -> Response
where B: Serialize {
    let allow_body = method == Method::POST || method == Method::PUT;
    let mut builder = Client::new()
        .request(method, &format!("{BACKEND}{path}"))
        .header("Content-Type", "application/json");
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

pub async fn post_request<B, T>(path: &str, body: B) -> Result<T, Error>
where
    T: DeserializeOwned,
    B: Serialize {
    Ok(
        request::<B>(
            Method::POST,
            path,
            body)
            .await
            .json::<T>()
            .await?
    )
}

pub async fn get_request<B>(path: &str) -> Result<B, Error>
where B: DeserializeOwned {
    Ok(
        request::<()>(
            Method::GET,
            path,
            ())
            .await
            .json::<B>()
            .await?
        )
}
