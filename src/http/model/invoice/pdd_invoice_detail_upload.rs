//! # 开票结果回传
//!
//! 第三方ERP在外部开票系统开完发票之后可以调用此接口回传开票结果
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct InvoiceItem {
    /// 开票金额 单位:分
    pub invoice_amount: i64,
    /// 发票代码
    pub invoice_code: Option<String>,
    /// 发票内容，pdf文件(电票回传)，图片文件(专票回传)，转码base64编码
    pub invoice_file_content: String,
    /// 发票号码
    pub invoice_no: String,
    /// 原蓝票代码（红票必填）
    pub original_invoice_code: Option<String>,
    /// 原蓝票号码（红票必填）
    pub original_invoice_no: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PddInvoiceDetailUploadRequest {
    /// 申请流水号
    pub application_id: Option<i64>,
    /// 抬头类型：0-个人，1-企业
    pub business_type: i32,
    /// 开票金额，整数，单位：分
    pub invoice_amount: Option<i64>,
    /// 发票代码
    pub invoice_code: Option<String>,
    /// 发票内容，pdf文件(电票回传)，图片文件(专票回传)，转码base64编码
    pub invoice_file_content: Option<String>,
    /// 多张发票列表（如果本字段为空，invoice_code、invoice_no、invoice_amount、invoice_file_content这四个字段必须填写）
    pub invoice_item_list: Option<Vec<InvoiceItem>>,
    /// 发票种类：0-电子发票，1-纸质发票，2-专票；目前只支持0
    pub invoice_kind: i32,
    /// 发票号码
    pub invoice_no: Option<String>,
    /// 开票日期,时间戳（毫秒）
    pub invoice_time: i64,
    /// 开票类型：0-蓝票，1-红票；目前 只支持0
    pub invoice_type: i32,
    /// 备注
    pub memo: Option<String>,
    /// 订单号
    pub order_sn: String,
    /// 原蓝票代码（红票必填）
    pub original_invoice_code: Option<String>,
    /// 原蓝票号码（红票必填）
    pub original_invoice_no: Option<String>,
    /// 专票回传必填，专票邮寄快递公司编码，见https://open.pinduoduo.com/application/document/api?id=pdd.logistics.companies.get返回的快递公司编码
    pub paper_shipping_id: Option<i32>,
    /// 专票回传必填，专票邮寄运单号
    pub paper_tracking_number: Option<String>,
    /// 开票人
    pub payee_operator: String,
    /// （企业抬头）开户账号
    pub payer_account: Option<String>,
    /// （企业抬头）地址
    pub payer_address: Option<String>,
    /// （企业抬头）开户银行
    pub payer_bank: Option<String>,
    /// 发票抬头
    pub payer_name: String,
    /// （企业抬头）电话
    pub payer_phone: Option<String>,
    /// 税号，企业必填
    pub payer_register_no: Option<String>,
    /// 不含税金额，整数，单位：分
    pub sum_price: i64,
    /// 总税额，整数，单位：分
    pub sum_tax: i32,
    /// 税率,整数
    pub tax_rate: i32,
}

impl RequestType for PddInvoiceDetailUploadRequest {
    type Response = PddInvoiceDetailUploadResponse;

    fn get_type(&self) -> &'static str {
        "pdd.invoice.detail.upload"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddInvoiceDetailUploadResponse {
    /// 发票流水号
    #[serde(default)]
    pub serial_no: String,
}
