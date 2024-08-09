//! 获取可用基础定向

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitBidQueryBaseTargetProfileRequest {
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
    /// 场景类型，0-搜索，2-展示
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
}

impl RequestType for PddAdApiUnitBidQueryBaseTargetProfileRequest {
    type Response = PddAdApiUnitBidQueryBaseTargetProfileResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.bid.query.base.target.profile"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 定向类型描述
    #[serde(default, rename = "targetName")]
    pub target_name: String,
    /// 定向类型
    #[serde(default, rename = "targetType")]
    pub target_type: i32,
    /// 用户数量预估
    #[serde(default, rename = "uvString")]
    pub uv_string: String,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitBidQueryBaseTargetProfileResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Vec<Result>>,
    #[serde(default)]
    pub success: bool,
}
