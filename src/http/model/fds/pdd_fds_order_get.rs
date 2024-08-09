//! # 订单详情
//!
//! 收到分配，更新地址消息，使用该接口拉取订单详情
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ParamFdsOrderGetRequest {
    /// 代打店铺id
    pub mall_mask_id: String,
    /// 代打订单号
    pub order_mask_sn: String,
}

#[derive(Debug, Serialize)]
pub struct PddFdsOrderGetRequest {
    /// 入参信息
    pub param_fds_order_get_request: ParamFdsOrderGetRequest,
}

impl RequestType for PddFdsOrderGetRequest {
    type Response = PddFdsOrderGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.fds.order.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddFdsOrderGetResponse {
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
    /// 商家设置的sku编码
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
    /// 加价发顺丰0：不是 1：是
    #[serde(default)]
    pub sf_only: i32,
    /// 分配状态 0：取消分配 1：已分配
    #[serde(default)]
    pub status: i32,
    /// 售后状态 0:初始化;1:待商家处理;2:待分配;3:待客服处理;4:退款中;5:退款成功;6:已撤销;7:客服驳回;9:商家拒绝，待用户处理;10:已同意退货退款;11:待商家处理;12:售后单失败;14:换货补寄待商家处理;15:换货补寄待用户处理;16:换货补寄成功;17:换货补寄失败;18:换货补寄待用户确认完成;21:待商家同意维修;22:待用户确认发货;24:维修关闭;25:维修成功;27:待用户确认收货;31:已同意拒收退款，待用户拒收;32:补寄待商家发货;
    #[serde(default)]
    pub after_sales_status: i32,
}
