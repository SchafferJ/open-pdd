//! # 自动开票发票查询
//!
//! 商家使用自动开票系统对订单进行开票，可通过此接口获取30天内已开发票对应的发票和订单信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddEinvoiceInfoQueryRequest {
    /// 最后更新时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数。开始时间结束时间间距不超过1小时。不能查询最近5分钟内的数据。开区间，返回数据不包含end_time
    pub end_time: i64,
    /// 发票类型 0-蓝票，1-红票，不传为全部
    pub invoice_type: Option<i32>,
    /// 页码。页码从 1开始
    pub page: i32,
    /// 返回数量。最小1，最大 50
    pub page_size: i32,
    /// 最后更新时间开始时间的时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至现在的总秒数。只能查询30天内的数据。闭区间，返回数据包含start_time
    pub start_time: i64,
}

impl RequestType for PddEinvoiceInfoQueryRequest {
    type Response = PddEinvoiceInfoQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.einvoice.info.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct InvoiceItem {
    /// 价税合计(放大100倍,单位分)
    #[serde(default)]
    pub amount: i64,
    /// 税收分类编码
    #[serde(default)]
    pub catalog_code: String,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 不含税金额(放大100倍，单位分)
    #[serde(default)]
    pub no_tax_amount: i64,
    /// 含税单价（放大1000000倍）
    #[serde(default)]
    pub price: i64,
    /// 数量（放大1000000倍）
    #[serde(default)]
    pub quantity: i64,
    /// 规格型号
    #[serde(default)]
    pub specification: String,
    /// 税率
    #[serde(default)]
    pub tax_rate: String,
    /// 总税额(放大100倍，单位分)
    #[serde(default)]
    pub total_tax: i64,
    /// 单位
    #[serde(default)]
    pub unit: String,
    /// 零税率标识，1：免税，2：不征税，3：普通零税率
    #[serde(default)]
    pub zero_tax_rate_flag: i32,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceInfo {
    /// 购方地址
    #[serde(default)]
    pub buyer_address: String,
    /// 购方银行账号
    #[serde(default)]
    pub buyer_bank_account: String,
    /// 购方银行名称
    #[serde(default)]
    pub buyer_bank_name: String,
    /// 购方名称(发票抬头)
    #[serde(default)]
    pub buyer_name: String,
    /// 购方电话
    #[serde(default)]
    pub buyer_phone_number: String,
    /// 购方税号
    #[serde(default)]
    pub buyer_tax_no: String,
    /// 复核人
    #[serde(default)]
    pub checker: String,
    /// 开票时间（毫秒，如：1594023438064），以服务商回传成功时间为准
    #[serde(default)]
    pub create_time: i64,
    /// 开票人
    #[serde(default)]
    pub drawer: String,
    /// 发票代码
    #[serde(default)]
    pub invoice_code: String,
    #[serde(default)]
    pub invoice_item_list: Option<Vec<InvoiceItem>>,
    /// 发票号码
    #[serde(default)]
    pub invoice_no: String,
    /// 发票开票日期（毫秒，如：1594023438064)
    #[serde(default)]
    pub invoice_time: i64,
    /// 发票类型 0-蓝票，1-红票
    #[serde(default)]
    pub invoice_type: i32,
    /// pdd订单号
    #[serde(default)]
    pub order_sn: String,
    /// 原蓝票代码（红票时返回）
    #[serde(default)]
    pub original_invoice_code: String,
    /// 原蓝票号码（红票时返回）
    #[serde(default)]
    pub original_invoice_no: String,
    /// 收款人
    #[serde(default)]
    pub payee: String,
    /// 发票下载地址，30分钟内有效
    #[serde(default)]
    pub pdf_path: String,
    /// 发票备注信息
    #[serde(default)]
    pub remark: String,
    /// 销方地址
    #[serde(default)]
    pub seller_address: String,
    /// 销方银行账号
    #[serde(default)]
    pub seller_bank_account: String,
    /// 销方银行名称
    #[serde(default)]
    pub seller_bank_name: String,
    /// 销方名称
    #[serde(default)]
    pub seller_name: String,
    /// 销方电话
    #[serde(default)]
    pub seller_phone_number: String,
    /// 销方税号
    #[serde(default)]
    pub seller_tax_no: String,
    /// 价税合计金额(放大100倍，单位分)
    #[serde(default)]
    pub total_amount: i64,
    /// 合计金额（不含税，放大100倍，单位分）
    #[serde(default)]
    pub total_price: i64,
    /// 合计税额(放大100倍，单位分)
    #[serde(default)]
    pub total_tax_amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddEinvoiceInfoQueryResponse {
    #[serde(default)]
    pub invoice_info_list: Option<Vec<InvoiceInfo>>,
}
