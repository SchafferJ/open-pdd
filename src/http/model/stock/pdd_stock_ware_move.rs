//! # 家电分仓库存-库存信息调整
//!
//! 家电分仓库存-库存信息调整
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct StockMoveOrderActionDto {
    /// 调整方向。1, "入库"；2, "出库"；3, "库存同步"
    pub move_direction: i32,
    /// 调整单备注
    pub order_note: Option<String>,
    /// 业务类型。1, "采购"；2, "调拨"；3, "退货"；4, "盘点"；5, "发货"；6, "库存同步"
    pub business_type: i32,
    /// 仓库编码
    pub warehouse_sn: String,
    /// 调整时间
    pub move_time: i64,
    /// 调整单号
    pub move_order_sn: String,
}

#[derive(Debug, Serialize)]
pub struct StockMoveRecordActionDto {
    /// 备注
    pub note: Option<String>,
    /// 调整数量
    pub move_num: i64,
    /// 货品sn
    pub ware_sn: String,
}

#[derive(Debug, Serialize)]
pub struct PddStockWareMoveRequest {
    pub stock_move_order_action_dto: StockMoveOrderActionDto,
    /// List<JsonObject>的json string, 一次传入StockMoveRecordActionDTO list size不超过30个
    pub stock_move_record_action_dto_list: Vec<StockMoveRecordActionDto>,
}

impl RequestType for PddStockWareMoveRequest {
    type Response = PddStockWareMoveResponse;

    fn get_type(&self) -> &'static str {
        "pdd.stock.ware.move"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddStockWareMoveResponse;
