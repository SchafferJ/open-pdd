//! # 获取商品类目属性(已废弃)
//!
//! 商品发布前，需要查询该类目的商品发布需要的属性，获取商品发布需要的模板-属性-属性值。已废弃，建议使用pdd.goods.cat.rule.get代替
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCatTemplateGetRequest {
    /// 类目id
    pub cat_id: i64,
}

impl RequestType for PddGoodsCatTemplateGetRequest {
    type Response = PddGoodsCatTemplateGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.cat.template.get"
    }
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
    /// 扩展信息，颜色的话色号在这里,ARGB，非销售属性为null
    #[serde(default)]
    pub extend_info: String,
    /// 分组信息，非销售属性为null
    #[serde(default)]
    pub group: Option<Group>,
    /// 是否父属性值
    #[serde(default)]
    pub is_parent: bool,
    /// 对应的父属性值id
    #[serde(default)]
    pub parent_vids: Vec<i64>,
    /// 规格id,非销售属性为null
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
    /// 是否允许填写备注
    #[serde(default)]
    pub can_note: bool,
    /// 最大可勾选数目,为0时代表不限
    #[serde(default)]
    pub choose_max_num: i32,
    /// 控件类型（0-可输入、1-可勾选、3-可输入又可勾选、5-单项时间选择器-年月日、6-双项时间选择器-年月日、7-单项时间选择器-年月、8-双项时间选择器-年月）9-调色盘、10-尺码选择器、11-输入数值范围、12-输入数值乘积-2维、13-输入数值乘积-3维
    #[serde(default)]
    pub control_type: i32,
    /// 属性特性:0普通，1颜色，2尺码
    #[serde(default)]
    pub feature: i32,
    /// 模板属性id
    #[serde(default)]
    pub id: i64,
    /// 最大可输入数目,为0时代表不可输入，非销售属性为null
    #[serde(default)]
    pub input_max_num: i32,
    /// 是否按条件展示
    #[serde(default)]
    pub is_condition_show: bool,
    /// is_key
    #[serde(default)]
    pub is_key: bool,
    /// 是否父属性
    #[serde(default)]
    pub is_parent: bool,
    /// 是否销售属性
    #[serde(default)]
    pub is_sale: bool,
    /// 输入最大值：文本类型代表文本最长长度、 数值类型代表数字最大值、时间类型代表时间最大值
    #[serde(default)]
    pub max_value: String,
    /// 输入最小值：文本类型代表文本最小长度、数值类型代表数字最小值、时间类型代表时间最小值
    #[serde(default)]
    pub min_value: String,
    /// 属性名称
    #[serde(default)]
    pub name: String,
    /// 属性别名
    #[serde(default)]
    pub name_alias: String,
    /// 父属性id
    #[serde(default)]
    pub parent_id: i64,
    /// 引用属性id
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
    /// show_only_standard
    #[serde(default)]
    pub show_only_standard: bool,
    /// 若属性按条件展示,则只有show_vids中的值被选择时属性才可使用
    #[serde(default)]
    pub show_vids: Vec<i64>,
    /// 销售属性规格id，非销售属性为null
    #[serde(default)]
    pub spec_id: i64,
    /// 属性值列表
    #[serde(default)]
    pub values: Option<Vec<Values>>,
    /// 小数点允许最大精度,为0时代表不允许输入小数
    #[serde(default)]
    pub value_precision: i32,
    /// 属性值类型（0-文本、1-数值、4-绝对时间、5-相对时间）
    #[serde(default)]
    pub value_type: i32,
    /// 属性值单位
    #[serde(default)]
    pub value_unit: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCatTemplateGetResponse {
    /// 限定规格不支持部分选取，为true时限定规格要么全选要么全不选
    #[serde(default)]
    pub choose_all_qualify_spec: bool,
    /// 模板id
    #[serde(default)]
    pub id: i64,
    /// 模板允许的最大的自定义规格数量
    #[serde(default)]
    pub input_max_spec_num: i64,
    /// is_single_item
    #[serde(default)]
    pub is_single_item: bool,
    /// 最大sku数目上限
    #[serde(default)]
    pub max_sku_num: i64,
    /// 属性信息
    #[serde(default)]
    pub properties: Option<Vec<Properties>>,
    /// 单个自定义规格值上限
    #[serde(default)]
    pub single_spec_value_num: i64,
}
