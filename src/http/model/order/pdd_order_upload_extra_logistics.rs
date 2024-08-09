//! # 订单额外运单信息上传
//!
//! 针对一笔订单分多笔物流发货的场景（分包发货、补寄、发放赠品），将支持商家额外上传运单号，额外运单作为补充信息仅用作消费者查看。
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
pub struct PddOrderUploadExtraLogisticsRequest {
    /// 订单多包裹发货时使用的其他发货快递信息
    pub extra_track_list: Vec<ExtraTrack>,
    /// 订单号
    pub order_sn: String,
    /// 额外运单类型，1=分包发货，2=补发商品，3=发放赠品
    pub extra_track_type: Option<i32>,
}

impl RequestType for PddOrderUploadExtraLogisticsRequest {
    type Response = PddOrderUploadExtraLogisticsResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.upload.extra.logistics"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddOrderUploadExtraLogisticsResponse {
    /// 是否成功，false-失败，true-成功
    #[serde(default)]
    pub success: bool,
}
