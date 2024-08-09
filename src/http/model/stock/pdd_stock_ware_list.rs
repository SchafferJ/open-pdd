//! # 查询货品列表
//!
//! 家电分仓库存-列表查询货品
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddStockWareListRequest {
    /// 货品id
    pub id: Option<i64>,
    /// 货品编码
    pub ware_sn: Option<String>,
    /// 货品名称
    pub ware_name: Option<String>,
    /// 类型 0:单独货品。1:组合货品
    pub ware_type: Option<i32>,
    /// 页数，从1开始
    pub page: i32,
    /// 每页记录数
    pub size: i32,
}

impl RequestType for PddStockWareListRequest {
    type Response = PddStockWareListResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.ware.list"
    }
}

#[derive(Debug, Deserialize)]
pub struct WareInfos {
    /// 子货品编码
    #[serde(default)]
    pub ware_sn: String,
    /// 子货品名称
    #[serde(default)]
    pub ware_name: String,
    /// 子货品数量
    #[serde(default)]
    pub ware_quantity: i64,
    /// 子货品id
    #[serde(default)]
    pub ware_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct Specs {
    /// 规格名称
    #[serde(default)]
    pub spec_key: String,
    /// 规格值
    #[serde(default)]
    pub spec_value: String,
    /// 规格id
    #[serde(default)]
    pub spec_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct WareSkus {
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// skuid
    #[serde(default)]
    pub sku_id: i64,
    /// 货品id
    #[serde(default)]
    pub ware_id: i64,
    /// 是否已绑定货品true/false
    #[serde(default)]
    pub exist_ware: bool,
    /// 上下架状态，true为上架
    #[serde(default)]
    pub is_onsale: bool,
    /// 规格信息
    #[serde(default)]
    pub specs: Option<Vec<Specs>>,
}

#[derive(Debug, Deserialize)]
pub struct WareDetails {
    /// 货品id
    #[serde(default)]
    pub id: i64,
    /// 货品类型.0:单独货品 1:组合货品
    #[serde(default)]
    pub r#type: i32,
    /// 组合货品中子货品的关联关系
    #[serde(default)]
    pub ware_infos: Option<Vec<WareInfos>>,
    /// 货品sku信息
    #[serde(default)]
    pub ware_skus: Option<Vec<WareSkus>>,
    /// 货品编码
    #[serde(default)]
    pub ware_sn: String,
    /// 货品名称
    #[serde(default)]
    pub ware_name: String,
    /// 备注
    #[serde(default)]
    pub note: String,
    /// 高低值服务，0低，1高
    #[serde(default)]
    pub service_quality: i32,
    /// 体积：立方厘米，精确到一位小数
    #[serde(default)]
    pub volume: i32,
    /// 长：厘米，精确到一位小数
    #[serde(default)]
    pub length: i32,
    /// 宽：厘米，精确到一位小数
    #[serde(default)]
    pub width: i32,
    /// 高：厘米，精确到一位小数
    #[serde(default)]
    pub height: i32,
    /// 重量：kg，精确到两位小数
    #[serde(default)]
    pub weight: i32,
    /// 毛重：kg，精确到两位小数
    #[serde(default)]
    pub gross_weight: i32,
    /// 净重：kg，精确到两位小数
    #[serde(default)]
    pub net_weight: i32,
    /// 皮重：kg，精确到两位小数
    #[serde(default)]
    pub tare_weight: i32,
    /// 单价：元，精确到一位小数
    #[serde(default)]
    pub price: i32,
    /// 颜色
    #[serde(default)]
    pub color: String,
    /// 包材
    #[serde(default)]
    pub packing: String,
    /// 库存
    #[serde(default)]
    pub quantity: i64,
    /// 创建时间（毫秒）
    #[serde(default)]
    pub created_at: i64,
    /// 更新时间（毫秒）
    #[serde(default)]
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddStockWareListResponse {
    /// 总数
    #[serde(default)]
    pub total: i32,
    /// 货品详情
    #[serde(default)]
    pub ware_details: Option<Vec<WareDetails>>,
}
