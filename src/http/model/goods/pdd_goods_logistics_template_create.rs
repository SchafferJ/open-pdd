//! # 创建商品物流模版
//!
//! 创建物流模版
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CostProvince {
    /// 省份ID
    pub province_id: i32,
}

#[derive(Debug, Serialize)]
pub struct CostTemplate {
    /// 首件
    pub first_standard: i64,
    /// 首件或首重价格，单位为分
    pub first_cost: i64,
    /// 续重或续件，续重时单位为克且数值须为1000的整数倍
    pub add_standard: i64,
    /// 续件或续重价格，单位为分
    pub add_cost: i64,
    /// 对不包邮地区，true-若要包邮须满足件数包邮，false-不开启满足件数包邮
    pub is_have_free_min_count: bool,
    /// 对不包邮地区，满足指定件数包邮，该值为商家设置的指定件数，若为-1则商家没有开启满足件数包邮
    pub have_free_min_count: i32,
    /// 对不包邮地区，true-若要包邮须满足指定价格则可以包邮，false-不开启满足指定价格包邮
    pub is_have_free_min_amount: bool,
    /// 对不包邮地区，满足指定价格包邮，该值为商家设置的指定订单金额，若为-1则商家没有开启满足指定价格包邮，注意，单位为分
    pub have_free_min_amount: i64,
    /// 省份列表
    pub cost_province_list: Vec<CostProvince>,
}

#[derive(Debug, Serialize)]
pub struct FreeProvince {
    /// 省份ID
    pub province_id: i32,
}

#[derive(Debug, Serialize)]
pub struct PddGoodsLogisticsTemplateCreateRequest {
    /// 非包邮模版列表
    pub cost_template_list: Vec<CostTemplate>,
    /// 包邮地区
    pub free_province_list: Vec<FreeProvince>,
    /// 计费方式，0-按件计费，1-按重量计费
    pub cost_type: i32,
    /// 运费模板名称
    pub template_name: String,
    /// 发货地省份id
    pub province_id: i32,
    /// 发货地城市id
    pub city_id: i32,
    /// 发货地区id
    pub district_id: i32,
}

impl RequestType for PddGoodsLogisticsTemplateCreateRequest {
    type Response = PddGoodsLogisticsTemplateCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.logistics.template.create"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsLogisticsTemplateCreateResponse {
    /// 模版id
    #[serde(default)]
    pub template_id: i64,
}
