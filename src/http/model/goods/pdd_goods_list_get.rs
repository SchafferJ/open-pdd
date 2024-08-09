//! # 商品列表接口
//!
//! 商品列表查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsListGetRequest {
    /// 模版id
    pub cost_template_id: Option<i64>,
    /// 商品创建时间结束时间的时间戳，指格林威治时间 1970 年 01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至结束时间的总秒数 PS：开始时间结束时间间距不超过7天
    pub created_at_end: Option<i64>,
    /// 商品创建时间开始时间的时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至开始时间的总秒数
    pub created_at_from: Option<i64>,
    /// 商品名称模糊查询,outer_id,is_onsale,goods_name三选一，优先级is_onsale>outer_id>goods_name
    pub goods_name: Option<String>,
    /// 上下架状态，0-下架，1-上架,outer_id,is_onsale,goods_name三选一，优先级is_onsale>outer_id>goods_name
    pub is_onsale: Option<i32>,
    /// 商家外部商品编码，支持多个，用逗号隔开，最多10个
    pub outer_goods_id: Option<String>,
    /// 商品外部编码（sku），同其他接口中的outer_id 、out_id、out_sku_sn、outer_sku_sn、out_sku_id、outer_sku_id 都为商家编码（sku维度）。outer_id,is_onsale,goods_name三选一，优先级is_onsale>outer_id>goods_name
    pub outer_id: Option<String>,
    /// 返回页码 默认 1，页码从 1 开始PS：当前采用分页返回，数量和页数会一起传，如果不传，则采用 默认值
    pub page: Option<i32>,
    /// 返回数量，默认 100，最大100。
    pub page_size: Option<i32>,
}

impl RequestType for PddGoodsListGetRequest {
    type Response = PddGoodsListGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.list.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct SpecDetails {
    /// 父规格id
    #[serde(default)]
    pub parent_id: i64,
    /// 父规格名
    #[serde(default)]
    pub parent_name: String,
    /// 子规格id
    #[serde(default)]
    pub spec_id: i64,
    /// 子规格名称
    #[serde(default)]
    pub spec_name: String,
    /// 规则备注
    #[serde(default)]
    pub spec_note: String,
}

#[derive(Debug, Deserialize)]
pub struct Sku {
    /// sku是否在架上，0-下架中，1-架上
    #[serde(default)]
    pub is_sku_onsale: i32,
    /// 商家外部编码（商品），同其他接口中的outer_goods_id 、out_goods_id、out_goods_sn、outer_goods_sn 都为商家编码（goods维度）。
    #[serde(default)]
    pub outer_goods_id: String,
    /// 商家外部编码（sku），同其他接口中的outer_id 、out_id、out_sku_sn、outer_sku_sn、out_sku_id、outer_sku_id 都为商家编码（sku维度）。
    #[serde(default)]
    pub outer_id: String,
    /// sku预扣库存
    #[serde(default)]
    pub reserve_quantity: i64,
    /// sku编码
    #[serde(default)]
    pub sku_id: i64,
    /// sku库存
    #[serde(default)]
    pub sku_quantity: i64,
    /// 规格名称
    #[serde(default)]
    pub spec: String,
    /// 规格信息
    #[serde(default)]
    pub spec_details: Option<Vec<SpecDetails>>,
}

#[derive(Debug, Deserialize)]
pub struct Goods {
    /// 商品创建时间的时间戳，指格林威治时间 1970 年01 月 01 日 00 时 00 分 00 秒(北京时间 1970 年 01 月 01 日 08 时 00 分 00 秒)起至商品创建时间的总秒数
    #[serde(default)]
    pub created_at: i64,
    /// 商品编码
    #[serde(default)]
    pub goods_id: i64,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 商品总数量
    #[serde(default)]
    pub goods_quantity: i64,
    /// 商品预扣库存
    #[serde(default)]
    pub goods_reserve_quantity: i64,
    /// 商品图片
    #[serde(default)]
    pub image_url: String,
    /// 是否多sku，0-单sku，1-多sku
    #[serde(default)]
    pub is_more_sku: i32,
    /// 是否在架上，0-下架中，1-架上
    #[serde(default)]
    pub is_onsale: i32,
    /// sku列表对象
    #[serde(default)]
    pub sku_list: Option<Vec<Sku>>,
    /// 商品缩略图
    #[serde(default)]
    pub thumb_url: String,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsListGetResponse {
    /// 商品列表对象
    #[serde(default)]
    pub goods_list: Option<Vec<Goods>>,
    /// 返回商品总数
    #[serde(default)]
    pub total_count: i32,
}
