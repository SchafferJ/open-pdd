//! # 查询订单承诺信息
//!
//! 查询订单承诺信息，用于打单等场景下的承诺信息展示
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOrderPromiseInfoGetRequest {
    /// 承诺id
    pub promise_id: i64,
}

impl RequestType for PddOrderPromiseInfoGetRequest {
    type Response = PddOrderPromiseInfoGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.promise.info.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PromiseInfo {
    /// 承诺详情
    #[serde(default)]
    pub detail_info: std::collections::BTreeMap<String, serde_json::Value>,
    /// 额外信息
    #[serde(default)]
    pub extra_info: std::collections::BTreeMap<String, serde_json::Value>,
    /// 是否已删除
    #[serde(default)]
    pub is_deleted: bool,
    /// 订单号
    #[serde(default)]
    pub order_sn: String,
    /// 服务承诺描述
    #[serde(default)]
    pub promise_desc: String,
    /// 服务承诺单性质，1-平台协议，2-客服承 诺，3-平台建议
    #[serde(default)]
    pub promise_level: i32,
    /// 服务承诺状态，0-已创建，1-履约中，2-履 约未达成，3-履约成功
    #[serde(default)]
    pub promise_status: i32,
    /// 服务承诺类型，1-指定快递/物流，2-优先发 货
    #[serde(default)]
    pub promise_type: i32,
    /// 承诺信息id
    #[serde(default)]
    pub promise_id: i64,
    /// 业务场景，1-发货场景
    #[serde(default)]
    pub scene_type: i32,
    /// 最后更新时间
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct PddOrderPromiseInfoGetResponse {
    #[serde(default)]
    pub promise_info: Option<PromiseInfo>,
}
