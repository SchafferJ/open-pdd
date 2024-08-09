//! # 旅游门票订单创建异步回调接口
//!
//! 供应商向拼多多进行创单回调请求
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Tickets {
    /// 辅助凭证，有辅助凭证时返回
    pub additional: Option<String>,
    /// 主凭证，code_type=2时返回
    pub code: Option<String>,
    /// 文件base64流，code_type=3时返回，大小小于800KB
    pub file: Option<String>,
    /// 外链，code_type=4时返回
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PddTicketOrderCreateNotifycationRequest {
    /// 码类型。status=2时必填。1.无凭证(身份证/手机号) 2. 数字码 3.QR图片 4.外链
    pub code_type: Option<i32>,
    /// 失败错误码。status=3时必填
    pub failed_code: Option<i32>,
    /// 失败原因。 status=3时必填
    pub failed_reason: Option<String>,
    /// 拼多多制票号
    pub order_no: String,
    /// isv订单号
    pub out_order_sn: String,
    /// 制码状态。 2.制作成功 3.制作失败
    pub status: i32,
    /// 凭证信息列表。status=2 且 code_type!=1 时必填
    pub tickets: Option<Vec<Tickets>>,
    /// 凭证类型。status=2时必填。1.一人一码 2.一人多码
    pub ticket_type: Option<i32>,
}

impl RequestType for PddTicketOrderCreateNotifycationRequest {
    type Response = PddTicketOrderCreateNotifycationResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ticket.order.create.notifycation"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddTicketOrderCreateNotifycationResponse {
    #[serde(default)]
    pub success: bool,
}
