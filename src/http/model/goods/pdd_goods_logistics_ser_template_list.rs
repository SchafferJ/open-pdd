//! # 商品送装服务模版列表
//!
//! 商品送装服务模版列表
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsLogisticsSerTemplateListRequest {
    /// 查询大小
    pub length: i32,
    /// 查询类型
    pub query_type: i32,
    /// 查询偏移量
    pub start: i32,
    /// 模板类型
    pub template_type: i32,
}

impl RequestType for PddGoodsLogisticsSerTemplateListRequest {
    type Response = PddGoodsLogisticsSerTemplateListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.logistics.ser.template.list"
    }
}

#[derive(Debug, Deserialize)]
pub struct List {
    /// 模版id
    #[serde(default)]
    pub template_id: String,
    /// 模版名称
    #[serde(default)]
    pub template_name: String,
    /// 更新时间
    #[serde(default)]
    pub update_time: i32,
    /// 使用情况
    #[serde(default)]
    pub using: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsLogisticsSerTemplateListResponse {
    /// 列表
    #[serde(default)]
    pub list: Option<Vec<List>>,
    /// 总数
    #[serde(default)]
    pub total: i32,
}
