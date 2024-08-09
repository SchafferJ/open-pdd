//! # 门票商品查询接口
//!
//! 门票商品查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddTicketGoodsQueryRequest {
    /// 草稿id，入参草稿id时，表示查询该草稿的信息
    pub goods_commit_id: Option<i64>,
    /// 商品id入参商品id时，表示查询该商品的线上商品信息。。
    pub goods_id: i64,
}

impl RequestType for PddTicketGoodsQueryRequest {
    type Response = PddTicketGoodsQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ticket.goods.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct CarouselVideo {
    /// 轮播视频id
    #[serde(default)]
    pub file_id: i64,
    /// 轮播视频url
    #[serde(default)]
    pub video_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GoodsProperties {
    /// 父规格id，仅销售属性有
    #[serde(default)]
    pub parent_spec_id: i64,
    /// 引用属性id
    #[serde(default)]
    pub ref_pid: i64,
    /// 规格id，仅销售属性有和sku中的spec对应
    #[serde(default)]
    pub spec_id: i64,
    /// 属性值
    #[serde(default)]
    pub value: String,
    /// 属性值单位
    #[serde(default)]
    pub value_unit: String,
    /// 属性值id
    #[serde(default)]
    pub vid: i64,
}

#[derive(Debug, Deserialize)]
pub struct ChildSkus {
    /// 日期。格式：2020-06-01
    #[serde(default)]
    pub date: String,
    /// 拼团价，单位为分。
    #[serde(default)]
    pub group_price: i64,
    /// 线上库存
    #[serde(default)]
    pub quantity: i64,
    /// 库存增减，当查草稿时返回。
    #[serde(default)]
    pub quantity_delta: i64,
    /// 线上预扣库存
    #[serde(default)]
    pub reserve_quantity: i64,
    /// 单买价，单位为分
    #[serde(default)]
    pub single_price: i64,
    /// skuId
    #[serde(default)]
    pub sku_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    /// 父规格id
    #[serde(default)]
    pub parent_id: i64,
    /// 父规格名称
    #[serde(default)]
    pub parent_name: String,
    /// 规格id
    #[serde(default)]
    pub spec_id: i64,
    /// 规格名称
    #[serde(default)]
    pub spec_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Sku {
    /// 子sku列表，仅当sku_type为日历库存且父sku数小于等于10个时返回。若父sku多于10个，需要在pdd.goods.child.sku.detail.get接口中查询子sku信息。
    #[serde(default)]
    pub child_skus: Option<Vec<ChildSkus>>,
    /// 拼团价，单位为分。当sku_type为日历库存时是可预定日期的拼团价最低价。
    #[serde(default)]
    pub group_price: i64,
    /// 上架状态。0=已下架，1=已上架。
    #[serde(default)]
    pub is_onsale: i32,
    /// 商品sku外部编码，同其他接口中的outer_id 、out_id、out_sku_sn、outer_sku_sn、out_sku_id、outer_sku_id 都为商家编码（sku维度）。
    #[serde(default)]
    pub out_sku_sn: String,
    /// 线上库存量
    #[serde(default)]
    pub quantity: i64,
    /// 库存增减，当查草稿时返回。
    #[serde(default)]
    pub quantity_delta: i64,
    /// 线上预扣库存量
    #[serde(default)]
    pub reserve_quantity: i64,
    /// 调pdd.scenic.sku.rule.get得到的规则id
    #[serde(default)]
    pub rule_id: String,
    /// 单买价，单位为分。当sku_type为日历库存时是可预定日期的单买价最低价。
    #[serde(default)]
    pub single_price: i64,
    /// skuId
    #[serde(default)]
    pub sku_id: String,
    /// sku规格列表
    #[serde(default)]
    pub spec: Option<Vec<Spec>>,
    /// SKU预览图
    #[serde(default)]
    pub thumb_url: String,
}

#[derive(Debug, Deserialize)]
pub struct PddTicketGoodsQueryResponse {
    /// 商品轮播图
    #[serde(default)]
    pub carousel_gallery: Vec<String>,
    /// 轮播视频
    #[serde(default)]
    pub carousel_video: Option<Vec<CarouselVideo>>,
    /// 类目id，国内门票（含港澳台）9088，国外门票20042。
    #[serde(default)]
    pub cat_id: i32,
    /// 电子票发码方式，0=手动电子票；1=实时电子票，自动发货。
    #[serde(default)]
    pub code_mode: i32,
    /// 商品草稿状态，查询草稿id时返回。0=编辑中，1=待审核，2=审核通过，3=审核驳回
    #[serde(default)]
    pub commit_status: i32,
    /// 商品详情图
    #[serde(default)]
    pub detail_gallery: Vec<String>,
    /// 商品描述
    #[serde(default)]
    pub goods_desc: String,
    /// 商品标题
    #[serde(default)]
    pub goods_name: String,
    /// 商品属性
    #[serde(default)]
    pub goods_properties: Option<Vec<GoodsProperties>>,
    /// 商品状态，查询商品id时返回。1=上架，2=下架，3=售罄，4=已删除
    #[serde(default)]
    pub goods_status: i32,
    /// 商品参考价，单位为分。
    #[serde(default)]
    pub market_price: i64,
    /// 商品goods外部编码，同其他接口中的outer_goods_id 、out_goods_id、out_goods_sn、outer_goods_sn 都为商品维度的商家编码。
    #[serde(default)]
    pub out_goods_sn: String,
    /// 预定时间限制，格式：1_20_00，含义：需要提前1天，且在20:00分之前才可预定那天的门票。若为空则表示不限制预定时间。0_24_00表示在当前的24点前预定都可以，等效于不限制预定时间。
    #[serde(default)]
    pub reserve_limit_rule: String,
    /// sku列表
    #[serde(default)]
    pub sku_list: Option<Vec<Sku>>,
    /// 销售方式，0=普通库存，1=日历库存。
    #[serde(default)]
    pub sku_type: i32,
}
