//! # 类目商品发布规则查询接口
//!
//! 通过叶子类目id获取该类目的发布规则，目前返回标品、商品服务、属性等规则。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCatRuleGetRequest {
    /// 类目id
    pub cat_id: i64,
    /// 商品id，编辑的时候需要传被编辑的商品id，发布商品时如果已有商品id也需要传
    pub goods_id: Option<i64>,
}

impl RequestType for PddGoodsCatRuleGetRequest {
    type Response = PddGoodsCatRuleGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.cat.rule.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct ShowCondition {
    /// 父属性id。
    #[serde(default)]
    pub parent_ref_pid: i64,
    /// 父属性值id。多个值任选其一即可。若为空表示任意值都可以。
    #[serde(default)]
    pub parent_vids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    /// 组id
    #[serde(default)]
    pub id: i32,
    /// 组名称
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Values {
    /// 扩展信息，表示颜色的色号。格式为ARGB
    #[serde(default)]
    pub extend_info: String,
    /// 分组信息
    #[serde(default)]
    pub group: Option<Group>,
    /// 表示对应父属性值id。当其中父属性值被选中时该子属性值才可选。为空则表示无此限制。
    #[serde(default)]
    pub parent_vids: Vec<i64>,
    /// 规格id，发商品时需要和sku上的spec对应。
    #[serde(default)]
    pub spec_id: i64,
    /// 属性值
    #[serde(default)]
    pub value: String,
    /// 基础属性值id
    #[serde(default)]
    pub vid: i64,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    /// 是否允许填写备注，仅当是销售属性时有意义
    #[serde(default)]
    pub can_note: bool,
    /// 可选择属性值数目，为0时代表不限。包括自定义的属性值和模版中给出的属性值。
    #[serde(default)]
    pub choose_max_num: i32,
    /// 可自定义属性值数目，为0时代表不可自定义。
    #[serde(default)]
    pub input_max_num: i32,
    /// 是否重要属性。填写重要属性有更多机会获取搜索、活动等场景流量。
    #[serde(default)]
    pub is_important: bool,
    /// 是否销售属性。销售属性需要在发商品时，商品属性上的属性值与规格中中的spec对应。
    #[serde(default)]
    pub is_sale: bool,
    /// 是否sku属性，sku维度的属性在商品发布时入参在sku对象下
    #[serde(default)]
    pub is_sku: bool,
    /// 最大值。在不同的属性值类型下有不同的含义。  文本类型时，代表文本最大长度；  数值类型时，代表数字最大值；  时间类型且最大值为时间时，代表时间最大值；  时间类型且最大值为数字时，代表距离今天或者本月往后的天数或月数。
    #[serde(default)]
    pub max_value: String,
    /// 最小值。在不同的属性值类型下有不同的含义。  文本类型时，代表文本最小长度；  数值类型时，代表数字最小值；  时间类型且最小值为时间时，代表时间最小值；  时间类型且最小值为数字时，代表距离今天或者本月往前的天数或月数。
    #[serde(default)]
    pub min_value: String,
    /// 属性名称
    #[serde(default)]
    pub name: String,
    /// 销售属性对应的父规格id。
    #[serde(default)]
    pub parent_spec_id: i64,
    /// 属性值类型。在发商品时传自定义的属性值时，有不同的格式。 0=文本; 1=数值，如“100”; 2=数值范围，如“10,20”，表示10到20之间; 3=数值乘积-二维，如“10,10”，表示10*10; 4=数值乘积-三维，如“10,10,10”，表示10*10*10; 5=单项时间选择-年月日，如“2020-05-20”; 6=双项时间选择-年月日，如“2020-05-20,2020-06-20”; 7=单项时间选择-年月，如“2020-05”; 8=双项时间选择-年月，如“2020-05,2020-06”。
    #[serde(default)]
    pub property_value_type: i32,
    /// 属性id
    #[serde(default)]
    pub ref_pid: i64,
    /// 是否必填
    #[serde(default)]
    pub required: bool,
    /// 必填规则具体内容，当required=true且required_rule_type=1时该字段有效，表示当出现哪些属性/属性值时，该属性需要必填，json格式，两层List结构，外层list代表"或"关系，内层list代表"且"关系，例如：[[{\"ref_pid\":123,\"vid\":123}]]
    #[serde(default)]
    pub required_rule: String,
    /// required=true时，该字段有效，为“0”时表示当前属性必填，不受其他属性影响，为“1”时表示当前属性是否必填由已填写的其他属性值决定
    #[serde(default)]
    pub required_rule_type: i32,
    /// 该属性的父属性。只有parent_pid下的show_vids中的值被选择时才可入参该属性。有多组父属性时，为且的关系。
    #[serde(default)]
    pub show_condition: Option<Vec<ShowCondition>>,
    /// 小数点允许最大精度，为0时代表不允许输入小数。对数值类属性值限制。
    #[serde(default)]
    pub value_precision: i32,
    /// 可选属性值单位，发商品填写自定义数值属性值时，选择其中之一作为单位。
    #[serde(default)]
    pub value_unit: Vec<String>,
    /// 属性值列表
    #[serde(default)]
    pub values: Option<Vec<Values>>,
}

#[derive(Debug, Deserialize)]
pub struct GoodsPropertiesRule {
    /// 多个销售属性是否需要同时传
    #[serde(default)]
    pub choose_all_qualify_spec: bool,
    /// 允许自定义的销售属性数量
    #[serde(default)]
    pub input_max_spec_num: i64,
    /// 属性信息
    #[serde(default)]
    pub properties: Option<Vec<Properties>>,
}

#[derive(Debug, Deserialize)]
pub struct GoodsServiceRule {
    #[serde(default)]
    pub goods_service_rule_map: std::collections::BTreeMap<String, serde_json::Value>,
    /// 可选的商品类型列表
    #[serde(default)]
    pub goods_type_list: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GoodsSkuRule {
    /// 团购价最高差倍率
    #[serde(default)]
    pub price_range_ratio: f64,
    /// 同个商品下规格值的加和数量上限
    #[serde(default)]
    pub spec_num_limit: i32,
}

#[derive(Debug, Deserialize)]
pub struct KeyProp {
    /// 关键属性名
    #[serde(default)]
    pub pname: String,
    /// 关键属性id
    #[serde(default)]
    pub ref_pid: i64,
}

#[derive(Debug, Deserialize)]
pub struct SpuRule {
    /// 标品管控类型。0=不管控；1=管控，表示商品发布时必须命中标品，且发布成功后不可更改关键属性；2=不可换品，表示发布成功后不可更改关键属性。
    #[serde(default)]
    pub control_type: i32,
    /// 关键属性
    #[serde(default)]
    pub key_prop: Option<Vec<KeyProp>>,
}

#[derive(Debug, Deserialize)]
pub struct TwoPiecesDiscountRule {
    /// 是否必须设置
    #[serde(default)]
    pub if_must_two_pieces_discount: bool,
    /// 允许的最大折扣
    #[serde(default)]
    pub max_two_pieces_discount: i32,
    /// 允许的最小折扣
    #[serde(default)]
    pub min_two_pieces_discount: i32,
    /// 推荐的折扣
    #[serde(default)]
    pub recommend_two_pieces_discount: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCatRuleGetResponse {
    /// 商品属性规则
    #[serde(default)]
    pub goods_properties_rule: Option<GoodsPropertiesRule>,
    /// 商品服务规则
    #[serde(default)]
    pub goods_service_rule: Option<GoodsServiceRule>,
    /// sku规则
    #[serde(default)]
    pub goods_sku_rule: Option<GoodsSkuRule>,
    /// 标品规则
    #[serde(default)]
    pub spu_rule: Option<SpuRule>,
    /// 满2件折扣相关规则
    #[serde(default)]
    pub two_pieces_discount_rule: Option<TwoPiecesDiscountRule>,
}
