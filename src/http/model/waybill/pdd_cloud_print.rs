//! # 云打印
//!
//! 云打印接口
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CustomAreaPrintData {
    /// 打印数据
    pub data: String,
    /// 模板url
    pub template_url: String,
}

#[derive(Debug, Serialize)]
pub struct WaybillPrinterData {
    /// 追加数据,例如：{\"sender\":{\"address\":{\"province\":\"辽宁\",\"city\":\"沈阳市\",\"district\":\"铁西区\",\"detail\":\"xxx\"},\"name\":\"xxx\",\"mobile\":\"139xxxx032\"}}
    pub add_data: Option<String>,
    /// 打印数据
    pub data: String,
    /// 是否加密
    pub encrypted: Option<bool>,
    /// 签名
    pub signature: Option<String>,
    /// 模板url
    pub template_url: String,
    /// 版本
    pub ver: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PrintData {
    /// 自定区打印数据
    pub custom_area_print_data: Option<CustomAreaPrintData>,
    /// 面单打印数据
    pub waybill_printer_data: WaybillPrinterData,
}

#[derive(Debug, Serialize)]
pub struct PrinterSetting {
    /// 是否打印下联logo
    pub need_bottom_logo: Option<bool>,
    /// 是否打印中联logo
    pub need_middle_logo: Option<bool>,
    /// 是否打印上联logo
    pub need_top_logo: Option<bool>,
    /// 打印方向 normal-正常 reverse-翻转
    pub orientation: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CloudPrintRequest {
    /// 打印数据列表
    pub print_data_list: Vec<PrintData>,
    /// 打印机id
    pub printer_id: String,
    /// 打印机设置
    pub printer_setting: Option<PrinterSetting>,
    /// 共享码
    pub share_code: String,
}

#[derive(Debug, Serialize)]
pub struct PddCloudPrintRequest {
    /// 云打印请求
    pub cloud_print_request: CloudPrintRequest,
}

impl RequestType for PddCloudPrintRequest {
    type Response = PddCloudPrintResponse;

    fn get_type(&self) -> &'static str {
        "pdd.cloud.print"
    }
}

#[derive(Debug, Deserialize)]
pub struct PrintResult {
    /// 失败原因
    #[serde(default)]
    pub fail_reason: String,
    /// 序号
    #[serde(default)]
    pub print_sequence: i64,
    /// 打印任务Id
    #[serde(default)]
    pub print_task_id: String,
    /// 是否打印成功
    #[serde(default)]
    pub result: bool,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 云打印结果列表
    #[serde(default)]
    pub print_result_list: Option<Vec<PrintResult>>,
}

#[derive(Debug, Deserialize)]
pub struct PddCloudPrintResponse {
    /// 错误码
    #[serde(default)]
    pub error_code: i32,
    /// 错误描述
    #[serde(default)]
    pub error_msg: String,
    /// 结果
    #[serde(default)]
    pub result: Option<Result>,
    /// 请求是否成功
    #[serde(default)]
    pub success: bool,
}
