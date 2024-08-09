//! # 订单关联运单信息上传
//!
//! 针对一笔订单存在多次发货的场景使用，包括但不仅限于补发，换货，分批次发货，线下手工订单等能够匹配到拼多多平台订单号的场景。商家将关联的发货运单号上传后，将用作提升消费者末端取件体验。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ExtraTrack {
    /// 快递公司id
    pub shipping_id: i32,
    /// 快递单号
    pub tracking_number: String,
}

#[derive(Debug, Serialize)]
pub struct PddOrderUploadRelationLogisticsRequest {
    /// 订单多包裹发货时使用的其他发货快递信息
    pub extra_track_list: Vec<ExtraTrack>,
    /// 订单号
    pub order_sn: String,
}

impl RequestType for PddOrderUploadRelationLogisticsRequest {
    type Response = PddOrderUploadRelationLogisticsResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.upload.relation.logistics"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOrderUploadRelationLogisticsResponse {
    /// 是否成功，false-失败，true-成功
    #[serde(default)]
    pub success: bool,
}
