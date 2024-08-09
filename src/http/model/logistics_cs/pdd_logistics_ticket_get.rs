//! # 快递公司工单查询接口
//!
//! 快递公司工单查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsTicketGetRequest {
    /// 必填，最后更新时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数 PS：开始时间结束时间间距不超过 30 分钟。示例：1523763012。
    pub end_updated_at: i64,
    /// 返回页码 默认 1，页码从 1 开始 PS：当前采用分页返回，数量和页数会一起传，如果不传，则采用 默认值。注：必须采用倒序的分页方式（从最后一页往回取）才能避免漏单问题
    pub page: Option<i32>,
    /// 返回数量，默认 100。最大 100
    pub page_size: Option<i32>,
    /// 必填，最后更新时间开始时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数。示例：1523762012。
    pub start_updated_at: i64,
}

impl RequestType for PddLogisticsTicketGetRequest {
    type Response = PddLogisticsTicketGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.ticket.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct ExpressAttachment {
    /// 物流商回传凭证链接
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct LogisticsTicket {
    /// 附件列表
    #[serde(default)]
    pub attach_url: Vec<String>,
    /// 末端品牌代码
    #[serde(default)]
    pub cabinet_code: String,
    /// 赔付金额(单位:分)
    #[serde(default)]
    pub compensate_amount: i64,
    /// 赔付状态 0:默认,1:未赔付,2:已赔付
    #[serde(default)]
    pub compensate_state: i32,
    /// create_type
    #[serde(default)]
    pub create_type: i32,
    /// 工单创建时间戳
    #[serde(default)]
    pub created_at: i64,
    /// 备注
    #[serde(default)]
    pub description: String,
    /// duty
    #[serde(default)]
    pub duty: i32,
    /// 物流商回传凭证
    #[serde(default)]
    pub express_attachment: Option<Vec<ExpressAttachment>>,
    /// 物流商快递编码
    #[serde(default)]
    pub express_company_id: i64,
    /// 处理人
    #[serde(default)]
    pub express_dealer: String,
    /// 处理人联系方式
    #[serde(default)]
    pub express_dealer_contact: String,
    /// 物流商处理结果
    #[serde(default)]
    pub handle_result: String,
    /// 寄件单号
    #[serde(default)]
    pub mail_order_sn: String,
    /// 订单金额
    #[serde(default)]
    pub pay_amount: i64,
    /// 订单号生成的物流单号
    #[serde(default)]
    pub pre_delivery_id: String,
    /// receive_address
    #[serde(default)]
    pub receive_address: String,
    /// 联系人电话
    #[serde(default)]
    pub receive_contact: String,
    /// 联系人姓名
    #[serde(default)]
    pub receive_name: String,
    /// 工单退回次数
    #[serde(default)]
    pub retreat_count: i64,
    /// send_address
    #[serde(default)]
    pub send_address: String,
    /// 0:默认,1:未签收,2:已签收
    #[serde(default)]
    pub sign_state: i32,
    /// 问题来源，	0:买家,1:卖家
    #[serde(default)]
    pub source: i32,
    /// 物流商回复状态，0：待回复，1：已回复
    #[serde(default)]
    pub status: i32,
    /// 物流投诉标签
    #[serde(default)]
    pub sub_type_ids: Vec<i32>,
    /// 工单id
    #[serde(default)]
    pub ticket_id: i64,
    /// 问题描述
    #[serde(default)]
    pub title: String,
    /// 问题类型id
    #[serde(default)]
    pub type_id: i64,
    /// 工单最后更新时间戳
    #[serde(default)]
    pub updated_at: i64,
    /// 紧急度，0:中,1:紧急
    #[serde(default)]
    pub urgent_type: i32,
    /// 运单号(可能为空字符串)
    #[serde(default)]
    pub waybill_no: String,
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsTicketGetResponse {
    /// 工单列表
    #[serde(default)]
    pub logistics_ticket_list: Option<Vec<LogisticsTicket>>,
    /// 列表总数
    #[serde(default)]
    pub total_count: i32,
}
