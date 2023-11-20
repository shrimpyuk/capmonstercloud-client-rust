#![allow(non_snake_case)]

use crate::error::SvcRequestError;
use crate::requests::tasks_names::TaskReqTrait;
use crate::responses::{SvcResponse, SvcRespTypeTrait};
use reqwestplus::{Client, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
#[cfg(feature = "debug-output")]
use tracing::warn;

pub(crate) mod tasks_data;
pub(crate) mod tasks_names;

#[allow(clippy::needless_lifetimes)]
pub(crate) async fn make_svc_request<T: SvcRespTypeTrait + DeserializeOwned>(
    url: Url,
    body: String,
    http_client: &Client,
) -> Result<SvcResponse<T>, SvcRequestError> {    
    #[cfg(feature = "debug-output")]
    warn!("Body:\n'{}'", body);
    
    let raw_resp = http_client
        .post(url)
        .body(body)
        .send()
        .await
        .map_err(SvcRequestError::PostRequestError)?;
    
    let resp_status = raw_resp.status();
    
    if resp_status != StatusCode::OK {
        return Err(SvcRequestError::NonSuccessRespStatus(resp_status));
    }
    
    Ok(SvcResponse::new(raw_resp))
}

#[derive(Serialize)]
pub(crate) struct CreateTaskRequest<T: TaskReqTrait> {
    clientKey: String,
    task: TaskData<T>,
    // pub(crate) callbackUrl: Option<String>,
    softId: u32,
}

impl<'a, T: TaskReqTrait> CreateTaskRequest<T> {
    pub(crate) fn new(
        clientKey: String,
        task: TaskData<T>,
        softId: u32,
    ) -> CreateTaskRequest<T> {
        Self {
            clientKey,
            task,
            softId,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct TaskData<T: TaskReqTrait> {
    #[serde(rename = "type")]
    typ: String,
    #[serde(flatten)]
    flat_data: T,
}

impl<'a, T: TaskReqTrait> TaskData<T> {
    pub(crate) fn new(flatten: T) -> Self {
        Self {
            typ: flatten.get_task_name().to_string(),
            flat_data: flatten,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct GetBalanceRequest {
    pub(crate) clientKey: String,
}

#[derive(Serialize)]
pub(crate) struct GetTaskResultRequest {
    pub(crate) clientKey: String,
    pub(crate) taskId: u32,
}
