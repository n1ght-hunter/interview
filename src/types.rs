use serde::{Deserialize, Serialize};



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root<T> {
    pub method: Option<String>,
    pub protocol: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
    pub ip: Option<String>,
    pub headers: Headers,
    pub parsed_query_params: ParsedQueryParams,
    pub parsed_body: T,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers {
    #[serde(rename = "Host")]
    pub host: Option<String>,
    #[serde(rename = "User-Agent")]
    pub user_agent: Option<String>,
    #[serde(rename = "Content-Length")]
    pub content_length: Option<String>,
    #[serde(rename = "Accept")]
    pub accept: Option<String>,
    #[serde(rename = "Content-Type")]
    pub content_type: Option<String>,
    #[serde(rename = "Accept-Encoding")]
    pub accept_encoding: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedQueryParams {
}

