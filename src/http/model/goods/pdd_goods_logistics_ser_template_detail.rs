//! # 商品送装服务模版详情
//!
//! 商品送装服务模版详情
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsLogisticsSerTemplateDetailRequest {
    /// 模版id
    pub template_id: String,
}

impl RequestType for PddGoodsLogisticsSerTemplateDetailRequest {
    type Response = PddGoodsLogisticsSerTemplateDetailResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.logistics.ser.template.detail"
    }
}

#[derive(Debug, Deserialize)]
pub struct Content {
    /// 属性区间大值，-1表示"其他"示例：要配置区间"100-200"的费用，则maxPro输入"200"要配置区间"其他"的费用，则maxPro输入"-1"备注：表示长度时单位为：mm
    #[serde(default)]
    pub max_pro: i64,
    /// 属性区间小值，-1表示"其他"示例：要配置区间"100-200"的费用，则minPro输入"100"要配置区间"其他"的费用，则minPro输入"-1"备注：表示长度时单位为：mm
    #[serde(default)]
    pub min_pro: i64,
    /// 价格。单位：分
    #[serde(default)]
    pub price: i64,
}

#[derive(Debug, Deserialize)]
pub struct List {
    /// 按属性收取费用时配置的内容：按属性限价时设置；按件限价时不用传，value和content必须设置一个
    #[serde(default)]
    pub content: Option<Vec<Content>>,
    /// 分类目收取服务费用方式：1-按件收取费用，2-按属性收取费用
    #[serde(default)]
    pub limit_type: i64,
    /// mms商品编辑页的展示字段
    #[serde(default)]
    pub mms_view: String,
    /// 按件收取费用时配置的内容：按件限价时输入数字，表示每件商品附加费用$value分；按属性限价时不用传value和content必须设置一个
    #[serde(default)]
    pub value: i64,
}

#[derive(Debug, Deserialize)]
pub struct Cat {
    /// 三级类目id
    #[serde(default)]
    pub cat_id3: i64,
    /// 四级类目id
    #[serde(default)]
    pub cat_id4: i64,
    /// 三级类目名称
    #[serde(default)]
    pub cat_name3: String,
    /// 四级类目名称
    #[serde(default)]
    pub cat_name4: String,
    /// 类目规则配置
    #[serde(default)]
    pub list: Option<Vec<List>>,
}

#[derive(Debug, Deserialize)]
pub struct ServiceArea {
    /// 市id，如果是全省选中，则市id为0
    #[serde(default)]
    pub city_id: i32,
    /// 区id，如果是全省或全市选中，则区id为0
    #[serde(default)]
    pub district_id: i32,
    /// 省id
    #[serde(default)]
    pub province_id: i32,
    /// 溢价值：按分溢价时，输入价格分的数字，表示value分；按百分比溢价时，输入0-500之间的数字，表示0%——500%备注：买家自提服务类型时，不用传该字段
    #[serde(default)]
    pub value: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsLogisticsSerTemplateDetailResponse {
    /// 分类目基础价格配置
    #[serde(default)]
    pub cat_list: Option<Vec<Cat>>,
    /// 店铺id
    #[serde(default)]
    pub mall_id: i64,
    /// 分地区配置溢价时使用的计价单位：0：价格分，按价格分计算费用；1：百分比，按照基础价格乘以百分比计算费用，除了"买家自提"服务，其他服务类型必传
    #[serde(default)]
    pub price_unit: i32,
    /// 服务地区范围配置
    #[serde(default)]
    pub service_area_list: Option<Vec<ServiceArea>>,
    /// 模版id
    #[serde(default)]
    pub template_id: String,
    /// 服务模板名称（不超过50字）
    #[serde(default)]
    pub template_name: String,
    /// 服务模板类型：2：送货上门 3：送货上门并安装 4：上门安装 5： 买家自提
    #[serde(default)]
    pub template_type: i32,
}
