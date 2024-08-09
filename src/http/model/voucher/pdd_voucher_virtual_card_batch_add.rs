//! # 批量添加卡券
//!
//! 供应商批量添加卡券
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct DataRequest {
    /// 卡密卡号，商家卡密必填
    #[serde(rename = "cardNo")]
    pub card_no: Option<String>,
    /// 用户核销卡密加密串，加密所使用public key向对接小二获取（加密算法"RSA"，填充方式"RSA/ECB/PKCS1Padding"）
    #[serde(rename = "encryptPassword")]
    pub encrypt_password: String,
}

#[derive(Debug, Serialize)]
pub struct Data {
    /// 充值地址
    #[serde(rename = "chargeAddress")]
    pub charge_address: Option<String>,
    /// 卡密信息列表，一次请求最多5000条卡密
    #[serde(rename = "dataList")]
    pub data_list: Vec<DataRequest>,
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: i64,
    /// skuId
    #[serde(rename = "skuId")]
    pub sku_id: i64,
}

#[derive(Debug, Serialize)]
pub struct PddVoucherVirtualCardBatchAddRequest {
    /// 业务数据
    pub data: Option<Data>,
}

impl RequestType for PddVoucherVirtualCardBatchAddRequest {
    type Response = PddVoucherVirtualCardBatchAddResponse;

    fn get_type(&self) -> &'static str {
        "pdd.voucher.virtual.card.batch.add"
    }
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 卡密批次Id
    #[serde(default, rename = "batchId")]
    pub batch_id: i64,
    /// 充值地址
    #[serde(default, rename = "chargeAddress")]
    pub charge_address: String,
    /// 店铺Id
    #[serde(default, rename = "mallId")]
    pub mall_id: i64,
    /// 批次添加的卡密数量
    #[serde(default, rename = "totalNum")]
    pub total_num: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddVoucherVirtualCardBatchAddResponse {
    /// 状态码
    #[serde(default)]
    pub code: i32,
    /// 错误信息
    #[serde(default)]
    pub message: String,
    /// 响应信息
    #[serde(default)]
    pub result: Option<Result>,
}
