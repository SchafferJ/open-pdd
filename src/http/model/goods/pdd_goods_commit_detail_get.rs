//! # 获取商品提交的商品详情
//!
//! 商品编辑或者提交之后，可以通过此接口查询提交后的编辑信息
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddGoodsCommitDetailGetRequest {
    /// 提交申请的序列id
    pub goods_commit_id: i64,
    /// 商品id
    pub goods_id: i64,
}

impl RequestType for PddGoodsCommitDetailGetRequest {
    type Response = PddGoodsCommitDetailGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.commit.detail.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct CarouselVideo {
    /// 商品视频id
    #[serde(default)]
    pub file_id: i64,
    /// 商品视频url
    #[serde(default)]
    pub video_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ElecGoodsAttributes {
    /// 开始时间（timeType=1时必填表示核销的开始时间）（精确到毫秒）
    #[serde(default)]
    pub begin_time: i64,
    /// 天数内有效（timeType=3必填，表示发货后几天内核销）
    #[serde(default)]
    pub days_time: i32,
    /// 截止时间（timeType=1,2时必填，表示发货后核销的截止时间）（精确到毫秒）
    #[serde(default)]
    pub end_time: i64,
    /// 卡券核销类型（1：起始时间内有效，2：发货后后至截止时间内有效，3：发货后多少天内有效）
    #[serde(default)]
    pub time_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct GoodsProperty {
    /// 属性值分组ID
    #[serde(default)]
    pub group_id: i32,
    /// 图片url
    #[serde(default)]
    pub img_url: String,
    /// 备注
    #[serde(default)]
    pub note: String,
    /// 父规格ID
    #[serde(default)]
    pub parent_spec_id: i64,
    /// 属性单位
    #[serde(default)]
    pub punit: String,
    /// 引用属性id
    #[serde(default)]
    pub ref_pid: i64,
    /// 规格ID
    #[serde(default)]
    pub spec_id: i64,
    /// 模板属性Id
    #[serde(default)]
    pub template_pid: i64,
    /// 基础属性值Id
    #[serde(default)]
    pub vid: i64,
    /// 基础属性值
    #[serde(default)]
    pub vvalue: String,
}

#[derive(Debug, Deserialize)]
pub struct GoodsTradeAttr {
    /// 提前预定天数
    #[serde(default)]
    pub advances_days: i32,
    /// 卡券有效期，日历日期后多少天可用。默认值为0表示仅限日历日当天使用.
    #[serde(default)]
    pub life_span: i32,
}

#[derive(Debug, Deserialize)]
pub struct GoodsTravelAttr {
    /// 是否需要出行人信息
    #[serde(default)]
    pub need_tourist: bool,
    /// 1:旅行类,2:住宿类,3:票务类
    #[serde(default)]
    pub r#type: i32,
}

#[derive(Debug, Deserialize)]
pub struct OverseaGoods {
    /// 保税仓唯一标识
    #[serde(default)]
    pub bonded_warehouse_key: String,
    /// 消费税率
    #[serde(default)]
    pub consumption_tax_rate: i32,
    /// 清关服务商
    #[serde(default)]
    pub customs_broker: String,
    /// 海关编号
    #[serde(default)]
    pub hs_code: String,
    /// 增值税率
    #[serde(default)]
    pub value_added_tax_rate: i32,
}

#[derive(Debug, Deserialize)]
pub struct OverseaSku {
    /// 计量单位编码，从接口pdd.gooods.sku.measurement.list获取desc
    #[serde(default)]
    pub measurement_code: String,
    /// 规格
    #[serde(default)]
    pub specifications: String,
    /// 税费
    #[serde(default)]
    pub taxation: i32,
}

#[derive(Debug, Deserialize)]
pub struct SkuProperty {
    /// 属性单位
    #[serde(default)]
    pub punit: String,
    /// 属性id
    #[serde(default)]
    pub ref_pid: i64,
    /// 属性值
    #[serde(default)]
    pub value: String,
    /// 属性值id
    #[serde(default)]
    pub vid: i64,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    /// 商品规格对应的ID
    #[serde(default)]
    pub parent_id: i64,
    /// 商品规格ID对应的规格名称
    #[serde(default)]
    pub parent_name: String,
    /// 生成的自定义规格ID
    #[serde(default)]
    pub spec_id: i64,
    /// 商家编辑的规格值，如颜色规格下设置白色属性
    #[serde(default)]
    pub spec_name: String,
    /// 规格备注
    #[serde(default)]
    pub spec_note: String,
}

#[derive(Debug, Deserialize)]
pub struct Sku {
    /// 上下架状态 1：上架 0 ：下架
    #[serde(default)]
    pub is_onsale: i32,
    /// sku送装参数：长度
    #[serde(default)]
    pub lengtj: i64,
    /// sku购买限制
    #[serde(default)]
    pub limit_quantity: i64,
    /// 商品团购价格 单位分
    #[serde(default)]
    pub multi_price: i64,
    /// 商家编码（sku维度），同其他接口中的outer_id 、out_id、out_sku_sn、outer_sku_sn、out_sku_id、outer_sku_id 都为商家编码（sku维度）
    #[serde(default)]
    pub out_sku_sn: String,
    /// 第三方sku Id
    #[serde(default)]
    pub out_source_sku_id: String,
    /// oversea_sku
    #[serde(default)]
    pub oversea_sku: Option<OverseaSku>,
    /// 商品单买价格 单位分
    #[serde(default)]
    pub price: i64,
    /// 库存
    #[serde(default)]
    pub quantity: i64,
    /// sku编码
    #[serde(default)]
    pub sku_id: i64,
    /// sku预售时间，单位秒
    #[serde(default)]
    pub sku_pre_sale_time: i32,
    /// sku属性
    #[serde(default)]
    pub sku_property_list: Option<Vec<SkuProperty>>,
    /// 商品规格列表
    #[serde(default)]
    pub spec: Option<Vec<Spec>>,
    /// sku预览图
    #[serde(default)]
    pub thumb_url: String,
    /// 重量，单位为g
    #[serde(default)]
    pub weight: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsCommitDetailGetResponse {
    /// 坏果包赔
    #[serde(default)]
    pub bad_fruit_claim: i32,
    /// 限购次数
    #[serde(default)]
    pub buy_limit: i64,
    /// 商品轮播图列表
    #[serde(default)]
    pub carousel_gallery_list: Vec<String>,
    /// 商品视频
    #[serde(default)]
    pub carousel_video: Option<Vec<CarouselVideo>>,
    /// 轮播视频
    #[serde(default)]
    pub carousel_video_url: i32,
    /// 类目id
    #[serde(default)]
    pub cat_id: i64,
    /// 驳回原因，仅在status=2时返回，其余状态返回空值
    #[serde(default)]
    pub commit_message: String,
    /// 运费模版id
    #[serde(default)]
    pub cost_template_id: i64,
    /// 地区/国家id
    #[serde(default)]
    pub country_id: i64,
    /// 团购人数
    #[serde(default)]
    pub customer_num: i64,
    /// 海关名称
    #[serde(default)]
    pub customs: String,
    /// 是否删除
    #[serde(default)]
    pub deleted: i32,
    /// 是否当日发货,0 否，1 是
    #[serde(default)]
    pub delivery_one_day: i32,
    /// 发货方式。0：无物流发货；1：有物流发货。
    #[serde(default)]
    pub delivery_type: i32,
    /// 商品详情图
    #[serde(default)]
    pub detail_gallery_list: Vec<String>,
    /// 卡券类商品属性
    #[serde(default)]
    pub elec_goods_attributes: Option<ElecGoodsAttributes>,
    /// 提交申请的序列ID
    #[serde(default)]
    pub goods_commit_id: i64,
    /// 商品描述， 字数限制：20-500，例如，新包装，保证产品的口感和新鲜度。单颗独立小包装，双重营养，1斤家庭分享装，更实惠新疆一级骏枣夹核桃仁。
    #[serde(default)]
    pub goods_desc: String,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 商品属性列表
    #[serde(default)]
    pub goods_property_list: Option<Vec<GoodsProperty>>,
    /// 商品状态，枚举：0-编辑中，1-待审核，2-审核通过，3-审核驳回
    #[serde(default)]
    pub goods_status: i32,
    /// 日历商品交易相关信息
    #[serde(default)]
    pub goods_trade_attr: Option<GoodsTradeAttr>,
    /// 商品出行信息
    #[serde(default)]
    pub goods_travel_attr: Option<GoodsTravelAttr>,
    /// 商品类型：1-国内普通商品，2-一般贸易，3-保税仓BBC直供，4-海外BC直邮 ,5-流量 ,6-话费 ,7-优惠券 ,8-QQ充值 ,9-加油卡，15-商家卡券，18-海外CC行邮 19-平台卡券
    #[serde(default)]
    pub goods_type: i32,
    /// 商品主图
    #[serde(default)]
    pub image_url: String,
    /// 是否支持正品发票；0-不支持、1-支持
    #[serde(default)]
    pub invoice_status: i32,
    /// 是否需要上报海关 0:否 1:是
    #[serde(default)]
    pub is_customs: i32,
    /// 是否支持假一赔十，0-不支持，1-支持
    #[serde(default)]
    pub is_folt: i32,
    /// 是否成团预售。0：不是；1:是。
    #[serde(default)]
    pub is_group_pre_sale: i32,
    /// 是否预售,true-预售商品，false-非预售商品
    #[serde(default)]
    pub is_pre_sale: i32,
    /// 是否7天无理由退换货，1-支持，0-不支持
    #[serde(default)]
    pub is_refundable: i32,
    /// 是否sku预售，0：否，1：是
    #[serde(default)]
    pub is_sku_pre_sale: i32,
    /// 缺重包退
    #[serde(default)]
    pub lack_of_weight_claim: i32,
    /// 本地服务id
    #[serde(default)]
    pub local_service_id_list: Vec<i32>,
    /// 买家自提模版id
    #[serde(default)]
    pub mai_jia_zi_ti: String,
    /// 参考价格，单位为分
    #[serde(default)]
    pub market_price: i64,
    /// 单次限量
    #[serde(default)]
    pub order_limit: i64,
    /// 原产地id，是指海淘商品的生产地址
    #[serde(default)]
    pub origin_country_id: i32,
    /// 第三方商品Id
    #[serde(default)]
    pub out_source_goods_id: String,
    /// 第三方商品来源
    #[serde(default)]
    pub out_source_type: i32,
    /// 商家编码（商品维度），同其他接口中的outer_goods_id 、out_goods_id、out_goods_sn、outer_goods_sn 都为商家编码（goods维度）
    #[serde(default)]
    pub outer_goods_id: String,
    /// oversea_goods
    #[serde(default)]
    pub oversea_goods: Option<OverseaGoods>,
    /// oversea_type
    #[serde(default)]
    pub oversea_type: i32,
    /// 预售时间
    #[serde(default)]
    pub pre_sale_time: i64,
    /// 0：不支持全国联保；1：支持全国联保
    #[serde(default)]
    pub quan_guo_lian_bao: i32,
    /// 是否二手
    #[serde(default)]
    pub second_hand: i32,
    /// 上门安装模版id
    #[serde(default)]
    pub shang_men_an_zhuang: String,
    /// 承诺发货时间（ 秒）
    #[serde(default)]
    pub shipment_limit_second: i64,
    /// 门店组id
    #[serde(default)]
    pub shop_group_id: i64,
    /// 门店组名称
    #[serde(default)]
    pub shop_group_name: String,
    /// sku列表
    #[serde(default)]
    pub sku_list: Option<Vec<Sku>>,
    /// 日历商品库存方式（0：普通型，1：日历型）
    #[serde(default)]
    pub sku_type: i32,
    /// 送货入户并安装模版id
    #[serde(default)]
    pub song_huo_an_zhuang: String,
    /// 送货入户模版id
    #[serde(default)]
    pub song_huo_ru_hu: String,
    /// 新包装，保证产品的口感和新鲜度。单颗独立小包装，双重营养，1斤家庭分享装，更实惠新疆一级骏枣夹核桃仁。
    #[serde(default)]
    pub tiny_name: String,
    /// 满2件折扣，可选范围0-100, 0表示取消，95表示95折，设置需先查询规则接口获取实际可填范围
    #[serde(default)]
    pub two_pieces_discount: i32,
    /// 保税仓
    #[serde(default)]
    pub warehouse: String,
    /// 水果类目温馨提示
    #[serde(default)]
    pub warm_tips: String,
    /// 只换不修的天数，目前只支持0和365
    #[serde(default)]
    pub zhi_huan_bu_xiu: i32,
    /// 保密发货，0：不支持，1：支持
    #[serde(default)]
    pub privacy_delivery: i32,
}
