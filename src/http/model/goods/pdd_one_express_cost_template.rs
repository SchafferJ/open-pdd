//! # 按id获取商品运费模版接口
//!
//! 根据id获取拼多多商家的物流运费模板信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOneExpressCostTemplateRequest {
    /// 运费模板id
    pub cost_template_id: i64,
}

impl RequestType for PddOneExpressCostTemplateRequest {
    type Response = PddOneExpressCostTemplateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.one.express.cost.template"
    }
}

#[derive(Debug, Deserialize)]
pub struct CostProvince {
    /// 省份
    #[serde(default)]
    pub province: String,
    /// 不包邮区域的ID
    #[serde(default)]
    pub province_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CostTemplate {
    /// 不包邮的区域
    #[serde(default)]
    pub cost_province_list: Option<Vec<CostProvince>>,
    /// 首件
    #[serde(default)]
    pub first_standard: i64,
    /// 首件或首重价格，单位为分
    #[serde(default)]
    pub first_cost: i64,
    /// 续重或续件
    #[serde(default)]
    pub add_standard: i64,
    /// 续件或续重价格，单位为分
    #[serde(default)]
    pub add_cost: i64,
    /// 对不包邮地区，true-若要包邮须满足件数包邮，false-不开启满足件数包邮
    #[serde(default)]
    pub is_have_free_min_count: bool,
    /// 对不包邮地区，满足指定件数包邮，该值为商家设置的指定件数，若为-1则商家没有开启满足件数包邮
    #[serde(default)]
    pub have_free_min_count: i64,
    /// 对不包邮地区，true-若要包邮须满足指定价格则可以包邮，false-不开启满足指定价格包邮
    #[serde(default)]
    pub is_have_free_min_amount: bool,
    /// 对不包邮地区，满足指定价格包邮，该值为商家设置的指定订单金额，若为-1则商家没有开启满足指定价格包邮，注意，单位为分
    #[serde(default)]
    pub have_free_min_amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct FreeDeliverHouseArea {
    /// 包邮送货上门的城区ID
    #[serde(default)]
    pub town_id: i32,
    /// 包邮送货上门的城市ID
    #[serde(default)]
    pub city_id: i32,
    /// 包邮送货上门的省份ID
    #[serde(default)]
    pub province_id: i32,
    /// 包邮送货上门的省份
    #[serde(default)]
    pub province: String,
    /// 包邮送货上门的城市
    #[serde(default)]
    pub city: String,
    /// 包邮送货上门的城区
    #[serde(default)]
    pub town: String,
}

#[derive(Debug, Deserialize)]
pub struct FreeProvince {
    /// 省份ID
    #[serde(default)]
    pub province_id: i64,
    /// 省份
    #[serde(default)]
    pub province: String,
}

#[derive(Debug, Deserialize)]
pub struct PddOneExpressCostTemplateResponse {
    /// 发货地省份ID
    #[serde(default)]
    pub province_id: i32,
    /// 发货地城市ID
    #[serde(default)]
    pub city_id: i32,
    /// 发货地区ID
    #[serde(default)]
    pub district_id: i32,
    /// 送货入户并安装服务，0-不支持、1-支持送货入户、2-支持送货入户并安装
    #[serde(default)]
    pub additional_service_type: i32,
    /// 是否顺丰包邮
    #[serde(default)]
    pub sf_free_type: i32,
    /// 不包邮区域/需要买家付邮费区域
    #[serde(default)]
    pub cost_template_list: Option<Vec<CostTemplate>>,
    /// 运费模板id
    #[serde(default)]
    pub template_id: i64,
    /// 运费模板名称
    #[serde(default)]
    pub template_name: String,
    /// 计费方式，0-按件计费，1-按重量计费
    #[serde(default)]
    pub cost_type: i32,
    /// 是否送货上门，对于包邮地区：true-商品包邮且送货上门，false-商品包邮但不送货上门
    #[serde(default)]
    pub free_deliver_house: bool,
    /// 送货上门地区列表
    #[serde(default)]
    pub free_deliver_house_area_list: Option<Vec<FreeDeliverHouseArea>>,
    /// 包邮省份对象
    #[serde(default)]
    pub free_province_list: Option<Vec<FreeProvince>>,
}
