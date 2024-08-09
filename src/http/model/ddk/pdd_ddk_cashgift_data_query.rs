//! # 查询多多礼金效果数据
//!
//! 查询多多礼金效果数据
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkCashgiftDataQueryRequest {
    /// 礼金ID，支持根据礼金ID查询
    pub cash_gift_id: Option<i64>,
    /// 礼金创建结束时间，查询该时间段内创建的所有礼金效果数据（礼金维度）。note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    pub end_time: Option<i64>,
    /// 分页数
    pub page: Option<i32>,
    /// 每页结果数
    pub page_size: Option<i32>,
    /// 礼金创建起始时间，查询该时间段内创建的所有礼金效果数据（礼金维度）。note：此时间为时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    pub start_time: Option<i64>,
}

impl RequestType for PddDdkCashgiftDataQueryRequest {
    type Response = PddDdkCashgiftDataQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.cashgift.data.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct GoodsInfo {
    /// 商品优惠券面额，单位为分
    #[serde(default)]
    pub coupon_discount: i64,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 商品原价，单位为分
    #[serde(default)]
    pub goods_price: i64,
    /// 商品goodsSign，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    #[serde(default)]
    pub goods_sign: String,
    /// 商品佣金比例，千分比
    #[serde(default)]
    pub rate: i32,
}

#[derive(Debug, Deserialize)]
pub struct List {
    /// 礼金券创建总金额，单位为分
    #[serde(default)]
    pub amount: i64,
    /// 礼金ID
    #[serde(default)]
    pub cash_gift_id: i64,
    /// 礼金名称
    #[serde(default)]
    pub cash_gift_name: String,
    /// 礼金券面额
    #[serde(default, rename = "couponAmount")]
    pub coupon_amount: i32,
    /// 已领取礼金券总金额，单位为分（实时数据）
    #[serde(default)]
    pub fetch_amount: i64,
    /// 已领取礼金券数量（实时数据）
    #[serde(default)]
    pub fetch_quantity: i32,
    /// 商品列表信息
    #[serde(default)]
    pub goods_info_list: Option<Vec<GoodsInfo>>,
    /// 礼金订单使用的券总金额，单位为分（实时数据）
    #[serde(default)]
    pub order_coupon_amount: i64,
    /// 礼金订单产生的总GMV，单位为分（实时数据）
    #[serde(default)]
    pub order_gmv: i64,
    /// 礼金订单数量（实时数据）
    #[serde(default)]
    pub order_quantity: i32,
    /// 礼金订单预估佣金，单位为分（实时数据）
    #[serde(default)]
    pub pre_promotion_amount: i64,
    /// 礼金券创建总数量
    #[serde(default)]
    pub quantity: i32,
    /// 退回礼金券总金额，单位为分
    #[serde(default)]
    pub refund_amount: i64,
    /// 退回礼金券数量
    #[serde(default)]
    pub refund_quantity: i32,
    /// 礼金状态：1-未生效；2-生效中；3-已过期；4-活动中止（用户主动停止）；5-活动中止（佣金降低）；6-活动中止（推广活动异常）
    #[serde(default)]
    pub status: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkCashgiftDataQueryResponse {
    /// 礼金账户余额，单位为分
    #[serde(default)]
    pub available_balance: i64,
    /// 多多礼金数据列表
    #[serde(default)]
    pub list: Option<Vec<List>>,
    /// 请求到的结果数
    #[serde(default)]
    pub total: i32,
}
