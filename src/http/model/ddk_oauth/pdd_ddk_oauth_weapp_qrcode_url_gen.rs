//! # 多多客工具生成单品推广小程序二维码
//!
//! 多多客工具生成单品推广小程序二维码url
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkOauthWeappQrcodeUrlGenRequest {
    /// 自定义参数，为链接打上自定义标签。自定义参数最长限制64个字节。
    pub custom_parameters: Option<String>,
    /// 商品ID，仅支持单个查询
    pub goods_id_list: Option<Vec<i64>>,
    /// 商品goodsSign列表，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign_list: Option<Vec<String>>,
    /// 推广位ID
    pub p_id: String,
    /// 招商多多客ID
    pub zs_duo_id: Option<i64>,
}

impl RequestType for PddDdkOauthWeappQrcodeUrlGenRequest {
    type Response = PddDdkOauthWeappQrcodeUrlGenResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.oauth.weapp.qrcode.url.gen"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOauthWeappQrcodeUrlGenResponse {
    /// 单品推广小程序二维码url
    #[serde(default)]
    pub url: String,
}
