//! # 多多客生成单品推广小程序二维码url
//!
//! 多多客生成单品推广小程序二维码url
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkWeappQrcodeUrlGenRequest {
    /// 多多礼金ID
    pub cash_gift_id: Option<i64>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key
    pub custom_parameters: Option<String>,
    /// 是否生成店铺收藏券推广链接
    pub generate_mall_collect_coupon: Option<bool>,
    /// 商品goodsSign列表，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign_list: Option<Vec<String>>,
    /// 推广位ID
    pub p_id: String,
    /// 招商多多客ID
    pub zs_duo_id: Option<i64>,
}

impl RequestType for PddDdkWeappQrcodeUrlGenRequest {
    type Response = PddDdkWeappQrcodeUrlGenResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.weapp.qrcode.url.gen"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkWeappQrcodeUrlGenResponse {
    /// 单品推广小程序二维码url
    #[serde(default)]
    pub url: String,
}
