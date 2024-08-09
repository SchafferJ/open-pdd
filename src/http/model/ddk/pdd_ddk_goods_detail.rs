//! # 多多进宝商品详情查询
//!
//! 查询多多进宝商品详情
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkGoodsDetailRequest {
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。（如果使用GET请求，请使用URLEncode处理参数）
    pub custom_parameters: Option<String>,
    /// 商品主图类型：1-场景图，2-白底图，默认为0
    pub goods_img_type: Option<i32>,
    /// 商品goodsSign，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign: Option<String>,
    /// 是否获取sku信息，默认false不返回。（特殊渠道权限，需额外申请）
    pub need_sku_info: Option<bool>,
    /// 推广位id
    pub pid: Option<String>,
    /// 搜索id，建议填写，提高收益。来自pdd.ddk.goods.recommend.get、pdd.ddk.goods.search、pdd.ddk.top.goods.list.query等接口
    pub search_id: Option<String>,
    /// 招商多多客ID
    pub zs_duo_id: Option<i64>,
    /// 风控参数
    pub risk_params: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

impl RequestType for PddDdkGoodsDetailRequest {
    type Response = PddDdkGoodsDetailResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.goods.detail"
    }
}

#[derive(Debug, Deserialize)]
pub struct Material {
    /// 素材ID
    #[serde(default)]
    pub id: String,
    /// 图片列表
    #[serde(default)]
    pub image_list: Vec<String>,
    /// 文字列表
    #[serde(default)]
    pub text_list: Vec<String>,
    /// 视频缩略图
    #[serde(default)]
    pub thumbnail_url: String,
    /// 素材类型，1-图文，2-视频
    #[serde(default)]
    pub r#type: i32,
    /// 视频url
    #[serde(default)]
    pub video_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    /// 规格备注
    #[serde(default)]
    pub note: String,
    /// 父规格id
    #[serde(default)]
    pub parent_spec_id: i64,
    /// 父规格名称。例如："型号"为父规格名称，"xx款"为该父规格下的子规格名称。
    #[serde(default)]
    pub parent_spec_value: String,
    /// 规格id
    #[serde(default)]
    pub spec_id: i64,
    /// 规格名称。例如："xx款"为规格名称, "型号"为该规格的父规格名称。
    #[serde(default)]
    pub spec_value: String,
}

#[derive(Debug, Deserialize)]
pub struct Sku {
    /// 上下架状态: 1-上架, 0-下架
    #[serde(default)]
    pub is_onsale: i32,
    /// 进宝算价结果，单位分
    #[serde(default)]
    pub jinbao_goods_price: i64,
    /// 最小成团价，单位分
    #[serde(default)]
    pub min_group_price: i64,
    /// skuId密文，可在pdd.ddk.goods.promotion.url.generate接口入参，返回的链接会拼上该skuIdCode。此类链接在被点击跳转商品详情页时，如果目标sku可用，则自动选中该sku
    #[serde(default)]
    pub sku_id_code: String,
    /// sku预览图
    #[serde(default)]
    pub sku_thumb_url: String,
    /// 商品规格列表
    #[serde(default)]
    pub spec_list: Option<Vec<Spec>>,
}

#[derive(Debug, Deserialize)]
pub struct GoodsDetails {
    /// 活动佣金比例，千分比（特定活动期间的佣金比例）
    #[serde(default)]
    pub activity_promotion_rate: i64,
    /// 商品活动标记数组，例：[4,7]，4-秒杀 7-百亿补贴等
    #[serde(default)]
    pub activity_tags: Vec<i32>,
    /// 商品品牌词信息，如“苹果”、“阿迪达斯”、“李宁”等
    #[serde(default)]
    pub brand_name: String,
    /// 全局礼金金额，单位分
    #[serde(default)]
    pub cash_gift_amount: i64,
    /// 商品类目ID，使用pdd.goods.cats.get接口获取
    #[serde(default)]
    pub cat_id: i64,
    /// 商品一~四级类目ID列表
    #[serde(default)]
    pub cat_ids: Vec<i32>,
    /// 店铺收藏券id
    #[serde(default)]
    pub clt_cpn_batch_sn: String,
    /// 店铺收藏券面额,单位为分
    #[serde(default)]
    pub clt_cpn_discount: i64,
    /// 店铺收藏券截止时间
    #[serde(default)]
    pub clt_cpn_end_time: i64,
    /// 店铺收藏券使用门槛价格,单位为分
    #[serde(default)]
    pub clt_cpn_min_amt: i64,
    /// 店铺收藏券总量
    #[serde(default)]
    pub clt_cpn_quantity: i64,
    /// 店铺收藏券剩余量
    #[serde(default)]
    pub clt_cpn_remain_quantity: i64,
    /// 店铺收藏券起始时间
    #[serde(default)]
    pub clt_cpn_start_time: i64,
    /// 优惠券面额，单位为分
    #[serde(default)]
    pub coupon_discount: i64,
    /// 优惠券失效时间，UNIX时间戳
    #[serde(default)]
    pub coupon_end_time: i64,
    /// 优惠券门槛金额，单位为分
    #[serde(default)]
    pub coupon_min_order_amount: i64,
    /// 优惠券剩余数量
    #[serde(default)]
    pub coupon_remain_quantity: i64,
    /// 优惠券生效时间，UNIX时间戳
    #[serde(default)]
    pub coupon_start_time: i64,
    /// 优惠券总数量
    #[serde(default)]
    pub coupon_total_quantity: i64,
    /// 创建时间（unix时间戳）
    #[serde(default)]
    pub create_at: i64,
    /// 描述分
    #[serde(default)]
    pub desc_txt: String,
    /// 额外优惠券
    #[serde(default)]
    pub extra_coupon_amount: i64,
    /// 参与多多进宝的商品描述
    #[serde(default)]
    pub goods_desc: String,
    /// 商品轮播图
    #[serde(default)]
    pub goods_gallery_urls: Vec<String>,
    /// 多多进宝商品主图
    #[serde(default)]
    pub goods_image_url: String,
    /// 参与多多进宝的商品标题
    #[serde(default)]
    pub goods_name: String,
    /// 商品goodsSign，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    #[serde(default)]
    pub goods_sign: String,
    /// 商品缩略图
    #[serde(default)]
    pub goods_thumbnail_url: String,
    /// 商品是否有优惠券 true-有，false-没有
    #[serde(default)]
    pub has_coupon: bool,
    /// 是否有店铺券
    #[serde(default)]
    pub has_mall_coupon: bool,
    /// 是否多人团
    #[serde(default)]
    pub is_multi_group: bool,
    /// 物流分
    #[serde(default)]
    pub lgst_txt: String,
    /// 店铺折扣
    #[serde(default)]
    pub mall_coupon_discount_pct: i32,
    /// 店铺券使用结束时间
    #[serde(default)]
    pub mall_coupon_end_time: i64,
    /// 最大使用金额
    #[serde(default)]
    pub mall_coupon_max_discount_amount: i32,
    /// 最小使用金额
    #[serde(default)]
    pub mall_coupon_min_order_amount: i32,
    /// 店铺券余量
    #[serde(default)]
    pub mall_coupon_remain_quantity: i64,
    /// 店铺券使用开始时间
    #[serde(default)]
    pub mall_coupon_start_time: i64,
    /// 店铺券总量
    #[serde(default)]
    pub mall_coupon_total_quantity: i64,
    /// 该商品所在店铺是否参与全店推广，0：否，1：是
    #[serde(default)]
    pub mall_cps: i32,
    /// 商家id
    #[serde(default)]
    pub mall_id: i64,
    /// 店铺logo图
    #[serde(default)]
    pub mall_img_url: String,
    /// 店铺名称
    #[serde(default)]
    pub mall_name: String,
    /// 商品素材列表
    #[serde(default)]
    pub material_list: Option<Vec<Material>>,
    /// 店铺类型，1-个人，2-企业，3-旗舰店，4-专卖店，5-专营店，6-普通店（未传为全部）
    #[serde(default)]
    pub merchant_type: i32,
    /// 最低价sku的拼团价，单位为分
    #[serde(default)]
    pub min_group_price: i64,
    /// 最低价sku的单买价，单位为分
    #[serde(default)]
    pub min_normal_price: i64,
    /// 快手专享
    #[serde(default)]
    pub only_scene_auth: bool,
    /// 商品标签ID，使用pdd.goods.opt.get接口获取
    #[serde(default)]
    pub opt_id: i64,
    /// 商品标签ID
    #[serde(default)]
    pub opt_ids: Vec<i32>,
    /// 商品标签名称
    #[serde(default)]
    pub opt_name: String,
    /// 推广计划类型: 1-全店推广 2-单品推广 3-定向推广 4-招商推广 5-分销推广
    #[serde(default)]
    pub plan_type: i32,
    /// 比价行为预判定佣金，需要用户备案
    #[serde(default)]
    pub predict_promotion_rate: i64,
    /// 佣金比例，千分比
    #[serde(default)]
    pub promotion_rate: i64,
    /// 已售卖件数
    #[serde(default)]
    pub sales_tip: String,
    /// 服务分
    #[serde(default)]
    pub serv_txt: String,
    /// 服务标签: 1-全场包邮,2-七天退换,3-退货包运费,4-送货入户并安装,5-送货入户,6-电子发票,7-诚信发货,8-缺重包赔,9-坏果包赔,10-果重保证,11-闪电退款,12-24小时发货,13-48小时发货,14-免税费,15-假一罚十,16-贴心服务,17-顺丰包邮,18-只换不修,19-全国联保,20-分期付款,21-纸质发票,22-上门安装,23-爱心助农,24-极速退款,25-品质保障,26-缺重包退,27-当日发货,28-可定制化,29-预约配送,30-商品进口,31-电器城,1000001-正品发票,1000002-送货入户并安装,2000001-价格保护
    #[serde(default)]
    pub service_tags: Vec<i32>,
    /// 招商分成服务费比例，千分比
    #[serde(default)]
    pub share_rate: i32,
    /// sku列表
    #[serde(default)]
    pub sku_list: Option<Vec<Sku>>,
    /// 优势渠道专属商品补贴金额，单位为分。针对优质渠道的补贴活动，指定优势渠道可通过推广该商品获取相应补贴。补贴活动入口：[进宝网站-官方活动]
    #[serde(default)]
    pub subsidy_amount: i32,
    /// 官方活动给渠道的收入补贴金额，不允许直接给下级代理展示，单位为分
    #[serde(default)]
    pub subsidy_duo_amount_ten_million: i32,
    /// 补贴活动类型：0-无补贴，1-千万补贴，4-千万神券，6-佣金翻倍
    #[serde(default)]
    pub subsidy_goods_type: i32,
    /// 优惠标签列表，包括："X元券","比全网低X元","服务费","精选素材","近30天低价","同款低价","同款好评","同款热销","旗舰店","一降到底","招商优选","商家优选","好价再降X元","全站销量XX","实时热销榜第X名","实时好评榜第X名","额外补X元"等
    #[serde(default)]
    pub unified_tags: Vec<String>,
    /// 商品视频url
    #[serde(default)]
    pub video_urls: Vec<String>,
    /// 招商团长id
    #[serde(default)]
    pub zs_duo_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkGoodsDetailResponse {
    /// 多多进宝商品对象列表
    #[serde(default)]
    pub goods_details: Option<Vec<GoodsDetails>>,
}
