use async_graphql::InputObject;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, InputObject, Clone)]
pub struct FileBody {
    pub headcount: usize,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject, Clone)]
pub struct Fileresponse {
    pub head: serde_json::Value,
    pub describe: serde_json::Value,
}
