//! # 商品详情接口
//!
//! 商品详情（此接口后续不再维护，请使用pdd.goods.detail.get接口）
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsInformationGetRequest {
    /// 商品编码
    pub goods_id: i64,
}

impl RequestType for PddGoodsInformationGetRequest {
    type Response = PddGoodsInformationGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.information.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct Sku {
    /// 商品规格名称
    #[serde(default)]
    pub spec: String,
    /// 商品sku编码
    #[serde(default)]
    pub sku_id: i64,
    /// 商品sku库存
    #[serde(default)]
    pub sku_quantity: i64,
    /// 商家外部编码（sku），同其他接口中的outer_id 、out_id、out_sku_sn、outer_sku_sn、out_sku_id、outer_sku_id 都为商家编码（sku维度）。
    #[serde(default)]
    pub outer_id: String,
    /// 商家外部编码（商品），同其他接口中的outer_goods_id 、out_goods_id、out_goods_sn、outer_goods_sn 都为商家编码（goods维度）。
    #[serde(default)]
    pub outer_goods_id: String,
    /// sku预览图
    #[serde(default)]
    pub sku_img: String,
    /// 拼团价格（元）
    #[serde(default)]
    pub group_price: String,
    /// 单买价格（元）
    #[serde(default)]
    pub single_price: String,
    /// 商品sku是否上架，0-下架，1-上架
    #[serde(default)]
    pub is_sku_onsale: i32,
}

#[derive(Debug, Deserialize)]
pub struct GoodsInfo {
    /// 商品编码
    #[serde(default)]
    pub goods_id: i64,
    /// 商品序列编码
    #[serde(default)]
    pub goods_sn: String,
    /// 1-国内普通商品，2-进口，3-国外海淘，4-直邮 ,5-流量,6-话费,7,优惠券;8-QQ充值,9-加油卡,18-CC行邮
    #[serde(default)]
    pub goods_type: i32,
    /// 商品一级类目
    #[serde(default)]
    pub goods_category: String,
    /// 叶子类目
    #[serde(default)]
    pub last_category: String,
    /// 是否七天无理由售后，0-不支持，1-支持
    #[serde(default)]
    pub is_refundable: i32,
    /// 承诺发货时间
    #[serde(default)]
    pub shipment_limit_second: i64,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 商品图片 url
    #[serde(default)]
    pub image_url: String,
    /// 商品库存
    #[serde(default)]
    pub goods_quantity: i32,
    /// 商品是否上架，0-下架，1-上架；
    #[serde(default)]
    pub is_onsale: i32,
    /// 商品是否全新，0-全新商品，1-二手商品
    #[serde(default)]
    pub second_hand: i32,
    /// 成团人数
    #[serde(default)]
    pub group_required_customer_num: i64,
    /// sku列表对象
    #[serde(default)]
    pub sku_list: Option<Vec<Sku>>,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsInformationGetResponse {
    /// 商品详情对象
    #[serde(default)]
    pub goods_info: Option<GoodsInfo>,
}
