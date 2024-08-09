//! # 创建多多进宝推广位
//!
//! 创建多多进宝推广位
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkGoodsPidGenerateRequest {
    /// 要生成的推广位数量，默认为10，范围为：1~100
    pub number: i64,
    /// 推广位名称，例如["1","2"]
    pub p_id_name_list: Option<Vec<String>>,
    /// 媒体id
    pub media_id: Option<i64>,
}

impl RequestType for PddDdkGoodsPidGenerateRequest {
    type Response = PddDdkGoodsPidGenerateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.goods.pid.generate"
    }
}

#[derive(Debug, Deserialize)]
pub struct PId {
    /// 推广位创建时间
    #[serde(default)]
    pub create_time: i64,
    /// 推广位名称
    #[serde(default)]
    pub pid_name: String,
    /// 调用方推广位ID
    #[serde(default)]
    pub p_id: String,
    /// 媒体id
    #[serde(default)]
    pub media_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkGoodsPidGenerateResponse {
    /// 多多进宝推广位对象列表
    #[serde(default)]
    pub p_id_list: Option<Vec<PId>>,
    /// PID剩余数量
    #[serde(default)]
    pub remain_pid_count: i32,
}
