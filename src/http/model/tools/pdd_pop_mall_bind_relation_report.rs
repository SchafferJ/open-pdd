//! # 店铺关联关系上报
//!
//! ISV上报存量店铺关联关系
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddPopMallBindRelationReportRequest {
    /// 关联时间
    pub bind_at: i64,
    /// 关联类型：0-关联码关联，1-授权关联
    pub bind_type: i32,
    /// 三方应用的用户id
    pub external_uid: String,
    /// 被关联方的店铺id
    pub invitee_mall_id: i64,
    /// 发起关联方的店铺id
    pub inviter_mall_id: i64,
    /// 当前店群包含的拼多多店铺id
    pub mall_list: Option<Vec<i64>>,
}

impl RequestType for PddPopMallBindRelationReportRequest {
    type Response = PddPopMallBindRelationReportResponse;

    fn get_type(&self) -> &'static str {
        "pdd.pop.mall.bind.relation.report"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddPopMallBindRelationReportResponse {
    #[serde(default)]
    pub success: bool,
}
