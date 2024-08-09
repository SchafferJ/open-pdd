//! # 多多进宝信息流投放商品报备进度查询
//!
//! 信息流渠道进行商品投放报备后，渠道可使用该接口进行报备商品审批进度查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkPromotionGoodsQueryRequest {
    /// 商品id
    pub goods_id: Option<i64>,
    /// 店铺id
    pub mall_id: Option<i64>,
    /// 分页查询页数
    pub page_number: Option<i32>,
    /// 分页查询页大小
    pub page_size: Option<i32>,
    /// 查询状态列表
    pub status_list: Option<Vec<i32>>,
    /// 最后更新开始时间
    pub update_end_time: Option<i64>,
    /// 最后更新结束时间（最长支持30天）
    pub update_start_time: Option<i64>,
}

impl RequestType for PddDdkPromotionGoodsQueryRequest {
    type Response = PddDdkPromotionGoodsQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.promotion.goods.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct Application {
    /// 审核信息
    #[serde(default)]
    pub comment: String,
    /// 报备提交时间
    #[serde(default)]
    pub commit_time: i64,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 店铺id
    #[serde(default)]
    pub mall_id: i64,
    /// 推广结束时间
    #[serde(default)]
    pub promotion_end_time: i64,
    /// 推广开始时间
    #[serde(default)]
    pub promotion_start_time: i64,
    /// 报备状态。0-已创建，1-已提交，2-已通过，3-已驳回
    #[serde(default)]
    pub status: i32,
    /// 最后更新时间
    #[serde(default)]
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkPromotionGoodsQueryResponse {
    /// 报备记录列表
    #[serde(default)]
    pub application_list: Option<Vec<Application>>,
    /// 报备记录总数
    #[serde(default)]
    pub total: i32,
}
