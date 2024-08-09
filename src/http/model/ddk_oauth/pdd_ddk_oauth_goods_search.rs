//! # 多多进宝商品查询
//!
//! 多多进宝商品查询
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Range {
    /// 区间的开始值
    pub range_from: Option<i64>,
    /// 0，最小成团价 1，券后价 2，佣金比例 3，优惠券价格 4，广告创建时间 5，销量 6，佣金金额 7，店铺描述分 8，店铺物流分 9，店铺服务分 10， 店铺描述分击败同行业百分比 11， 店铺物流分击败同行业百分比 12，店铺服务分击败同行业百分比 13，商品分 17 ，优惠券/最小团购价 18，过去两小时pv 19，过去两小时销量
    pub range_id: Option<i32>,
    /// 区间的结束值
    pub range_to: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PddDdkOauthGoodsSearchRequest {
    /// 活动商品标记数组，例：[4,7]，4-秒杀，7-百亿补贴，10851-千万补贴，11879-千万神券，10913-招商礼金商品，31-品牌黑标，10564-精选爆品-官方直推爆款，10584-精选爆品-团长推荐，24-品牌高佣，其他的值请忽略
    pub activity_tags: Option<Vec<i32>>,
    /// 屏蔽商品类目包：1-拼多多小程序屏蔽的类目&关键词;2-虚拟类目;3-医疗器械;4-处方药;5-非处方药
    pub block_cat_packages: Option<Vec<i32>>,
    /// 自定义屏蔽一级/二级/三级类目ID，自定义数量不超过20个;使用pdd.goods.cats.get接口获取cat_id
    pub block_cats: Option<Vec<i32>>,
    /// 商品类目ID，使用pdd.goods.cats.get接口获取
    pub cat_id: Option<i64>,
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为：  {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。（如果使用GET请求，请使用URLEncode处理参数）
    pub custom_parameters: Option<String>,
    /// 是否使用工具商专属推广计划，默认为false
    pub force_auth_duo_id: Option<bool>,
    /// 商品主图类型：1-场景图，2-白底图，默认为0
    pub goods_img_type: Option<i32>,
    /// 商品goodsSign列表，例如：["c9r2omogKFFAc7WBwvbZU1ikIb16_J3CTa8HNN"]，支持通过goodsSign查询商品。goodsSign是加密后的goodsId, goodsId已下线，请使用goodsSign来替代。使用说明：https://jinbao.pinduoduo.com/qa-system?questionId=252
    pub goods_sign_list: Option<Vec<String>>,
    /// 是否为品牌商品
    pub is_brand_goods: Option<bool>,
    /// 商品关键词，与opt_id字段选填一个或全部填写。可支持goods_id、拼多多链接（即拼多多app商详的链接）、进宝长链/短链（即为pdd.ddk.goods.promotion.url.generate接口生成的长短链）
    pub keyword: Option<String>,
    /// 翻页时建议填写前页返回的list_id值
    pub list_id: Option<String>,
    /// 店铺类型，1-个人，2-企业，3-旗舰店，4-专卖店，5-专营店，6-普通店（未传为全部）
    pub merchant_type: Option<i32>,
    /// 店铺类型数组，例如：[1,2]
    pub merchant_type_list: Option<Vec<i32>>,
    /// 商品标签类目ID，使用pdd.goods.opt.get获取
    pub opt_id: Option<i64>,
    /// 默认值1，商品分页数
    pub page: Option<i32>,
    /// 默认100，每页商品数量
    pub page_size: Option<i32>,
    /// 推广位id
    pub pid: Option<String>,
    /// 筛选范围列表 样例：[{"range_id":0,"range_from":1,"range_to":1500},{"range_id":1,"range_from":1,"range_to":1500}]
    pub range_list: Option<Vec<Range>>,
    /// 排序方式:0-综合排序;1-按佣金比率升序;2-按佣金比例降序;3-按价格升序;4-按价格降序;5-按销量升序;6-按销量降序;7-优惠券金额排序升序;8-优惠券金额排序降序;9-券后价升序排序;10-券后价降序排序;11-按照加入多多进宝时间升序;12-按照加入多多进宝时间降序;13-按佣金金额升序排序;14-按佣金金额降序排序;15-店铺描述评分升序;16-店铺描述评分降序;17-店铺物流评分升序;18-店铺物流评分降序;19-店铺服务评分升序;20-店铺服务评分降序;27-描述评分击败同类店铺百分比升序，28-描述评分击败同类店铺百分比降序，29-物流评分击败同类店铺百分比升序，30-物流评分击败同类店铺百分比降序，31-服务评分击败同类店铺百分比升序，32-服务评分击败同类店铺百分比降序
    pub sort_type: Option<i32>,
    /// 是否使用个性化推荐，true表示使用，false表示不使用，默认true。
    pub use_customized: Option<bool>,
    /// 是否只返回优惠券的商品，false返回所有商品，true只返回有优惠券的商品
    pub with_coupon: Option<bool>,
    /// 风控参数
    pub risk_params: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

impl RequestType for PddDdkOauthGoodsSearchRequest {
    type Response = PddDdkOauthGoodsSearchResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.oauth.goods.search"
    }
}

#[derive(Debug, Deserialize)]
pub struct Goods {
    /// 活动佣金比例，千分比（特定活动期间的佣金比例）
    #[serde(default)]
    pub activity_promotion_rate: i64,
    /// 商品活动标记数组，例：[4,7]，4-秒杀 7-百亿补贴等
    #[serde(default)]
    pub activity_tags: Vec<i32>,
    /// 活动类型，0-无活动;1-秒杀;3-限量折扣;12-限时折扣;13-大促活动;14-名品折扣;15-品牌清仓;16-食品超市;17-一元幸运团;18-爱逛街;19-时尚穿搭;20-男人帮;21-9块9;22-竞价活动;23-榜单活动;24-幸运半价购;25-定金预售;26-幸运人气购;27-特色主题活动;28-断码清仓;29-一元话费;30-电器城;31-每日好店;32-品牌卡;101-大促搜索池;102-大促品类分会场;
    #[serde(default)]
    pub activity_type: i32,
    /// 商品品牌词信息，如“苹果”、“阿迪达斯”、“李宁”等
    #[serde(default)]
    pub brand_name: String,
    /// 全局礼金金额，单位分
    #[serde(default)]
    pub cash_gift_amount: i64,
    /// 商品类目id
    #[serde(default)]
    pub cat_ids: Vec<i64>,
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
    /// 优惠券门槛价格，单位为分
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
    /// 商品是否有素材(图文、视频)
    #[serde(default)]
    pub has_material: bool,
    /// 是否多人团
    #[serde(default)]
    pub is_multi_group: bool,
    /// 物流分
    #[serde(default)]
    pub lgst_txt: String,
    /// 店铺券折扣
    #[serde(default)]
    pub mall_coupon_discount_pct: i32,
    /// 店铺券结束使用时间
    #[serde(default)]
    pub mall_coupon_end_time: i64,
    /// 店铺券id
    #[serde(default)]
    pub mall_coupon_id: i64,
    /// 最大使用金额
    #[serde(default)]
    pub mall_coupon_max_discount_amount: i32,
    /// 最小使用金额
    #[serde(default)]
    pub mall_coupon_min_order_amount: i32,
    /// 店铺券余量
    #[serde(default)]
    pub mall_coupon_remain_quantity: i64,
    /// 店铺券开始使用时间
    #[serde(default)]
    pub mall_coupon_start_time: i64,
    /// 店铺券总量
    #[serde(default)]
    pub mall_coupon_total_quantity: i64,
    /// 该商品所在店铺是否参与全店推广，0：否，1：是
    #[serde(default)]
    pub mall_cps: i32,
    /// 店铺id
    #[serde(default)]
    pub mall_id: i64,
    /// 店铺名字
    #[serde(default)]
    pub mall_name: String,
    /// 店铺类型，1-个人，2-企业，3-旗舰店，4-专卖店，5-专营店，6-普通店
    #[serde(default)]
    pub merchant_type: i32,
    /// 最小拼团价（单位为分）
    #[serde(default)]
    pub min_group_price: i64,
    /// 最小单买价格（单位为分）
    #[serde(default)]
    pub min_normal_price: i64,
    /// 快手专享
    #[serde(default)]
    pub only_scene_auth: bool,
    /// 商品标签ID，使用pdd.goods.opts.get接口获取
    #[serde(default)]
    pub opt_id: i64,
    /// 商品标签id
    #[serde(default)]
    pub opt_ids: Vec<i64>,
    /// 商品标签名
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
    /// 搜索id，建议生成推广链接时候填写，提高收益
    #[serde(default)]
    pub search_id: String,
    /// 服务分
    #[serde(default)]
    pub serv_txt: String,
    /// 服务标签: 1-全场包邮,2-七天退换,3-退货包运费,4-送货入户并安装,5-送货入户,6-电子发票,7-诚信发货,8-缺重包赔,9-坏果包赔,10-果重保证,11-闪电退款,12-24小时发货,13-48小时发货,14-免税费,15-假一罚十,16-贴心服务,17-顺丰包邮,18-只换不修,19-全国联保,20-分期付款,21-纸质发票,22-上门安装,23-爱心助农,24-极速退款,25-品质保障,26-缺重包退,27-当日发货,28-可定制化,29-预约配送,30-商品进口,31-电器城,1000001-正品发票,1000002-送货入户并安装,2000001-价格保护
    #[serde(default)]
    pub service_tags: Vec<i64>,
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
    /// 招商团长id
    #[serde(default)]
    pub zs_duo_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkOauthGoodsSearchResponse {
    /// 商品列表
    #[serde(default)]
    pub goods_list: Option<Vec<Goods>>,
    /// 翻页时必填前页返回的list_id值
    #[serde(default)]
    pub list_id: String,
    /// 搜索id，建议生成推广链接时候填写，提高收益
    #[serde(default)]
    pub search_id: String,
    /// 返回商品总数
    #[serde(default)]
    pub total_count: i32,
}
