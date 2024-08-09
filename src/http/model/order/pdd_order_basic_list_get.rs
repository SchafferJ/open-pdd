//! # 订单基础信息列表查询接口（根据成交时间）
//!
//! 根据成团时间查询订单列表，只有订单基础信息，不包含消费者信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOrderBasicListGetRequest {
    /// 必填，成交时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数 PS：开始时间结束时间间距不超过 24 小时
    pub end_confirm_at: i32,
    /// 发货状态，1：待发货，2：已发货待签收，3：已签收 5：全部
    pub order_status: i32,
    /// 返回页码 默认 1，页码从 1 开始 PS：当前采用分页返回，数量和页数会一起传，如果不传，则采用 默认值
    pub page: i32,
    /// 返回数量，默认 100。最大 100
    pub page_size: i32,
    /// 售后状态 1：无售后或售后关闭，2：售后处理中，3：退款中，4： 退款成功 5：全部
    pub refund_status: i32,
    /// 必填，成交时间开始时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数
    pub start_confirm_at: i32,
    /// 订单类型 0-普通订单 ，1- 定金订单
    pub trade_type: Option<i32>,
    /// 是否启用has_next的分页方式，如果指定true,则返回的结果中不包含总记录数，但是会新增一个是否存在下一页的的字段，通过此种方式获取增量交易，效率在原有的基础上有80%的提升。
    pub use_has_next: Option<bool>,
}

impl RequestType for PddOrderBasicListGetRequest {
    type Response = PddOrderBasicListGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.basic.list.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Item {
    /// 商品数量
    #[serde(default)]
    pub goods_count: i32,
    /// 商品编码
    #[serde(default)]
    pub goods_id: String,
    /// 商品图片
    #[serde(default)]
    pub goods_img: String,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 商品单件 单价：元
    #[serde(default)]
    pub goods_price: f64,
    /// 商品规格
    #[serde(default)]
    pub goods_spec: String,
    /// 商品维度外部编码，注意：编辑商品后必须等待商品审核通过后方可生效，订单中商品信息为交易快照的商品信息。
    #[serde(default)]
    pub outer_goods_id: String,
    /// sku维度商家外部编码，注意：编辑商品后必须等待商品审核通过后方可生效，订单中商品信息为交易快照的商品信息。
    #[serde(default)]
    pub outer_id: String,
    /// 商品sku编码
    #[serde(default)]
    pub sku_id: String,
}

#[derive(Debug, Deserialize)]
pub struct StepOrderInfo {
    /// 已付定金 单位：元
    #[serde(default)]
    pub advanced_paid_fee: f64,
    /// 膨胀金额 单位：元
    #[serde(default)]
    pub step_discount_amount: f64,
    /// 分阶段已付金额 单位：元
    #[serde(default)]
    pub step_paid_fee: f64,
    /// 定金订单状态：step_trade_status 枚举：0-定金未付尾款未付、1-定金已付尾款未付、2-定金已付尾款已付
    #[serde(default)]
    pub step_trade_status: i32,
}

#[derive(Debug, Deserialize)]
pub struct Order {
    /// 商品一级分类
    #[serde(default)]
    pub cat_id_1: i64,
    /// 商品二级分类
    #[serde(default)]
    pub cat_id_2: i64,
    /// 商品三级分类
    #[serde(default)]
    pub cat_id_3: i64,
    /// 商品四级分类
    #[serde(default)]
    pub cat_id_4: i64,
    /// 成交时间
    #[serde(default)]
    pub confirm_time: String,
    /// 送货入户费用 单位：元
    #[serde(default)]
    pub delivery_home_value: f64,
    /// 送货入户并安装费用 单位：元
    #[serde(default)]
    pub delivery_install_value: f64,
    /// 折扣金额，单位：元，折扣金额=平台优惠+商家优惠+团长免单优惠金额
    #[serde(default)]
    pub discount_amount: f64,
    /// 商品金额，单位：元，商品金额=商品销售价格*商品数量-改价金额（接口暂无该字段）
    #[serde(default)]
    pub goods_amount: f64,
    /// 送货入户并安装服务 0-不支持送货，1-送货入户不安装，2-送货入户并安装
    #[serde(default)]
    pub home_delivery_type: i32,
    /// 上门安装费用 单位：元
    #[serde(default)]
    pub home_install_value: f64,
    /// 是否是抽奖订单，1-非抽奖订单，2-抽奖订单
    #[serde(default)]
    pub is_lucky_flag: i32,
    /// 订单商品列表
    #[serde(default)]
    pub item_list: Option<Vec<Item>>,
    /// 订单编号
    #[serde(default)]
    pub order_sn: String,
    /// 订单状态
    #[serde(default)]
    pub order_status: i32,
    /// 支付金额，单位：元，支付金额=商品金额-折扣金额+邮费
    #[serde(default)]
    pub pay_amount: f64,
    /// 平台优惠金额，单位：元
    #[serde(default)]
    pub platform_discount: f64,
    /// 邮费，单位：元
    #[serde(default)]
    pub postage: f64,
    /// 售后状态
    #[serde(default)]
    pub refund_status: i32,
    /// 订单审核状态（0-正常订单， 1-审核中订单）
    #[serde(default)]
    pub risk_control_status: i32,
    /// 商家优惠金额，单位：元
    #[serde(default)]
    pub seller_discount: f64,
    /// { "step_discount_amount":4, "advanced_paid_fee":1, "step_paid_fee":1.1, "step_trade_status":2 }
    #[serde(default)]
    pub step_order_info: Option<StepOrderInfo>,
    /// 订单类型 0-普通订单 ，1- 定金订单
    #[serde(default)]
    pub trade_type: i32,
    /// 订单的更新时间
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct PddOrderBasicListGetResponse {
    /// 是否存在下一页
    #[serde(default)]
    pub has_next: bool,
    /// 订单信息列表
    #[serde(default)]
    pub order_list: Option<Vec<Order>>,
    /// 订单总数
    #[serde(default)]
    pub total_count: i32,
}
