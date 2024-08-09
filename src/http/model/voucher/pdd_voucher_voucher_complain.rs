//! # 卡券投诉接口
//!
//! 卡券投诉接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Voucher {
    /// 卡券ID
    pub voucher_id: Option<String>,
    /// 卡券号
    pub voucher_no: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PddVoucherVoucherComplainRequest {
    /// 订单号
    pub order_sn: String,
    /// 外部流水号
    pub out_biz_no: String,
    /// 优惠券信息列表,例子[{"voucher_id":"test voucher_id","voucher_no":"test voucher_no"}]
    pub voucher_list: Option<Vec<Voucher>>,
    /// 投诉人
    pub complain_user: String,
    /// 投诉人电话
    pub complain_user_mobile: String,
    /// 投诉内容
    pub complain_content: String,
    /// ["http://testimg.yangkeduo.com/pdd_oms/2018-01-16/411068e948835ae053a86c13f8ebb5ee.jpg"]
    pub complain_attachment_list: Vec<String>,
    /// 枚举值1、大闸蟹死蟹或者少蟹 ；2、大闸蟹重量不符；3、大闸蟹公母数量不符；4、大闸蟹产地不符；5、欺诈发货（收到的产品非大闸蟹）；6、蟹券无法提货
    /// 7、其他质量问题
    pub complain_type: i32,
}

impl RequestType for PddVoucherVoucherComplainRequest {
    type Response = PddVoucherVoucherComplainResponse;

    fn get_type(&self) -> &'static str {
        "pdd.voucher.voucher.complain"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddVoucherVoucherComplainResponse {
    /// 是否请求成功
    #[serde(default)]
    pub is_success: bool,
}
