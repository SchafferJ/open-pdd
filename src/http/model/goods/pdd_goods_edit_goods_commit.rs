//! # 新增或编辑草稿接口
//!
//! 新增或编辑草稿
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CarouselVideo {
    /// 商品视频id
    pub file_id: Option<i64>,
    /// 商品视频url
    pub video_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ElecGoodsAttributes {
    /// 开始时间（timeType=1时必填表示核销的开始时间）（精确到毫秒）
    pub begin_time: Option<i64>,
    /// 天数内有效（timeType=3必填，表示发货后几天内核销）
    pub days_time: Option<i32>,
    /// 截止时间（timeType=1,2时必填，表示发货后核销的截止时间）（精确到毫秒
    pub end_time: Option<i64>,
    /// 卡券核销类型（1：起始时间内有效，2：发货后后至截止时间内有效，3：发货后多少天内有效）
    pub time_type: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct GoodsProperties {
    /// 组id，非销售属性不用传
    pub group_id: Option<i32>,
    /// 图片url，非销售属性不用传
    pub img_url: Option<String>,
    /// 备注，非销售属性不用传
    pub note: Option<String>,
    /// 父属性id，非销售属性不用传
    pub parent_spec_id: Option<i64>,
    /// 引用属性ID
    pub ref_pid: Option<i64>,
    /// 属性id，非销售属性不用传
    pub spec_id: Option<i64>,
    /// 模板属性id
    pub template_pid: Option<i64>,
    /// 属性值
    pub value: Option<String>,
    /// 属性单位
    pub value_unit: Option<String>,
    /// 属性值id
    pub vid: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct BookingNotes {
    /// 预定须知图片地址
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GoodsTradeAttr {
    /// 提前预定天数，默认为0表示当天可预定
    pub advances_days: Option<i32>,
    /// 预订须知
    pub booking_notes: Option<BookingNotes>,
    /// 卡券有效期，日历日期后多少天可用。默认值为0表示仅限日历日当天使用
    pub life_span: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct GoodsTravelAttr {
    /// 出行人是否必填（默认是）
    pub need_tourist: Option<bool>,
    /// 日历商品类型1:旅行类,2:住宿类,3:票务类
    pub r#type: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct OverseaGoods {
    /// 保税仓唯一标识
    pub bonded_warehouse_key: String,
    /// 消费税率
    pub consumption_tax_rate: Option<i32>,
    /// 清关服务商
    pub customs_broker: Option<String>,
    /// 海关编号
    pub hs_code: Option<String>,
    /// 增值税率
    pub value_added_tax_rate: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct OverseaSku {
    /// 计量单位编码，从接口pdd.gooods.sku.measurement.list获取code
    pub measurement_code: String,
    /// 规格
    pub specifications: String,
    /// 税费
    pub taxation: i32,
}

#[derive(Debug, Serialize)]
pub struct SkuProperties {
    /// 属性单位
    pub punit: String,
    /// 属性id
    pub ref_pid: i64,
    /// 属性值
    pub value: String,
    /// 属性值id
    pub vid: i64,
}

#[derive(Debug, Serialize)]
pub struct Sku {
    /// sku上架状态，0-已下架，1-上架中
    pub is_onsale: i32,
    /// sku送装参数：长度
    pub length: Option<i64>,
    /// sku购买限制，只入参999
    pub limit_quantity: i64,
    /// 商品团购价格
    pub multi_price: i64,
    /// 商品sku外部编码
    pub out_sku_sn: Option<String>,
    /// 第三方sku Id
    pub out_source_sku_id: Option<String>,
    /// oversea_sku
    pub oversea_sku: Option<OverseaSku>,
    /// 商品单买价格
    pub price: i64,
    /// 商品sku库存初始数量，后续库存update只使用stocks.update接口进行调用
    pub quantity: i64,
    /// sku预售时间戳，单位秒；不更新传null，取消传0，更新传实际值
    pub sku_pre_sale_time: Option<i32>,
    /// sku属性
    pub sku_properties: Vec<SkuProperties>,
    /// 商品规格列表，根据pdd.goods.spec.id.get生成的规格属性id，例如：颜色规格下商家新增白色和黑色，大小规格下商家新增L和XL，则由4种spec组合，入参一种组合即可，在skulist中需要有4个spec组合的sku
    pub spec_id_list: String,
    /// sku预览图，预览图尺寸：等宽高，且高度不低于480px，现已支持1M大小，越清晰越好卖，SKU预览图格式：仅支持JPG,PNG格式
    pub thumb_url: String,
    /// 重量，单位为g
    pub weight: i64,
}

#[derive(Debug, Serialize)]
pub struct PddGoodsEditGoodsCommitRequest {
    /// 是否自动补充标品属性
    pub auto_fill_spu_property: Option<bool>,
    /// 坏果包赔
    pub bad_fruit_claim: Option<i32>,
    /// 限购次数
    pub buy_limit: Option<i64>,
    /// 商品轮播图，按次序上传，图片格式支持JPEG/JPG/PNG， 图片尺寸长宽比1：1且尺寸不低于480px，图片大小最高1MB
    pub carousel_gallery: Option<Vec<String>>,
    /// 商品视频
    pub carousel_video: Option<Vec<CarouselVideo>>,
    /// 轮播视频
    pub carousel_video_url: Option<String>,
    /// 叶子类目ID
    pub cat_id: i64,
    /// 物流运费模板ID，可使用pdd.logistics.template.get获取
    pub cost_template_id: Option<i64>,
    /// 地区/国家ID，0-中国，暂时只传0（普通商品）
    pub country_id: Option<i32>,
    /// 团购人数
    pub customer_num: Option<i64>,
    /// 海关名称，只在goods_type为直供商品时有效（现阶段暂不支持）
    pub customs: Option<String>,
    /// 是否当日发货,0 否，1 是
    pub delivery_one_day: Option<i32>,
    /// 发货方式。0：无物流发货；1：有物流发货。
    pub delivery_type: Option<i32>,
    /// 商品详情图：
    /// a. 尺寸要求宽度处于480~1200px之间，高度0-1500px之间
    /// b. 大小1M以内
    /// c. 数量限制在20张之间
    /// d. 图片格式仅支持JPG,PNG格式
    /// e. 点击上传时，支持批量上传详情图
    pub detail_gallery: Option<Vec<String>>,
    /// 卡券类商品属性
    pub elec_goods_attributes: Option<ElecGoodsAttributes>,
    /// 商品描述， 字数限制：20-500，例如，新包装，保证产品的口感和新鲜度。单颗独立小包装，双重营养，1斤家庭分享装，更实惠新疆一级骏枣夹核桃仁。
    pub goods_desc: Option<String>,
    /// 商品标题，例如，新疆特产 红满疆枣夹核桃500g
    pub goods_name: Option<String>,
    /// 商品属性列表
    pub goods_properties: Option<Vec<GoodsProperties>>,
    /// 日历商品交易相关信息
    pub goods_trade_attr: Option<GoodsTradeAttr>,
    /// 商品出行信息
    pub goods_travel_attr: Option<GoodsTravelAttr>,
    /// 1-国内普通商品，2-一般贸易，3-保税仓BBC直供，4-海外BC直邮 ,5-流量 ,6-话费 ,7-优惠券 ,8-QQ充值 ,9-加油卡，15-商家卡券，18-海外CC行邮 19-平台卡券
    pub goods_type: Option<i32>,
    /// 商品主图，请参考拼多多首页大图，如果商品参加部分活动则必填，否则无法参加活动
    /// a. 尺寸750 x 352px
    /// b. 大小100k以内
    /// c. 图片格式仅支持JPG,PNG格式
    /// d. 图片背景应以纯白为主, 商品图案居中显示
    /// e. 图片不可以添加任何品牌相关文字或logo
    pub image_url: Option<String>,
    /// 是否支持正品发票；0-不支持、1-支持
    pub invoice_status: Option<i32>,
    /// 是否需要上报海关，现阶段入参默认false，入参true会失败
    pub is_customs: Option<bool>,
    /// 是否支持假一赔十，false-不支持，true-支持
    pub is_folt: Option<bool>,
    /// 是否成团预售。0：不是；1:是。
    pub is_group_pre_sale: Option<i32>,
    /// 是否预售,true-预售商品，false-非预售商品
    pub is_pre_sale: Option<bool>,
    /// 是否7天无理由退换货，true-支持，false-不支持
    pub is_refundable: Option<bool>,
    /// 是否sku预售，1：是，0：否
    pub is_sku_pre_sale: Option<i32>,
    /// 缺重包退
    pub lack_of_weight_claim: Option<i32>,
    /// 本地服务id
    pub local_service_id_list: Option<Vec<i32>>,
    /// 买家自提模版id
    pub mai_jia_zi_ti: Option<String>,
    /// 参考价格，单位为分
    pub market_price: Option<i64>,
    /// 单次限量
    pub order_limit: Option<i64>,
    /// 原产地id，是指海淘商品的生产地址，仅在goods type=3/4的时候必填，可以通过pdd.goods.country.get获取
    pub origin_country_id: Option<i32>,
    /// 商品goods外部编码
    pub out_goods_id: Option<String>,
    /// 第三方商品Id
    pub out_source_goods_id: Option<String>,
    /// 第三方商品来源
    pub out_source_type: Option<i32>,
    /// {
    /// 	"consumption_tax_rate": 1,
    /// 	"value_added_tax_rate": 9,
    /// 	"hs_code": "2200",
    /// 	"customs_broker": "sss",
    /// 	"customs_declaration_method": 1,
    /// 	"bonded_warehouse": "sss",
    /// 	"bonded_warehouse_key": "pp"
    /// }
    pub oversea_goods: Option<OverseaGoods>,
    /// oversea_type
    pub oversea_type: Option<i32>,
    /// 预售时间，is_pre_sale为1时必传，UNIX时间戳
    pub pre_sale_time: Option<i64>,
    /// 0：不支持全国联保；1：支持全国联保
    pub quan_guo_lian_bao: Option<i32>,
    /// 是否二手商品，true -二手商品 ，false-全新商品
    pub second_hand: Option<bool>,
    /// 上门安装模版id
    pub shang_men_an_zhuang: Option<String>,
    /// 承诺发货时间（ 秒），48小时或24小时，is_pre_sale为1时不必传
    pub shipment_limit_second: Option<i64>,
    /// 门店组id
    pub shop_group_id: Option<i64>,
    /// 尺码表id
    pub size_spec_id: Option<i64>,
    /// sku对象列表,实例：[{
    /// 	"is_onsale": 1,
    /// 	"limit_quantity": 999,
    /// 	"price": "2200",
    /// 	"weight": 1000,
    /// 	"multi_price": "1900",
    /// 	"thumb_url": "http://t06img.yangkeduo.com/images/2018-04-15/ced035033b5d40b589140af882621c03.jpg",
    /// 	"out_sku_sn": "L",
    /// 	"quantity": 100,
    /// 	"spec_id_list": "[25]",
    /// 	"oversea_sku": {
    /// 		"measurement_code": "计量单位编码",
    /// 		"taxation": "税费",
    /// 		"specifications": "规格"
    /// 	}
    /// }]
    pub sku_list: Option<Vec<Sku>>,
    /// 库存方式（0：普通型，1：日历型）
    pub sku_type: Option<i32>,
    /// 送货入户并安装模版id
    pub song_huo_an_zhuang: Option<String>,
    /// 送货入户模版id
    pub song_huo_ru_hu: Option<String>,
    /// 商品短标题（仅在部分活动中生效），字数限制为4-20字
    pub tiny_name: Option<String>,
    /// 满2件折扣，可选范围0-100, 0表示取消，95表示95折，设置需先查询规则接口获取实际可填范围
    pub two_pieces_discount: Option<i32>,
    /// 保税仓，只在goods_type为直供商品时有效（现阶段暂不支持）
    pub warehouse: Option<String>,
    /// 水果类目温馨提示，只在水果类目商品才生效， 字数限制：商品描述goods_desc+温馨提示总计不超过500字。
    pub warm_tips: Option<String>,
    /// 只换不修的天数，目前只支持0和365
    pub zhi_huan_bu_xiu: Option<i32>,
    /// 保密发货，0:不支持，1:支持
    pub privacy_delivery: Option<i32>,
}

impl RequestType for PddGoodsEditGoodsCommitRequest {
    type Response = PddGoodsEditGoodsCommitResponse;

    fn get_type(&self) -> &'static str {
        "pdd.goods.edit.goods.commit"
    }
}

#[derive(Debug, Deserialize)]
pub struct PddGoodsEditGoodsCommitResponse {
    /// 草稿id
    #[serde(default)]
    pub goods_commit_id: i64,
    /// 商品id
    #[serde(default)]
    pub goods_id: i64,
    /// 商品匹配到的标品ID
    #[serde(default)]
    pub matched_spu_id: i64,
}
