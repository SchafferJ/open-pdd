//! # 千万神券活动数据列表
//!
//! 支持当前日期前6天到后7天的时间范围查询千万神券活动，日期超过范围将用最大边界时间替换
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkTmcActivityListRequest {
    /// 页码 从1开始
    pub page_num: i32,
    /// 每页结果数，默认值: 20 最大50
    pub page_size: i32,
    /// 活动开始时间最小时间 格式: "yyyy-MM-dd HH:mm:ss"
    pub start_time_lower: String,
    /// 活动开始时间最大时间 格式: "yyyy-MM-dd HH:mm:ss"
    pub start_time_upper: String,
}

impl RequestType for PddDdkTmcActivityListRequest {
    type Response = PddDdkTmcActivityListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.tmc.activity.list"
    }
}

#[derive(Debug, Deserialize)]
pub struct TmcAtyVo {
    /// 活动结束时间
    #[serde(default)]
    pub end_time: String,
    /// 神券活动编号 用于生链是传递
    #[serde(default)]
    pub id: i64,
    /// 活动名称
    #[serde(default)]
    pub name: String,
    /// 活动开始时间
    #[serde(default)]
    pub start_time: String,
    /// 活动类型 1日常活动日 2品牌日
    #[serde(default)]
    pub r#type: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkTmcActivityListResponse {
    /// 实际查询的结束时间
    #[serde(default)]
    pub query_end_time: String,
    /// 实际查询的开始时间
    #[serde(default)]
    pub query_start_time: String,
    /// 千万神券活动列表
    #[serde(default)]
    pub tmc_aty_vo_list: Option<Vec<TmcAtyVo>>,
    /// 活动总数
    #[serde(default)]
    pub total: i32,
}
