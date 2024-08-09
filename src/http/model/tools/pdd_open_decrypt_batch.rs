//! # 批量数据解密接口
//!
//! 批量数据解密
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Data {
    /// 解密tag，对于订单数据是订单号
    pub data_tag: String,
    /// 密文
    pub encrypted_data: String,
}

#[derive(Debug, Serialize)]
pub struct PddOpenDecryptBatchRequest {
    /// 数据列表, 默认列表大小不超过100
    pub data_list: Vec<Data>,
}

impl RequestType for PddOpenDecryptBatchRequest {
    type Response = PddOpenDecryptBatchResponse;

    fn get_type(&self) -> &'static str {
        "pdd.open.decrypt.batch"
    }
}

#[derive(Debug, Deserialize)]
pub struct DataDecrypt {
    /// 解密tag，对于订单数据是订单号
    #[serde(default)]
    pub data_tag: String,
    /// 1、虚拟卡密;2、虚拟卡号;3、支付申报订单号;4、支付单号;5、收件人;6、收件人手机号;7、收件人完整地址;8、收件人详细地址;9、快递单号;10、身份证号;11、身份证姓名
    #[serde(default)]
    pub data_type: i32,
    /// 解密结果
    #[serde(default)]
    pub decrypted_data: String,
    /// 加密数据
    #[serde(default)]
    pub encrypted_data: String,
    /// 错误码, 0:成功
    #[serde(default)]
    pub error_code: i32,
    /// 错误信息
    #[serde(default)]
    pub error_msg: String,
    /// 虚拟号分机号
    #[serde(default)]
    pub virtual_identify_number: String,
    /// 虚拟号类型：0-非虚拟号 1-虚拟号
    #[serde(default)]
    pub virtual_number_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddOpenDecryptBatchResponse {
    /// list
    #[serde(default)]
    pub data_decrypt_list: Option<Vec<DataDecrypt>>,
}
