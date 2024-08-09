//! # 轨迹查询接口
//!
//! 查询单个运单详情
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsOrdertraceGetRequest {
    /// 1
    pub company_code: Option<String>,
    /// 1
    pub mail_no: Option<String>,
}

impl RequestType for PddLogisticsOrdertraceGetRequest {
    type Response = PddLogisticsOrdertraceGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.ordertrace.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Trace {
    /// 节点说明 ，指明当前节点揽收、派送，签收。
    #[serde(default)]
    pub action: String,
    /// 描述
    #[serde(default)]
    pub desc: String,
    /// 地址地一
    #[serde(default)]
    pub node_description: String,
    /// 状态描述
    #[serde(default)]
    pub status_desc: String,
    /// 状态发生的时间
    #[serde(default)]
    pub status_time: String,
    /// 时间。。
    #[serde(default)]
    pub time: String,
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsOrdertraceGetResponse {
    /// 返回列表
    #[serde(default)]
    pub trace_list: Option<Vec<Trace>>,
}
