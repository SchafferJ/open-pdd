//! # 多多进宝商品推荐API
//!
//! 多多进宝商品推荐API
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkGoodsRecommendGetRequest {
    /// 活动商品标记数组，例：[4,7]，4-秒杀，7-百亿补贴，10851-千万补贴，11879-千万神券，10913-招商礼金商品，31-品牌黑标，10564-精选爆品-官方直推爆款，10584-精选爆品-团长推荐，24-品牌高佣，其他的值请忽略
    pub activity_tags: Option<Vec<i32>>,
    /// 猜你喜欢场景的商品类目，商品类目ID，使用pdd.goods.cats.get接口获取
    pub cat_id: Option<i64>,
    /// 进宝频道推广商品: 1-今日销量榜,3-相似商品推荐,4-猜你喜欢(和进宝网站精选一致),5-实时热销榜,6-实时收益榜。默认值5
    pub channel_type: Option<i32>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 为用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 为上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。
    pub custom_parameters: Option<String>,
    /// 商品主图类型：1-场景图，2-白底图，默认为0
    pub goods_img_type: Option<i32>,
    /// 商品goodsSign列表，相似商品推荐场景时必传，仅取数组的第一位，例如：["c9r2omogKFFAc7WBwvbZU1ikIb16_J3CTa8HNN"]。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign_list: Option<Vec<String>>,
    /// 一页请求数量；默认值 ： 20
    pub limit: Option<i32>,
    /// 翻页时建议填写前页返回的list_id值
    pub list_id: Option<String>,
    /// 从多少位置开始请求；默认值 ： 0，offset需是limit的整数倍，仅支持整页翻页
    pub offset: Option<i32>,
    /// 推广位id
    pub pid: Option<String>,
    /// 风控参数
    pub risk_params: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

impl RequestType for PddDdkGoodsRecommendGetRequest {
    type Response = PddDdkGoodsRecommendGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.goods.recommend.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct List {
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
    /// 商品类目id
    #[serde(default)]
    pub cat_id: String,
    /// 商品一~四级类目ID列表
    #[serde(default)]
    pub cat_ids: Vec<i64>,
    /// 优惠券面额,单位为分
    #[serde(default)]
    pub coupon_discount: i64,
    /// 优惠券失效时间,UNIX时间戳
    #[serde(default)]
    pub coupon_end_time: i64,
    /// 优惠券门槛价格,单位为分
    #[serde(default)]
    pub coupon_min_order_amount: i64,
    /// 优惠券金额
    #[serde(default)]
    pub coupon_price: i64,
    /// 优惠券剩余数量
    #[serde(default)]
    pub coupon_remain_quantity: i64,
    /// 优惠券生效时间,UNIX时间戳
    #[serde(default)]
    pub coupon_start_time: i64,
    /// 优惠券总数量
    #[serde(default)]
    pub coupon_total_quantity: i64,
    /// 创建时间
    #[serde(default)]
    pub create_at: i64,
    /// 描述分
    #[serde(default)]
    pub desc_txt: String,
    /// 额外优惠券，单位为分
    #[serde(default)]
    pub extra_coupon_amount: i64,
    /// 商品描述
    #[serde(default)]
    pub goods_desc: String,
    /// 商品主图
    #[serde(default)]
    pub goods_image_url: String,
    /// 商品特殊标签列表。例: [1]，1-APP专享
    #[serde(default)]
    pub goods_labels: Vec<i32>,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 商品等级
    #[serde(default)]
    pub goods_rate: i64,
    /// 商品goodsSign，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    #[serde(default)]
    pub goods_sign: String,
    /// 商品缩略图
    #[serde(default)]
    pub goods_thumbnail_url: String,
    /// 商品类型
    #[serde(default)]
    pub goods_type: i32,
    /// 商品是否带券,true-带券,false-不带券
    #[serde(default)]
    pub has_coupon: bool,
    /// 商品是否有素材(图文、视频)
    #[serde(default)]
    pub has_material: bool,
    /// 物流分
    #[serde(default)]
    pub lgst_txt: String,
    /// 商家id
    #[serde(default)]
    pub mall_id: i64,
    /// 店铺名称
    #[serde(default)]
    pub mall_name: String,
    /// 市场服务费
    #[serde(default)]
    pub market_fee: i64,
    /// 商家类型
    #[serde(default)]
    pub merchant_type: String,
    /// 最小成团价格，单位分
    #[serde(default)]
    pub min_group_price: i64,
    /// 最小单买价格，单位分
    #[serde(default)]
    pub min_normal_price: i64,
    /// 商品标签类目ID,使用pdd.goods.opt.get获取
    #[serde(default)]
    pub opt_id: String,
    /// 商品一~四级标签类目ID列表
    #[serde(default)]
    pub opt_ids: Vec<i64>,
    /// 商品标签名
    #[serde(default)]
    pub opt_name: String,
    /// 比价行为预判定佣金，需要用户备案
    #[serde(default)]
    pub predict_promotion_rate: i64,
    /// 佣金比例,千分比
    #[serde(default)]
    pub promotion_rate: i64,
    /// 二维码主图
    #[serde(default)]
    pub qr_code_image_url: String,
    /// 商品近1小时在多多进宝的实时销量（仅实时热销榜提供）
    #[serde(default)]
    pub realtime_sales_tip: String,
    /// 销售量
    #[serde(default)]
    pub sales_tip: String,
    /// 搜索id，建议生成推广链接时候填写，提高收益。
    #[serde(default)]
    pub search_id: String,
    /// 服务分
    #[serde(default)]
    pub serv_txt: String,
    /// 分享描述
    #[serde(default)]
    pub share_desc: String,
    /// 招商分成服务费比例，千分比
    #[serde(default)]
    pub share_rate: i32,
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
}

#[derive(Debug, Deserialize)]
pub struct PddDdkGoodsRecommendGetResponse {
    /// 列表
    #[serde(default)]
    pub list: Option<Vec<List>>,
    /// 翻页时必填前页返回的list_id值
    #[serde(default)]
    pub list_id: String,
    /// 搜索id，建议生成推广链接时候填写，提高收益。
    #[serde(default)]
    pub search_id: String,
    /// total
    #[serde(default)]
    pub total: i32,
}
