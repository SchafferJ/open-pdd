//! # 多多进宝信息流渠道备案授权素材上传接口
//!
//! 多多进宝信息流渠道备案授权素材上传使用
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkGoodsPromotionRightAuthRequest {
    /// 推广商品视频素材url
    pub demo_url: String,
    /// 渠道duoId
    pub duo_id: i64,
    /// 商品GoodsId
    pub goods_id: i64,
    /// 商家资质证明图片url列表，1到3张图
    pub mall_certificate_url: Vec<String>,
    /// 推广视频预览码url
    pub promotion_code_url: String,
    /// 推广结束时间戳，毫秒
    pub promotion_end_time: i64,
    /// 推广开始时间戳，毫秒
    pub promotion_start_time: i64,
    /// 商品图片素材url列表，0到3张图
    pub thumb_pic_url: Option<Vec<String>>,
}

impl RequestType for PddDdkGoodsPromotionRightAuthRequest {
    type Response = PddDdkGoodsPromotionRightAuthResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.goods.promotion.right.auth"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddDdkGoodsPromotionRightAuthResponse {
    /// 备案失败原因
    #[serde(default)]
    pub reason: String,
    /// 备案结果
    #[serde(default)]
    pub result: bool,
}
