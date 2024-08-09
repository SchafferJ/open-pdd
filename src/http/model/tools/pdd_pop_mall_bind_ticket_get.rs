//! # 获取店铺关联ticket
//!
//! ISV多店铺关联时，获取发起方店铺身份ticket，用于生成店铺关联链接
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPopMallBindTicketGetRequest {
    /// 三方应用的用户id
    pub external_uid: String,
    /// 当前店群包含的拼多多店铺id
    pub mall_list: Option<Vec<i64>>,
}

impl RequestType for PddPopMallBindTicketGetRequest {
    type Response = PddPopMallBindTicketGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.pop.mall.bind.ticket.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPopMallBindTicketGetResponse {
    /// ticket
    #[serde(default)]
    pub ticket: String,
}
