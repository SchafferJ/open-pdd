//! 创建单元

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AdImageVO {
    /// 图片链接，可用图片参考以下接口返回：pdd.ad.api.goods.query.gallery.images（轮播图），pdd.ad.api.goods.query.long.images（长图）
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

#[derive(Debug, Serialize)]
pub struct AdTextVO {
    /// 标题
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct AdCreativeCreateMessages {
    /// 创意图片列表
    #[serde(rename = "adImageVOList")]
    pub ad_image_vo_list: Option<Vec<AdImageVO>>,
    /// 创意标题列表
    #[serde(rename = "adTextVOList")]
    pub ad_text_vo_list: Option<Vec<AdTextVO>>,
    /// 创意规格，6：商品轮播图，7：商品长图，其余规格暂不支持
    #[serde(rename = "creativeSpecificationId")]
    pub creative_specification_id: i64,
}

#[derive(Debug, Serialize)]
pub struct AdKeywordCreateMessage {
    /// 关键词出价
    pub bid: i64,
    /// 关键词溢价比例。万分比
    #[serde(rename = "premiumRate")]
    pub premium_rate: Option<i64>,
    /// 关键词
    pub word: String,
}

#[derive(Debug, Serialize)]
pub struct AdKeywordSetMessage {
    /// 词包出价。出价需在[0.10, 99.00]之间。
    #[serde(rename = "keywordSetBid")]
    pub keyword_set_bid: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct AdProductCreateMessage {
    /// 商品Id
    #[serde(rename = "goodsId")]
    pub goods_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct OptionalOptimizationBidMessage {
    /// 可选优化出价价格
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: i64,
    /// 可选优化出价目标。3表示优化店铺关注，4表示优化商品收藏，5表示优化询单
    #[serde(rename = "optimizationGoal")]
    pub optimization_goal: i32,
}

#[derive(Debug, Serialize)]
pub struct OptimizationMessage {
    /// 数据积累期出价。当使用OCPX时对该字段赋值。
    #[serde(rename = "accumulationBid")]
    pub accumulation_bid: Option<i64>,
    /// 智能投放期出价。当使用OCPX时对该字段赋值。
    #[serde(rename = "optimizationBid")]
    pub optimization_bid: Option<i64>,
    /// 优化目标。0表示不优化。1表示优化ROI，2表示优化转化成本.自定义单元时，该值必传0；当单元使用展示自动调价功能(ECPC)时，该值必须传1；当单元使用展示OCPC功能（plan_strategy=3）时，该值必须传2。
    #[serde(rename = "optimizationGoal")]
    pub optimization_goal: i32,
    /// 优化方式。0表示不优化，1表示ECPC，2表示OCPC。当单元使用ECPC时，该值必须传1；当使用OCPC时，该值必须传2。
    #[serde(rename = "optimizationMethod")]
    pub optimization_method: Option<i32>,
    /// 可选优化出价列表。当使用OCPX时对该字段赋值。
    #[serde(rename = "optionalOptimizationBidMessageList")]
    pub optional_optimization_bid_message_list: Option<Vec<OptionalOptimizationBidMessage>>,
}

#[derive(Debug, Serialize)]
pub struct AdUnitCreateMessage {
    /// 推广单元名称
    #[serde(rename = "adName")]
    pub ad_name: Option<String>,
    /// 基础出价。单位厘。
    pub bid: Option<i64>,
    /// 智能优化广告相关。当单元使用自定义推广时，不要使用该字段。
    #[serde(rename = "optimizationMessage")]
    pub optimization_message: Option<OptimizationMessage>,
}

#[derive(Debug, Serialize)]
pub struct AreaStruct {
    /// 地域Id列表。具体地域Id编码参见接口返回：pdd.ad.api.unit.bid.query.targeting.tag.list
    #[serde(rename = "areaIds")]
    pub area_ids: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct AdTargetingSet {
    /// 地域定向
    #[serde(rename = "areaStruct")]
    pub area_struct: Option<AreaStruct>,
}

#[derive(Debug, Serialize)]
pub struct AdTargetingCreateMessage {
    /// 定向集合
    #[serde(rename = "adTargetingSet")]
    pub ad_targeting_set: Option<AdTargetingSet>,
    /// 定向名称
    #[serde(rename = "targetingName")]
    pub targeting_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AudienceBidCreateMessage {
    /// 定向创建信息
    #[serde(rename = "adTargetingCreateMessage")]
    pub ad_targeting_create_message: Option<AdTargetingCreateMessage>,
    /// 人群定向类型，可用枚举值，参考接口：pdd.ad.api.unit.bid.query.base.target.profile
    #[serde(rename = "bidReferenceId")]
    pub bid_reference_id: i64,
    /// 出价，万分比
    #[serde(rename = "bidValue")]
    pub bid_value: i64,
    /// 人群定向二级Id。默认为0。
    #[serde(rename = "subBidReferenceId")]
    pub sub_bid_reference_id: i64,
}

#[derive(Debug, Serialize)]
pub struct LocationBidCreateMessage {
    /// 资源位定向类型。可取值参考接口：pdd.ad.api.unit.bid.query.available.location
    #[serde(rename = "bidReferenceId")]
    pub bid_reference_id: i64,
    /// 出价，万分比
    #[serde(rename = "bidValue")]
    pub bid_value: i64,
}

#[derive(Debug, Serialize)]
pub struct SmartCreativeCreateMessage {
    /// 智能创意流量分配比例
    #[serde(rename = "creativeFlowRate")]
    pub creative_flow_rate: Option<i32>,
    /// 是否启用智能创意标识
    #[serde(rename = "enableSmartCreative")]
    pub enable_smart_creative: Option<i32>,
    /// 智能创意标题
    #[serde(rename = "smartCreativeTitle")]
    pub smart_creative_title: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdUnitCreateComplexMessage {
    /// 创意创建信息列表
    #[serde(rename = "adCreativeCreateMessagesList")]
    pub ad_creative_create_messages_list: Option<Vec<AdCreativeCreateMessages>>,
    /// 关键词创建列表
    #[serde(rename = "adKeywordCreateMessageList")]
    pub ad_keyword_create_message_list: Option<Vec<AdKeywordCreateMessage>>,
    /// 智能词包相关信息
    #[serde(rename = "adKeywordSetMessage")]
    pub ad_keyword_set_message: Option<AdKeywordSetMessage>,
    /// 商品创建信息
    #[serde(rename = "adProductCreateMessage")]
    pub ad_product_create_message: Option<AdProductCreateMessage>,
    /// 单元创建信息
    #[serde(rename = "adUnitCreateMessage")]
    pub ad_unit_create_message: AdUnitCreateMessage,
    /// 人群定向创建信息列表
    #[serde(rename = "audienceBidCreateMessageList")]
    pub audience_bid_create_message_list: Option<Vec<AudienceBidCreateMessage>>,
    /// 资源位定向创建信息列表。仅支持展示广告。
    #[serde(rename = "locationBidCreateMessageList")]
    pub location_bid_create_message_list: Option<Vec<LocationBidCreateMessage>>,
    /// 智能创意创建信息
    #[serde(rename = "smartCreativeCreateMessage")]
    pub smart_creative_create_message: Option<SmartCreativeCreateMessage>,
}

#[derive(Debug, Serialize)]
pub struct PddAdApiUnitCreateRequest {
    /// 单元创建信息
    #[serde(rename = "adUnitCreateComplexMessage")]
    pub ad_unit_create_complex_message: AdUnitCreateComplexMessage,
    /// 广告计划Id
    #[serde(rename = "planId")]
    pub plan_id: i64,
    /// 场景类型。0表示搜索，2表示展示。
    #[serde(rename = "scenesType")]
    pub scenes_type: i32,
}

impl RequestType for PddAdApiUnitCreateRequest {
    type Response = PddAdApiUnitCreateResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.unit.create"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiUnitCreateResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    /// 单元Id
    #[serde(default)]
    pub result: i64,
    #[serde(default)]
    pub success: bool,
}
