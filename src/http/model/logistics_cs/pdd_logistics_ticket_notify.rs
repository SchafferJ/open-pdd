//! # 快递公司处理结果回调接口
//!
//! 快递公司处理结果回调
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsTicketNotifyRequest {
    /// 附件url,示例：["http://testimg.yangkeduo.com/pdd_oms/2018-01-16/411068e948835ae053a86c13f8ebb5ee.jpg"]
    pub attach_path_list: Option<Vec<String>>,
    /// 赔付金额(单位:分)
    pub compensate_amount: i64,
    /// 是否赔付，0:默认,1:未赔付,2:已赔付
    pub compensate_state: i32,
    /// 责任方，0:默认, 1:消费者,2:商家,3:快递公司,4:其他
    pub duty: Option<i32>,
    /// 处理人
    pub express_dealer: Option<String>,
    /// 处理人联系方式
    pub express_dealer_contact: Option<String>,
    /// 处理结果
    pub handle_result: String,
    /// 电联结果，当reply_type=2时，为必填项，同时该字段的值为一个json格式的字符串，格式参考示例 示例：{"call_result":1,"call_timestamp":"1688283125000","caller_name":"小张","caller_contract":"15067188888"} call_result:电联结果（int,必填） 取值如下：1：停机/空号、2：电话占线、3：无人接听、4：接通后非本人、5：接通但反馈没投诉过 call_timestamp:电联时间戳（long ,必填） caller_name:回拨人姓名（String,必填） caller_contract:回拨联系方式（String,必填）
    pub reply_call_result: Option<String>,
    /// 回复类型，1:回复工单处理结果，2:回复电联结果，当为空时，默认赋值1
    pub reply_type: Option<i32>,
    /// 签收状态，0:默认,1:未签收,2:已签收
    pub sign_state: i32,
    /// 工单id
    pub ticket_id: i64,
    /// 运单号(可为空字符串)
    pub waybill_no: String,
}

impl RequestType for PddLogisticsTicketNotifyRequest {
    type Response = PddLogisticsTicketNotifyResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.ticket.notify"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsTicketNotifyResponse {
    /// 是否修改成功
    #[serde(default)]
    pub is_success: bool,
}
