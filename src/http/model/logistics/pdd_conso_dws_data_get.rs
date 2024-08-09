//! # 获取集运DWS设备采集数据
//!
//! 集运获取集运DWS设备采集数据
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 设备id
    #[serde(rename = "equipmentId")]
    pub equipment_id: String,
    /// 设备供应商
    #[serde(rename = "equipmentSupplier")]
    pub equipment_supplier: String,
    /// 体积-高(单位：cm)
    pub height: Option<String>,
    /// 体积-长(单位：cm)
    pub length: Option<String>,
    /// 扫描时间戳(毫秒)
    #[serde(rename = "scanTime")]
    pub scan_time: i64,
    /// 运单号
    #[serde(rename = "trckNo")]
    pub trck_no: String,
    /// 体积(单位：cm^3)
    pub volume: Option<String>,
    /// 体积重
    #[serde(rename = "volumeWeight")]
    pub volume_weight: Option<String>,
    /// 称重重量(单位：g)
    pub weight: Option<String>,
    /// 体积-宽(单位：cm)
    pub width: Option<String>,
    /// 图片链接列表
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct PddConsoDwsDataGetRequest {
    /// 获取DWS数据请求参数
    pub request: Request,
}

impl RequestType for PddConsoDwsDataGetRequest {
    type Response = PddConsoDwsDataGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.conso.dws.data.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddConsoDwsDataGetResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
