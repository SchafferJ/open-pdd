//! # 末端三段轨迹回传
//!
//! 集运末端业务，物流商回传未拼接二段物流的三段轨迹信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct Request {
    /// 地址
    pub address: Option<String>,
    /// 扫描城市名称
    pub city: Option<String>,
    /// 轨迹详情描述
    pub description: String,
    /// 三级地址，区/县
    pub district: Option<String>,
    /// 问题件原因code
    #[serde(rename = "failReason")]
    pub fail_reason: Option<String>,
    /// 数据id java.util.UUID生成
    pub id: String,
    /// 操作时间 格式：yyyy-MM-dd hh:mm:ss
    #[serde(rename = "operationTime")]
    pub operation_time: String,
    /// 省份
    pub province: Option<String>,
    /// 快递公司id
    #[serde(rename = "shippingId")]
    pub shipping_id: i64,
    /// 扫描站点名称
    #[serde(rename = "siteName")]
    pub site_name: Option<String>,
    /// 扫描站点编码 站点编号(各快递公司用于区分站点的唯一id)
    #[serde(rename = "siteNo")]
    pub site_no: Option<String>,
    /// 扫描站点类型 1:网点；2:中转中心；3:代收点
    #[serde(rename = "siteType")]
    pub site_type: Option<i32>,
    /// 轨迹状态 如：GOT、SEND
    pub status: String,
    /// 运单号
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    /// 物流号 物流订单号
    #[serde(rename = "trackingOrderNo")]
    pub tracking_order_no: String,
}

#[derive(Debug, Serialize)]
pub struct PddTailExpressTraceSyncRequest {
    /// 轨迹信息
    pub request: Request,
}

impl RequestType for PddTailExpressTraceSyncRequest {
    type Response = PddTailExpressTraceSyncResponse;

    fn get_type(&self) -> &'static str {
        "pdd.tail.express.trace.sync"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::ARK
    }
}

#[derive(Debug, Deserialize)]
pub struct PddTailExpressTraceSyncResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub success: bool,
}
