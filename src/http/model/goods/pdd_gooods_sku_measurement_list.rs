//! # 商品sku计量单位枚举
//!
//! 商品sku计量单位枚举
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGooodsSkuMeasurementListRequest;

impl RequestType for PddGooodsSkuMeasurementListRequest {
    type Response = PddGooodsSkuMeasurementListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.gooods.sku.measurement.list"
    }
}

#[derive(Debug, Deserialize)]
pub struct Measurement {
    /// 编码
    #[serde(default)]
    pub code: String,
    /// 说明
    #[serde(default)]
    pub desc: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGooodsSkuMeasurementListResponse {
    /// 列表
    #[serde(default)]
    pub measurement_list: Option<Vec<Measurement>>,
}
