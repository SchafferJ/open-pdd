//! # 合并发货订单分组
//!
//! 合并发货订单分组。用于订单合并发货分组，确保相同收货信息的订单合并到相同组。 ①.普通订单相同open_address_id可直接合单，不同open_address_id可调用本接口进行判定。同一天的订单open_address_id不同不可合单，无需再调用本接口 ②.集运订单可直接调用本接口进行判定 ③.收件地址发生更新后需重新调用本接口。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Order {
    /// 订单号
    pub order_sn: String,
}

#[derive(Debug, Serialize)]
pub struct PddOrderMergeShipOrderGroupRequest {
    /// 订单列表，最多支持100个
    pub order_list: Vec<Order>,
}

impl RequestType for PddOrderMergeShipOrderGroupRequest {
    type Response = PddOrderMergeShipOrderGroupResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.merge.ship.order.group"
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderGroup {
    /// 可以合并发货的订单号列表。不同列表之间不能合并发货
    #[serde(default)]
    pub order_sn_list: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PddOrderMergeShipOrderGroupResponse {
    /// 订单分组结果
    #[serde(default)]
    pub order_group_list: Option<Vec<OrderGroup>>,
}
