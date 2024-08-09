//! # 查询货品详情
//!
//! 家电分仓库存-查看货品详情
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddStockWareDetailQueryRequest {
    /// 货品id
    pub ware_id: i64,
}

impl RequestType for PddStockWareDetailQueryRequest {
    type Response = PddStockWareDetailQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.ware.detail.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct WareInfos {
    /// 子货品id
    #[serde(default)]
    pub ware_id: i64,
    /// 子货品名称
    #[serde(default)]
    pub ware_name: String,
    /// 子货品数量
    #[serde(default)]
    pub ware_quantity: i64,
    /// 子货品编码
    #[serde(default)]
    pub ware_sn: String,
}

#[derive(Debug, Deserialize)]
pub struct Specs {
    /// 规格id
    #[serde(default)]
    pub spec_id: i64,
    /// 规格名称
    #[serde(default)]
    pub spec_key: String,
    /// 规格值
    #[serde(default)]
    pub spec_value: String,
}

#[derive(Debug, Deserialize)]
pub struct WareSkus {
    /// 是否已经绑定货品false/true
    #[serde(default)]
    pub exist_ware: bool,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 上下架状态，true表示上架
    #[serde(default)]
    pub is_onsale: bool,
    /// skuid
    #[serde(default)]
    pub sku_id: i64,
    /// 规格信息
    #[serde(default)]
    pub specs: Option<Vec<Specs>>,
    /// 货品id
    #[serde(default)]
    pub ware_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddStockWareDetailQueryResponse {
    /// 颜色
    #[serde(default)]
    pub color: String,
    /// 创建时间（毫秒）
    #[serde(default)]
    pub created_at: i64,
    /// 毛重：kg，精确到两位小数
    #[serde(default)]
    pub gross_weight: i32,
    /// 高：厘米，精确到一位小数
    #[serde(default)]
    pub height: i32,
    /// 货品id
    #[serde(default)]
    pub id: i64,
    /// 长：厘米，精确到一位小数
    #[serde(default)]
    pub length: i32,
    /// 净重：kg，精确到两位小数
    #[serde(default)]
    pub net_weight: i32,
    /// 备注
    #[serde(default)]
    pub note: String,
    /// 包材
    #[serde(default)]
    pub packing: String,
    /// 单价：元，精确到一位小数
    #[serde(default)]
    pub price: i32,
    /// 库存
    #[serde(default)]
    pub quantity: i64,
    /// 高低值服务，0低，1高
    #[serde(default)]
    pub service_quality: i32,
    /// 皮重：kg，精确到两位小数
    #[serde(default)]
    pub tare_weight: i32,
    /// 货品类型.0:单独货品  1:组合货品
    #[serde(default)]
    pub r#type: i32,
    /// 更新时间毫秒）
    #[serde(default)]
    pub updated_at: i64,
    /// 体积：立方厘米，精确到一位小数
    #[serde(default)]
    pub volume: i32,
    /// 组合货品中子货品的关联关系
    #[serde(default)]
    pub ware_infos: Option<Vec<WareInfos>>,
    /// 货品名称
    #[serde(default)]
    pub ware_name: String,
    /// 货品sku信息
    #[serde(default)]
    pub ware_skus: Option<Vec<WareSkus>>,
    /// 货品编码
    #[serde(default)]
    pub ware_sn: String,
    /// 重量：kg，精确到两位小数
    #[serde(default)]
    pub weight: i32,
    /// 宽：厘米，精确到一位小数
    #[serde(default)]
    pub width: i32,
}
