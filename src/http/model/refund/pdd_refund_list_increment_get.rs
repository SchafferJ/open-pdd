//! # 售后列表接口
//!
//! 售后列表增量查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddRefundListIncrementGetRequest {
    /// 必填，售后状态 0：无售后 2：买家申请退款，待商家处理 3：退货退款，待商家处理 4：商家同意退款，退款中 5：平台同意退款，退款中 6：驳回退款，待买家处理 7：已同意退货退款,待用户发货 8：平台处理中 9：平台拒绝退款，退款关闭 10：退款成功 11：买家撤销 12：买家逾期未处理，退款失败 13：买家逾期，超过有效期 14：换货补寄待商家处理 15：换货补寄待用户处理 16：换货补寄成功 17：换货补寄失败 18：换货补寄待用户确认完成 21：待商家同意维修 22：待用户确认发货 24：维修关闭 25：维修成功 27：待用户确认收货 31：已同意拒收退款，待用户拒收 32：补寄待商家发货 33：待商家召回
    pub after_sales_status: i32,
    /// 必填，售后类型 1：全部 2：仅退款 3：退货退款 4：换货 5：缺货补寄 6：维修
    pub after_sales_type: i32,
    /// 必填，最后更新时间结束时间的UNIX时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时00 分 00 秒)起至现在的总秒数 PS：开始时间结束时间间距不超过 30 分钟
    pub end_updated_at: i64,
    /// 订单号。若入参含订单号，则可查询订单下的全部售后单。且入参中除订单号，page，page_size外的其他查询条件不起作用（标记必填的仍旧需要输入）。
    pub order_sn: Option<String>,
    /// 返回页码 默认 1，页码从 1 开始 PS：当前采用分页返回，数量和页数会一起传，如果不传，则采用 默认值
    pub page: Option<i32>,
    /// 返回数量，默认 100。最大 100
    pub page_size: Option<i32>,
    /// 必填，最后更新时间开始时间的UNIX时间戳，指格林威治时间 1970 年01月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00分 00 秒)起至现在的总秒数
    pub start_updated_at: i64,
}

impl RequestType for PddRefundListIncrementGetRequest {
    type Response = PddRefundListIncrementGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.refund.list.increment.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Refund {
    /// 售后原因
    #[serde(default)]
    pub after_sale_reason: String,
    /// 售后状态 0：无售后 2：买家申请退款，待商家处理 3：退货退款，待商家处理 4：商家同意退款，退款中 5：平台同意退款，退款中 6：驳回退款，待买家处理 7：已同意退货退款,待用户发货 8：平台处理中 9：平台拒绝退款，退款关闭 10：退款成功 11：买家撤销 12：买家逾期未处理，退款失败 13：买家逾期，超过有效期 14：换货补寄待商家处理 15：换货补寄待用户处理 16：换货补寄成功 17：换货补寄失败 18：换货补寄待用户确认完成 21：待商家同意维修 22：待用户确认发货 24：维修关闭 25：维修成功 27：待用户确认收货 31：已同意拒收退款，待用户拒收 32：补寄待商家发货
    #[serde(default)]
    pub after_sales_status: i32,
    /// 售后类型
    #[serde(default)]
    pub after_sales_type: i32,
    /// 成团时间
    #[serde(default)]
    pub confirm_time: String,
    /// 创建时间
    #[serde(default)]
    pub created_time: String,
    /// 订单折扣金额（元）
    #[serde(default)]
    pub discount_amount: String,
    /// 1纠纷退款 0非纠纷退款
    #[serde(default)]
    pub dispute_refund_status: i32,
    /// 商品图片
    #[serde(default)]
    pub good_image: String,
    /// 商品编码
    #[serde(default)]
    pub goods_id: i64,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 商品数量
    #[serde(default)]
    pub goods_number: String,
    /// 商品单价
    #[serde(default)]
    pub goods_price: String,
    /// 售后编号
    #[serde(default)]
    pub id: i64,
    /// 订单金额（元）
    #[serde(default)]
    pub order_amount: String,
    /// 订单编号
    #[serde(default)]
    pub order_sn: String,
    /// 商家外部编码（商品）
    #[serde(default)]
    pub outer_goods_id: String,
    /// 商家外部编码（sku）
    #[serde(default)]
    pub outer_id: String,
    /// 退款金额（元）
    #[serde(default)]
    pub refund_amount: String,
    /// 同意退款操作人角色0:"默认",1:"用户",2:"商家",3:"平台",4:"系统",5:"团长",6:"司机",7:"代理人"
    #[serde(default)]
    pub refund_operator_role: i32,
    /// 退货物流公司名称
    #[serde(default)]
    pub shipping_name: String,
    /// 商品规格ID
    #[serde(default)]
    pub sku_id: String,
    /// 极速退款标志位 1：极速退款，0：非极速退款
    #[serde(default)]
    pub speed_refund_flag: i32,
    /// 极速退款状态，"1"：有极速退款资格，"2"：极速退款失败, "3" 表示极速退款成功，其他表示非极速退款
    #[serde(default)]
    pub speed_refund_status: String,
    /// 快递运单号
    #[serde(default)]
    pub tracking_number: String,
    /// 更新时间
    #[serde(default)]
    pub updated_time: String,
    /// 0-未勾选 1-消费者选择的收货状态为未收到货 2-消费者选择的收货状态为已收到货
    #[serde(default)]
    pub user_shipping_status: String,
}

#[derive(Debug, Deserialize)]
pub struct PddRefundListIncrementGetResponse {
    /// 售后列表对象
    #[serde(default)]
    pub refund_list: Option<Vec<Refund>>,
    /// 返回的售后订单列表总数
    #[serde(default)]
    pub total_count: i32,
}
