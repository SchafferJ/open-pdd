//! # 商品运费模版接口
//!
//! 获取拼多多商家的物流运费模板信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsLogisticsTemplateGetRequest {
    /// 默认返回运费模板的页数为1，最高为100页，注意：page与page_size必须传一个
    pub page: Option<i32>,
    /// 默认返回20条模板数据，最多100条数据
    pub page_size: Option<i32>,
    /// 0-新发布商品，1-编辑商品。如传值为空，平台默认为发布商品
    pub goods_status: Option<i32>,
}

impl RequestType for PddGoodsLogisticsTemplateGetRequest {
    type Response = PddGoodsLogisticsTemplateGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.logistics.template.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct LogisticsTemplate {
    /// 计费方式，0-按件计费，1-按重量计费
    #[serde(default)]
    pub cost_type: i32,
    /// 最近更新时间
    #[serde(default)]
    pub last_updated_time: i32,
    /// 模板id
    #[serde(default)]
    pub template_id: i64,
    /// 运费模板名称
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsLogisticsTemplateGetResponse {
    /// 商家运费模板对象列表
    #[serde(default)]
    pub logistics_template_list: Option<Vec<LogisticsTemplate>>,
    /// 返回的运费模板总数
    #[serde(default)]
    pub total_count: i32,
}
