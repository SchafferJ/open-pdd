//! # 店铺信息接口
//!
//! 通过此接口获取店铺信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddMallInfoGetRequest;

impl RequestType for PddMallInfoGetRequest {
    type Response = PddMallInfoGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.mall.info.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddMallInfoGetResponse {
    /// 店铺logo
    #[serde(default)]
    pub logo: String,
    /// 店铺描述
    #[serde(default)]
    pub mall_desc: String,
    /// 店铺id
    #[serde(default)]
    pub mall_id: i64,
    /// 店铺名称
    #[serde(default)]
    pub mall_name: String,
    /// 店铺类型,1:个人 2:企业 3:旗舰店 4:专卖店 5:专营店 6:普通店
    #[serde(default)]
    pub merchant_type: i32,
    /// 店铺身份,0:厂商 1:分销商 2:都不是 3:都是
    #[serde(default)]
    pub mall_character: i32,
}
