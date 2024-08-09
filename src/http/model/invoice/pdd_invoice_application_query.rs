//! # 开票申请单查询
//!
//! 当消费者在拼多多平台申请开票之后，第三方ERP通过此接口获取开票申请信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddInvoiceApplicationQueryRequest {
    /// 订单号；订单号和申请时间必填其一
    pub order_sn: Option<String>,
    /// 页码，默认1
    pub page: Option<i32>,
    /// 每页返回数目，默认50
    pub page_size: Option<i32>,
    /// 是否正品发票 0=非正品发票 1=是正品发票
    pub quality_goods_invoice: Option<i32>,
    /// 申请状态：0-已拒绝，1-申请中，2-已同意
    pub status: Option<i32>,
    /// 申请结束时间, 时间戳（单位毫秒，查询时间间隔不可超过15天）
    pub update_end_time: Option<i64>,
    /// 申请开始时间, 时间戳（单位毫秒，查询时间间隔不可超过15天）
    pub update_start_time: Option<i64>,
}

impl RequestType for PddInvoiceApplicationQueryRequest {
    type Response = PddInvoiceApplicationQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.invoice.application.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct InvoiceApplication {
    /// 申请状态：0-已拒绝，1-申请中，2-已同意
    #[serde(default)]
    pub application_status: i32,
    /// 申请时间
    #[serde(default)]
    pub apply_time: i32,
    /// 抬头类型：0-个人，1-企业
    #[serde(default)]
    pub business_type: i32,
    /// 开票金额，单位：分
    #[serde(default)]
    pub invoice_amount: String,
    /// 发票种类：0-电子，1-纸质，2-专票；目前只支持0和2
    #[serde(default)]
    pub invoice_kind: i32,
    /// 发票类型：0-蓝票，1-红票
    #[serde(default)]
    pub invoice_type: i32,
    /// 开票方式 0=手动开票,1=自动开票
    #[serde(default)]
    pub invoice_way: i32,
    /// 店铺id
    #[serde(default)]
    pub mall_id: i64,
    /// 备注
    #[serde(default)]
    pub memo: String,
    /// 订单号
    #[serde(default)]
    pub order_sn: String,
    /// （企业抬头）开户账号
    #[serde(default)]
    pub payer_account: String,
    /// （企业抬头）地址
    #[serde(default)]
    pub payer_address: String,
    /// （企业抬头）开户银行
    #[serde(default)]
    pub payer_bank: String,
    /// 发票抬头
    #[serde(default)]
    pub payer_name: String,
    /// （企业抬头）电话
    #[serde(default)]
    pub payer_phone: String,
    /// 企业税号，抬头为企业类型必填
    #[serde(default)]
    pub payer_register_no: String,
    /// 是否正品发票：0=非正品发票，1=非正品发票
    #[serde(default)]
    pub quality_goods_invoice: i32,
    /// 驳回原因
    #[serde(default)]
    pub reason: String,
    /// 不含税金额，暂为null
    #[serde(default)]
    pub sum_price: String,
    /// 总税额，暂为null
    #[serde(default)]
    pub sum_tax: String,
    /// 税率，暂为null
    #[serde(default)]
    pub tax_rate: String,
    /// 开票申请触发类型：1-申请开票，2-改抬头
    #[serde(default)]
    pub trigger_status: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddInvoiceApplicationQueryResponse {
    /// 发票申请列表
    #[serde(default)]
    pub invoice_application_list: Option<Vec<InvoiceApplication>>,
}
