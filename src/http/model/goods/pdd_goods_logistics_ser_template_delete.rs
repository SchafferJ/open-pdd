//! # 商品送装服务模版删除
//!
//! 商品送装服务模版删除
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsLogisticsSerTemplateDeleteRequest {
    /// 模版id
    pub template_id: String,
}

impl RequestType for PddGoodsLogisticsSerTemplateDeleteRequest {
    type Response = PddGoodsLogisticsSerTemplateDeleteResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.logistics.ser.template.delete"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsLogisticsSerTemplateDeleteResponse {
    /// is_success
    #[serde(default)]
    pub is_success: bool,
}
