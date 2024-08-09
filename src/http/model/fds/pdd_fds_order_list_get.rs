//! # 根据更新时间查询订单列表
//!
//! 厂家首次接入ISV时，同步商家分配给厂家历史订单列表，最多支持同步近一个月数据
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ParamFdsOrderListGetRequest {
    /// 必填，更新时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总毫秒数 PS：开始时间结束时间间距不超过半小时
    pub end_updated_time: i64,
    /// 返回页码，页码从 1 开始 PS：当前采用分页返回，数量和页数会一起传
    pub page: i32,
    /// 返回数量，最大 100
    pub page_size: i32,
    /// 必填，更新时间开始时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总毫秒数 PS：开始时间结束时间间距不超过半小时
    pub start_updated_time: i64,
}

#[derive(Debug, Serialize)]
pub struct PddFdsOrderListGetRequest {
    /// 入参信息
    pub param_fds_order_list_get_request: ParamFdsOrderListGetRequest,
}

impl RequestType for PddFdsOrderListGetRequest {
    type Response = PddFdsOrderListGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.fds.order.list.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Order {
    /// 售后状态 0:初始化;1:待商家处理;2:待分配;3:待客服处理;4:退款中;5:退款成功;6:已撤销;7:客服驳回;9:商家拒绝，待用户处理;10:已同意退货退款;11:待商家处理;12:售后单失败;14:换货补寄待商家处理;15:换货补寄待用户处理;16:换货补寄成功;17:换货补寄失败;18:换货补寄待用户确认完成;21:待商家同意维修;22:待用户确认发货;24:维修关闭;25:维修成功;27:待用户确认收货;31:已同意拒收退款，待用户拒收;32:补寄待商家发货;
    #[serde(default)]
    pub after_sales_status: i32,
    /// 分配时间,毫秒
    #[serde(default)]
    pub allow_time: i64,
    /// 市
    #[serde(default)]
    pub city: String,
    /// 区
    #[serde(default)]
    pub district: String,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 商品数量
    #[serde(default)]
    pub goods_number: i32,
    /// 规格
    #[serde(default)]
    pub goods_spec: String,
    /// 代打店铺id
    #[serde(default)]
    pub mall_mask_id: String,
    /// 代打店铺自定义名称
    #[serde(default)]
    pub mall_mask_name: String,
    /// 代打订单号
    #[serde(default)]
    pub order_mask_sn: String,
    /// 商家配置的sku编码
    #[serde(default)]
    pub out_sku_sn: String,
    /// 结算价格，单位：分
    #[serde(default)]
    pub product_price: i32,
    /// 货号
    #[serde(default)]
    pub product_sn: String,
    /// 省
    #[serde(default)]
    pub province: String,
    /// 收件人姓名+电话+地址相同,receiver_id字段相同，该功能上线前字段为""
    #[serde(default)]
    pub receiver_id: String,
    /// 卖家备注
    #[serde(default)]
    pub remark: String,
    /// 运单回传状态 0：未回传 1：已回传
    #[serde(default)]
    pub return_status: i32,
    /// 加价发顺丰状态 0：不是 1：是
    #[serde(default)]
    pub sf_only: i32,
    /// 分配状态 0：取消分配 1：已分配
    #[serde(default)]
    pub status: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddFdsOrderListGetResponse {
    /// 订单列表
    #[serde(default)]
    pub order_list: Option<Vec<Order>>,
    /// 订单总数
    #[serde(default)]
    pub total: i32,
}
