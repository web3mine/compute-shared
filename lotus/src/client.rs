use crate::{config::LotusConfig, types::api::Rs};
use crate::{Error, RPCRequest};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde_json::{json, Value};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct LotusClient {
    config: LotusConfig,
    client: Client,
}

impl<'de> LotusClient {
    pub fn init(config: LotusConfig) -> LotusClient {
        LotusClient {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub fn build(&self) -> RequestBuilder {
        let request_builder = self
            .client
            .post(self.config.host_url.to_string())
            .header("Content-Type", self.config.content_type.to_string());

        if !&self.config.token.is_empty() {
            request_builder.bearer_auth(self.config.token.to_string())
        } else {
            request_builder
        }
    }

    pub fn format<T: serde::Serialize>(&self, method: String, val: T) -> RPCRequest {
        RPCRequest {
            jsonrpc: "2.0".to_string(),
            method,
            id: 1,
            params: json!(val),
        }
    }

    pub async fn send<Res: serde::de::DeserializeOwned>(
        &self,
        method: String,
        data: Vec<Value>,
    ) -> std::result::Result<Res, Error> {
        let request = self
            .build()
            .json(&self.format(format!("Filecoin.{}", method), &data));
        tracing::debug!("Request: {:?}", request);

        let response = request.send().await?;

        let status = response.status();
        let text_body = response.text().await?;
        tracing::debug!("Response\nstatus code: {}\nbody: {}", status, text_body);

        if status != StatusCode::OK {
            tracing::error!("Lotus error\nstatus code: {}\nbody: {}", status, text_body);
            return Err(Error::Lotus(status, text_body));
        }

        let trace_body = |error, body: &str| {
            tracing::error!(
                "Invalid body: {}",
                if body.is_empty() {
                    "<empty-body>"
                } else {
                    body
                }
            );

            error
        };

        let json_response = serde_json::from_str::<Rs<Res>>(&text_body);
        match json_response {
            Ok(rs) => Ok(rs.result),
            Err(err) => Err(trace_body(err, &text_body))?,
        }
    }

    pub async fn send_no_reponse_body(
        &self,
        method: String,
        data: Vec<Value>,
    ) -> std::result::Result<(), Error> {
        let request = self
            .build()
            .json(&self.format(format!("Filecoin.{}", method), &data));
        tracing::debug!("Request: {:?}", request);

        let response = request.send().await?;

        let status = response.status();
        let text_body = response.text().await?;
        tracing::debug!("Response\nstatus code: {}\nbody: {}", status, text_body);

        if status != StatusCode::OK {
            tracing::error!("Lotus error\nstatus code: {}\nbody: {}", status, text_body);
            return Err(Error::Lotus(status, text_body));
        }

        Ok(())
    }
}
