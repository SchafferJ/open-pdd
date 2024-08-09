//! # 门票商品新建及更新接口
//!
//! 门票商品新建及更新
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CarouselVideo {
    /// 轮播视频id
    pub file_id: Option<i64>,
    /// 轮播视频url
    pub video_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GoodsProperties {
    /// 父规格id，仅对于销售属性入参
    pub parent_spec_id: Option<i64>,
    /// 引用属性id
    pub ref_pid: Option<i64>,
    /// 规格id，仅对于销售属性入参，和sku中的spec对应
    pub spec_id: Option<i64>,
    /// 属性值
    pub value: Option<String>,
    /// 属性值单位
    pub value_unit: Option<String>,
    /// 属性值id
    pub vid: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ChildSkus {
    /// 日期。格式：2020-06-01。每个sku最多支持180天。
    pub date: Option<String>,
    /// 拼团价，单位为分。
    pub group_price: Option<i64>,
    /// 库存增减。比如传-10表示将对应的sku库存减10。
    pub quantity_delta: Option<i64>,
    /// 单买价，单位为分。
    pub single_price: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct Sku {
    /// 仅当sku_type为日历库存时入参。若父sku多于10个，需要通过pdd.goods.child.sku.edit接口分批维护。
    pub child_skus: Option<Vec<ChildSkus>>,
    /// 拼团价，单位为分。仅当sku_type为普通库存时入参
    pub group_price: Option<i64>,
    /// 上架状态。0=已下架，1=已上架。新建sku时不传时表示上架。
    pub is_onsale: Option<i32>,
    /// sku外部编码，同其他接口中的outer_id 、out_id、out_sku_sn、outer_sku_sn、out_sku_id、outer_sku_id 都为商家编码（sku维度）。
    pub out_sku_sn: Option<String>,
    /// 库存增减。仅当sku_type为普通库存时入参。比如传-10表示将对应的sku库存减10。
    pub quantity_delta: Option<i64>,
    /// 调pdd.scenic.sku.rule.get得到的规则id。在发布成功后不可修改。
    pub rule_id: Option<String>,
    /// 单买价，单位为分。仅当sku_type为普通库存时入参。
    pub single_price: Option<i64>,
    /// 如果传值，则在原sku基础上进行编辑，如果传空，则新增sku
    pub sku_id: Option<i64>,
    /// 商品规格列表，从pdd.goods.cat.template.get中获取销售属性规格id后，再在pdd.goods.spec.id.get获取spec_id。spec_id需要和goods_properties中的对应。对于多种规格，需要传每个规格的spec_id的值，如[20,5]。在发布后不可修改。
    pub spec_id_list: Option<Vec<i64>>,
    /// SKU预览图。图片格式支持JPEG/JPG/PNG， 图片尺寸长宽比1：1且尺寸不低于480px，图片大小最高1MB。先通过pdd.goods.image.upload上传图片
    pub thumb_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PddTicketGoodsUploadRequest {
    /// 商品轮播图，按次序上传，图片格式支持JPEG/JPG/PNG， 图片尺寸长宽比1：1且尺寸不低于480px，图片大小最高1MB。先通过pdd.goods.image.upload上传图片
    pub carousel_gallery: Option<Vec<String>>,
    /// 轮播视频。需要先上传到pdd.goods.filespace.image.upload
    pub carousel_video: Option<Vec<CarouselVideo>>,
    /// 类目id，国内门票（含港澳台）传9088，国外门票传20042。发布成功后不能修改。新增商品时必填。
    pub cat_id: Option<i64>,
    /// 电子票发码方式，0=手动电子票；1=实时电子票，自动发货。新增商品时必填。
    pub code_mode: Option<i32>,
    /// 商品详情图： a. 尺寸要求宽度处于480~1200px之间，高度0-1500px之间 b. 大小1M以内 c. 数量限制在20张之间 d. 图片格式仅支持JPG,PNG格式 。先通过pdd.goods.image.upload上传图片，新增商品时必填。
    pub detail_gallery: Option<Vec<String>>,
    /// 草稿id，编辑草稿时必传。
    pub goods_commit_id: Option<i64>,
    /// 商品描述，字数限制：20~500。新增商品时必填。
    pub goods_desc: Option<String>,
    /// 商品id，编辑商品时必传。
    pub goods_id: Option<i64>,
    /// 商品标题，新增商品时必填。
    pub goods_name: Option<String>,
    /// 商品属性，先调pdd.goods.cat.template.get，根据cat_id获取，新增商品时必填。
    pub goods_properties: Option<Vec<GoodsProperties>>,
    /// 是否获取商品发布警告信息，默认为忽略
    pub ignore_edit_warn: Option<bool>,
    /// 是否提交本次编辑，0=不提交，表示仅保存草稿，不进行提交，不会进行校验；1=提交，表示提交本次编辑内容，会进行校验；不传时默认为提交
    pub is_submit: Option<i32>,
    /// 商品参考价，单位为分，必须高于最高的sku单买价。新增商品时必填。
    pub market_price: Option<i64>,
    /// 商品goods外部编码，同其他接口中的outer_goods_id 、out_goods_id、out_goods_sn、outer_goods_sn 都为商品维度的商家编码。
    pub out_goods_sn: Option<String>,
    /// 预定时间限制，格式：1_20_00，含义：需要提前1天，且在20:00分之前才可预定那天的门票。若不传则表示不限制预定时间。0_24_00表示在当前的24点前预定都可以，等效于不限制预定时间。
    pub reserve_limit_rule: Option<String>,
    /// sku列表。新增商品时必填。整个sku_list会作为整体更新。
    pub sku_list: Option<Vec<Sku>>,
    /// 销售方式，0=普通库存，1=日历库存。对于普通库存入参sku维度的价格库存，对于日历库存需要在pdd.goods.child.sku.edit入参child_sku维度的价格库存后再提交。编辑商品时不允许修改。
    pub sku_type: Option<i32>,
    /// 提交后上下架状态，0=上架；1=保持原样。表示编辑商品并提交后商品的上下架状态，不传时默认为0，上架。
    pub sync_goods_operate: Option<i32>,
}

impl RequestType for PddTicketGoodsUploadRequest {
    type Response = PddTicketGoodsUploadResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ticket.goods.upload"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddTicketGoodsUploadResponse {
    /// 草稿id
    #[serde(default)]
    pub goods_commit_id: i64,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
}
