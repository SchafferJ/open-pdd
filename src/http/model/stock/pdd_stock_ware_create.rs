//! # 创建货品
//!
//! 家电分仓库存-创建货品
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct WareInfos {
    pub ware_quantity: i32,
    pub ware_id: i64,
}

#[derive(Debug, Serialize)]
pub struct PddStockWareCreateRequest {
    /// 类型 0:单独货品。1:组合货品
    pub ware_type: i32,
    /// 组合货品中子货品的关联关系, ware_type为1时必填；
    pub ware_infos: Option<Vec<WareInfos>>,
    /// 货品编码
    pub ware_sn: String,
    /// 货品名称
    pub ware_name: String,
    /// 备注
    pub note: Option<String>,
    /// 高低值服务
    pub service_quality: Option<i32>,
    /// 体积：立方毫米，只精确到100（即：最末两位为0）
    pub volume: Option<i32>,
    /// 长：毫米，精确到1
    pub length: Option<i32>,
    /// 宽：毫米，精确到1
    pub width: Option<i32>,
    /// 高：毫米，精确到1
    pub height: Option<i32>,
    /// 重量：g，精确到10（即：末位为0）
    pub weight: i32,
    /// 毛重：g，精确到10（即：末位为0）
    pub gross_weight: Option<i32>,
    /// 净重：g，精确到10（即：末位为0）
    pub net_weight: Option<i32>,
    /// 皮重：g，精确到10（即：末位为0）
    pub tare_weight: Option<i32>,
    /// 单价：分，精确到10（即：末位为0）
    pub price: Option<i32>,
    /// 颜色
    pub color: Option<String>,
    /// 包材
    pub packing: Option<String>,
}

impl RequestType for PddStockWareCreateRequest {
    type Response = PddStockWareCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.ware.create"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddStockWareCreateResponse;
