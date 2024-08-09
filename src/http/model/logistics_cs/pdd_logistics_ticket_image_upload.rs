//! # 工单图片上传接口
//!
//! 快递公司处理结果回调接口 附件图片url生成
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsTicketImageUploadRequest {
    /// 支持格式有：jpg/jpeg、png等图片格式，入参为图片的base64编码，最大支持1M
    pub image: String,
}

impl RequestType for PddLogisticsTicketImageUploadRequest {
    type Response = PddLogisticsTicketImageUploadResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.ticket.image.upload"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsTicketImageUploadResponse {
    /// 图片url
    #[serde(default)]
    pub image_url: String,
}
