//! # 批量数据解密脱敏接口
//!
//! 批量数据解密脱敏
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
pub struct PddOpenDecryptMaskBatchRequest {
    /// 数据列表, 默认列表大小不超过100
    pub data_list: Vec<Data>,
}

impl RequestType for PddOpenDecryptMaskBatchRequest {
    type Response = PddOpenDecryptMaskBatchResponse;

    fn get_type(&self) -> &'static str {
        "pdd.open.decrypt.mask.batch"
    }
}

#[derive(Debug, Deserialize)]
pub struct DataDecrypt {
    /// 解密tag，对于订单数据是订单号
    #[serde(default)]
    pub data_tag: String,
    /// 1、虚拟卡密;2、虚拟卡号;3、支付商品编码;4、支付单号;5、收件人;6、收件人手机号;7、收件人完整地址;8、收件人详细地址;9、快递单号;10、身份证号;11、身份证姓名
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
}

#[derive(Debug, Deserialize)]
pub struct PddOpenDecryptMaskBatchResponse {
    /// list
    #[serde(default)]
    pub data_decrypt_list: Option<Vec<DataDecrypt>>,
}
