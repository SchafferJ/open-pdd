//! 更新创意内容

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AdImageVO {
    /// 图片链接，可用图片参考以下接口返回：pdd.ad.api.goods.query.gallery.images（轮播图），pdd.ad.api.goods.query.long.images（长图）
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

#[derive(Debug, Serialize)]
pub struct AdTextVO {
    /// 标题文字
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct AdCreativeUpdateMessage {
    /// 创意图片列表
    #[serde(rename = "adImageVOList")]
    pub ad_image_vo_list: Option<Vec<AdImageVO>>,
    /// 创意标题列表
    #[serde(rename = "adTextVOList")]
    pub ad_text_vo_list: Option<Vec<AdTextVO>>,
    /// 创意规格Id，6-轮播图，7-长图
    #[serde(rename = "creativeSpecificationId")]
    pub creative_specification_id: i64,
}

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitCreativeUpdateContentRequest {
    /// 创意更新输入
    #[serde(rename = "adCreativeUpdateMessage")]
    pub ad_creative_update_message: AdCreativeUpdateMessage,
    /// 创意单元Id
    #[serde(rename = "unitCreativeId")]
    pub unit_creative_id: i64,
}

impl RequestType for PddAdApiUnitCreativeUpdateContentRequest {
    type Response = PddAdApiUnitCreativeUpdateContentResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.creative.update.content"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitCreativeUpdateContentResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 是否创建成功
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub success: bool,
}
