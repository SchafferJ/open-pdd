//! # 订单详情
//!
//! 查询单个订单详情（只能获取到成交时间三个月以内的交易信息）
//! 注：虚拟订单充值手机号信息无法通过此接口获取，请联系虚拟类目运营人员。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddOrderInformationGetRequest {
    /// 订单号
    pub order_sn: String,
}

impl RequestType for PddOrderInformationGetRequest {
    type Response = PddOrderInformationGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.order.information.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct CardInfo {
    /// 卡号
    #[serde(default)]
    pub card_no: String,
    /// 卡密
    #[serde(default)]
    pub mask_password: String,
}

#[derive(Debug, Deserialize)]
pub struct ConsolidateInfo {
    /// 集运类型 0-中国香港集运、1-中国新疆中转、2-哈萨克斯坦集运、3-中国西藏中转、5-日本集运、6-中国台湾集运、7-韩国集运、8-新加坡集运、9-马来西亚集运、10-泰国集运、11-越南集运、12-吉尔吉斯斯坦集运、13-乌兹别克斯坦集运、14-中国甘肃中转、15-中国内蒙古中转、16-中国宁夏中转、17-中国青海中转、18-中国澳门集运、19-柬埔寨集运、20-老挝集运、21-塔吉克斯坦集运、22-亚美尼亚集运、23-格鲁吉亚集运
    #[serde(default)]
    pub consolidate_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct ExtraDelivery {
    /// 快递公司编号
    #[serde(default)]
    pub logistics_id: i32,
    /// 快递运单号
    #[serde(default)]
    pub tracking_number: String,
}

#[derive(Debug, Deserialize)]
pub struct GiftDelivery {
    /// 快递ID
    #[serde(default)]
    pub logistics_id: i32,
    /// 运单号
    #[serde(default)]
    pub tracking_number: String,
}

#[derive(Debug, Deserialize)]
pub struct Gift {
    /// 赠品数量
    #[serde(default)]
    pub goods_count: i32,
    /// 赠品id
    #[serde(default)]
    pub goods_id: i64,
    /// 赠品图片
    #[serde(default)]
    pub goods_img: String,
    /// 赠品名称
    #[serde(default)]
    pub goods_name: String,
    /// 赠品销售价格
    #[serde(default)]
    pub goods_price: f64,
    /// 赠品规格
    #[serde(default)]
    pub goods_spec: String,
    /// 商家外部商品编码
    #[serde(default)]
    pub outer_goods_id: String,
    /// 商家外部sku编码
    #[serde(default)]
    pub outer_id: String,
    /// 赠品规格编码
    #[serde(default)]
    pub sku_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    /// 商品数量
    #[serde(default)]
    pub goods_count: i32,
    /// 商品编号
    #[serde(default)]
    pub goods_id: i64,
    /// 商品图片
    #[serde(default)]
    pub goods_img: String,
    /// 商品名称
    #[serde(default)]
    pub goods_name: String,
    /// 商品销售价格
    #[serde(default)]
    pub goods_price: f64,
    /// 商品规格，使用（规格值1,规格值2）组合作为sku的表示，中间以英文逗号隔开
    #[serde(default)]
    pub goods_spec: String,
    /// 商家外部编码（商品），注意：编辑商品后必须等待商品审核通过后方可生效，订单中商品信息为交易快照的商品信息。
    #[serde(default)]
    pub outer_goods_id: String,
    /// 商家外部编码（sku），注意：编辑商品后必须等待商品审核通过后方可生效，订单中商品信息为交易快照的商品信息。
    #[serde(default)]
    pub outer_id: String,
    /// 商品规格编码
    #[serde(default)]
    pub sku_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct WareSubInfo {
    /// 子货品id
    #[serde(default)]
    pub ware_id: i64,
    /// 子货品1编码
    #[serde(default)]
    pub ware_name: String,
    /// 子货品数量
    #[serde(default)]
    pub ware_quantity: i64,
    /// 子货品编码
    #[serde(default)]
    pub ware_sn: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderDepotInfo {
    /// 仓库编码
    #[serde(default)]
    pub depot_code: String,
    /// 仓库id
    #[serde(default)]
    pub depot_id: String,
    /// 仓库名称
    #[serde(default)]
    pub depot_name: String,
    /// 仓库类型，1：自有仓 2：订阅仓 两者都不是则传空
    #[serde(default)]
    pub depot_type: i32,
    /// 货品id
    #[serde(default)]
    pub ware_id: String,
    /// 货品名称
    #[serde(default)]
    pub ware_name: String,
    /// 货品编码
    #[serde(default)]
    pub ware_sn: String,
    /// 子货品列表（组合货品才会有子货品信息）
    #[serde(default)]
    pub ware_sub_info_list: Option<Vec<WareSubInfo>>,
    /// 货品类型（0：普通货品，1：组合货品）
    #[serde(default)]
    pub ware_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct OrderTag {
    /// 标签名称
    #[serde(default)]
    pub name: String,
    /// 是否有标签：0=无标签，1=有标签
    #[serde(default)]
    pub value: i32,
}

#[derive(Debug, Deserialize)]
pub struct PromotionDetail {
    /// 优惠金额（元）
    #[serde(default)]
    pub discount_amount: f64,
    /// 优惠券类型。30-以旧换新优惠（优惠金额已包含平台优惠金额里）
    #[serde(default)]
    pub promotion_type: i32,
}

#[derive(Debug, Deserialize)]
pub struct ResendDelivery {
    /// 快递ID
    #[serde(default)]
    pub logistics_id: i32,
    /// 运单号
    #[serde(default)]
    pub tracking_number: String,
}

#[derive(Debug, Deserialize)]
pub struct ServiceFeeDetail {
    /// 服务费金额，单位：元
    #[serde(default)]
    pub service_fee: f64,
    /// 服务费类型
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Deserialize)]
pub struct StepOrderInfo {
    /// 已付定金 单位：元
    #[serde(default)]
    pub advanced_paid_fee: f64,
    /// 膨胀金额 （包含券减） 单位：元
    #[serde(default)]
    pub step_discount_amount: f64,
    /// 分阶段已付金额（定金+尾款） 单位：元
    #[serde(default)]
    pub step_paid_fee: f64,
    /// 定金订单状态：step_trade_status 枚举：0-定金未付尾款未付、1-定金已付尾款未付、2-定金已付尾款已付
    #[serde(default)]
    pub step_trade_status: i32,
}

#[derive(Debug, Deserialize)]
pub struct StoreInfo {
    /// 门店id
    #[serde(default)]
    pub store_id: i64,
    /// 门店名称
    #[serde(default)]
    pub store_name: String,
    /// 门店自定义编码
    #[serde(default)]
    pub store_number: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderInfo {
    /// 收件详细地址
    #[serde(default)]
    pub address: String,
    /// 详细地址（打码）
    #[serde(default)]
    pub address_mask: String,
    /// 售后状态 0：无售后 2：买家申请退款，待商家处理 3：退货退款，待商家处理 4：商家同意退款，退款中 5：平台同意退款，退款中 6：驳回退款，待买家处理 7：已同意退货退款,待用户发货 8：平台处理中 9：平台拒绝退款，退款关闭 10：退款成功 11：买家撤销 12：买家逾期未处理，退款失败 13：买家逾期，超过有效期 14：换货补寄待商家处理 15：换货补寄待用户处理 16：换货补寄成功 17：换货补寄失败 18：换货补寄待用户确认完成 21：待商家同意维修 22：待用户确认发货 24：维修关闭 25：维修成功 27：待用户确认收货 31：已同意拒收退款，待用户拒收 32：补寄待商家发货 33：同意召回后退款，待商家召回
    #[serde(default)]
    pub after_sales_status: i32,
    /// 保税仓名称
    #[serde(default)]
    pub bonded_warehouse: String,
    /// 买家留言信息
    #[serde(default)]
    pub buyer_memo: String,
    /// 团长免单优惠金额，只在团长免单活动中才会返回优惠金额
    #[serde(default)]
    pub capital_free_discount: f64,
    /// 卡号信息列表
    #[serde(default)]
    pub card_info_list: Option<Vec<CardInfo>>,
    /// 商品一级分类
    #[serde(default)]
    pub cat_id_1: i64,
    /// 商品二级分类
    #[serde(default)]
    pub cat_id_2: i64,
    /// 商品三级分类
    #[serde(default)]
    pub cat_id_3: i64,
    /// 商品四级分类
    #[serde(default)]
    pub cat_id_4: i64,
    /// 收件地城市
    #[serde(default)]
    pub city: String,
    /// 城市编码
    #[serde(default)]
    pub city_id: i32,
    /// 成交状态：0：未成交、1：已成交、2：已取消
    #[serde(default)]
    pub confirm_status: i32,
    /// 成交时间
    #[serde(default)]
    pub confirm_time: String,
    /// 集运信息
    #[serde(default)]
    pub consolidate_info: Option<ConsolidateInfo>,
    /// 收件地国家或地区
    #[serde(default)]
    pub country: String,
    /// 国家或地区编码
    #[serde(default)]
    pub country_id: i32,
    /// 创建时间
    #[serde(default)]
    pub created_time: String,
    /// 送货入户费用 单位：元
    #[serde(default)]
    pub delivery_home_value: f64,
    /// 送货入户并安装 单位：元
    #[serde(default)]
    pub delivery_install_value: f64,
    /// 是否当日发货，1-是，0-否
    #[serde(default)]
    pub delivery_one_day: i32,
    /// 折扣金额（元）折扣金额=平台优惠+商家优惠+团长免单优惠金额
    #[serde(default)]
    pub discount_amount: f64,
    /// 多多支付立减金额，单位：元
    #[serde(default)]
    pub duo_duo_pay_reduction: f64,
    /// 是否多多批发，1-是，0-否
    #[serde(default)]
    pub duoduo_wholesale: i32,
    /// 订单多包裹发货时使用的其他发货快递信息
    #[serde(default)]
    pub extra_delivery_list: Option<Vec<ExtraDelivery>>,
    /// 是否顺丰包邮 1表示是 0表示否
    #[serde(default)]
    pub free_sf: i32,
    /// 赠品额外运单列表
    #[serde(default)]
    pub gift_delivery_list: Option<Vec<GiftDelivery>>,
    /// 赠品列表
    #[serde(default)]
    pub gift_list: Option<Vec<Gift>>,
    /// 商品金额（元）商品金额=商品销售价格*商品数量-订单改价折扣金额
    #[serde(default)]
    pub goods_amount: f64,
    /// 团id
    #[serde(default)]
    pub group_order_id: i64,
    /// 团身份。0-团员，1-团长
    #[serde(default)]
    pub group_role: i32,
    /// 成团状态：0：拼团中、1：已成团、2：团失败
    #[serde(default)]
    pub group_status: i32,
    /// 送货入户并安装服务 0-不支持送货，1-送货入户不安装，2-送货入户并安装
    #[serde(default)]
    pub home_delivery_type: i32,
    /// 上门安装费用 单位：元
    #[serde(default)]
    pub home_install_value: f64,
    /// 支付申报订单号（多多国际清关请使用此字段，单号以XP开头）
    #[serde(default)]
    pub inner_transaction_id: String,
    /// 发票申请,1代表有 0代表无
    #[serde(default)]
    pub invoice_status: i32,
    /// 是否抽奖订单，1-非抽奖订单，2-抽奖订单
    #[serde(default)]
    pub is_lucky_flag: i32,
    /// 是否为预售商品 1表示是 0表示否
    #[serde(default)]
    pub is_pre_sale: i32,
    /// 是否缺货 0-无缺货处理 1： 有缺货处理
    #[serde(default)]
    pub is_stock_out: i32,
    /// 订单中商品sku列表
    #[serde(default)]
    pub item_list: Option<Vec<Item>>,
    /// 订单承诺发货时间
    #[serde(default)]
    pub last_ship_time: String,
    /// 快递公司编号
    #[serde(default)]
    pub logistics_id: i64,
    /// 市场业务类型，0-普通订单，1-拼内购订单
    #[serde(default)]
    pub mkt_biz_type: i32,
    /// 只换不修，1:是，0:否
    #[serde(default)]
    pub only_support_replace: i32,
    /// 合单ID
    #[serde(default)]
    pub open_address_id: String,
    /// 订单改价折扣金额，单位元
    #[serde(default)]
    pub order_change_amount: f64,
    /// 仓库信息
    #[serde(default)]
    pub order_depot_info: Option<OrderDepotInfo>,
    /// 订单编号
    #[serde(default)]
    pub order_sn: String,
    /// 发货状态，枚举值：1：待发货，2：已发货待签收，3：已签收
    #[serde(default)]
    pub order_status: i32,
    /// 订单标签列表，no_trace_delivery=无痕发货，only_support_replace=只换不修，duoduo_wholesale=多多批发，return_freight_payer=退货包运费，free_sf=顺丰包邮，support_nationwide_warranty=全国联保，self_contained=门店自提，delivery_one_day=当日发货，oversea_tracing=全球购溯源，distributional_sale=分销订单，open_in_festival=不打烊，region_black_delay_shipping=发货时间可延迟，same_city_distribution=同城配送，has_subsidy_postage=补贴运费红包，has_sf_express_service=顺丰加价，community_group=小区团购，has_ship_additional=加运费发顺丰，ship_additional_order=加运费补差价订单，conso_order=集运订单，allergy_refund=过敏包退，professional_appraisal=专业鉴定，ship_hold=暂停发货，home_delivery_door=送货上门，direct_mail_activity=直邮活动
    #[serde(default)]
    pub order_tag_list: Option<Vec<OrderTag>>,
    /// 支付金额（元）支付金额=商品金额-折扣金额+邮费+服务费
    #[serde(default)]
    pub pay_amount: f64,
    /// 支付单号
    #[serde(default)]
    pub pay_no: String,
    /// 支付时间
    #[serde(default)]
    pub pay_time: String,
    /// 支付方式，枚举值：QQ,WEIXIN,ALIPAY,LIANLIANPAY
    #[serde(default)]
    pub pay_type: String,
    /// 平台优惠金额
    #[serde(default)]
    pub platform_discount: f64,
    /// 邮费
    #[serde(default)]
    pub postage: f64,
    /// 预售时间
    #[serde(default)]
    pub pre_sale_time: String,
    /// 承诺送达时间
    #[serde(default)]
    pub promise_delivery_time: String,
    /// 优惠券信息
    #[serde(default)]
    pub promotion_detail_list: Option<Vec<PromotionDetail>>,
    /// 收件地省份
    #[serde(default)]
    pub province: String,
    /// 省份编码
    #[serde(default)]
    pub province_id: i32,
    /// 确认收货时间
    #[serde(default)]
    pub receive_time: String,
    /// 收件人地址，不拼接省市区。订单状态为待发货状态，且订单未被风控打标的情况下返回密文数据；其余情况返回空字符串。
    #[serde(default)]
    pub receiver_address: String,
    /// 收件人地址（打码）
    #[serde(default)]
    pub receiver_address_mask: String,
    /// 收件人姓名。订单状态为待发货状态，且订单未被风控打标的情况下返回密文数据；其余情况返回空字符串。
    #[serde(default)]
    pub receiver_name: String,
    /// 收件人姓名（打码）
    #[serde(default)]
    pub receiver_name_mask: String,
    /// 收件人电话。订单状态为待发货状态，且订单未被风控打标的情况下返回密文数据；其余情况返回空字符串。
    #[serde(default)]
    pub receiver_phone: String,
    /// 收件人手机号（打码）
    #[serde(default)]
    pub receiver_phone_mask: String,
    /// 退款状态，枚举值：1：无售后或售后关闭，2：售后处理中，3：退款中，4： 退款成功
    #[serde(default)]
    pub refund_status: i32,
    /// 商家订单备注
    #[serde(default)]
    pub remark: String,
    /// 订单备注标记，1-红色，2-黄色，3-绿色，4-蓝色，5-紫色
    #[serde(default)]
    pub remark_tag: i32,
    /// 订单备注标记名称
    #[serde(default)]
    pub remark_tag_name: String,
    /// 补寄额外运单列表
    #[serde(default)]
    pub resend_delivery_list: Option<Vec<ResendDelivery>>,
    /// 退货包运费，1:是，0:否
    #[serde(default)]
    pub return_freight_payer: i32,
    /// 订单审核状态（0-正常订单， 1-审核中订单）
    #[serde(default)]
    pub risk_control_status: i32,
    /// 是否门店自提，1-是，0-否
    #[serde(default)]
    pub self_contained: i32,
    /// 店铺优惠金额
    #[serde(default)]
    pub seller_discount: f64,
    /// 服务费明细列表，sf_express_fee=顺丰加价服务，install_fee=上门安装服务，store_install_fee=到店安装服务，take_to_store_install_fee=携货到店安装，dismantle_and_home_install_fee=拆旧+上门安装
    #[serde(default)]
    pub service_fee_detail: Option<Vec<ServiceFeeDetail>>,
    /// 关联的加运费发顺丰的补差价订单
    #[serde(default)]
    pub ship_additional_link_order: String,
    /// 加运费补差价订单的原单
    #[serde(default)]
    pub ship_additional_origin_order: String,
    /// 发货时间
    #[serde(default)]
    pub shipping_time: String,
    /// 创建交易时的物流方式(1-预约配送，2-1小时达，3-消费者预约送达)
    #[serde(default)]
    pub shipping_type: i32,
    /// 定金订单信息 ，非定金订单该字段为null
    #[serde(default)]
    pub step_order_info: Option<StepOrderInfo>,
    /// 缺货处理状态 -1:无缺货处理 0: 缺货待处理 1缺货已处理
    #[serde(default)]
    pub stock_out_handle_status: i32,
    /// 门店信息
    #[serde(default)]
    pub store_info: Option<StoreInfo>,
    /// 全国联保，1:是，0:否
    #[serde(default)]
    pub support_nationwide_warranty: i32,
    /// 收件地区县
    #[serde(default)]
    pub town: String,
    /// 区县编码
    #[serde(default)]
    pub town_id: i32,
    /// 快递运单号
    #[serde(default)]
    pub tracking_number: String,
    /// 订单类型 0-普通订单、1-定金订单
    #[serde(default)]
    pub trade_type: i32,
    /// 订单最近一次更新时间
    #[serde(default)]
    pub updated_at: String,
    /// 催发货时间
    #[serde(default)]
    pub urge_shipping_time: String,
    /// 预约配送日期
    #[serde(default)]
    pub yyps_date: String,
    /// 预约配送时段
    #[serde(default)]
    pub yyps_time: String,
}

#[derive(Debug, Deserialize)]
pub struct PddOrderInformationGetResponse {
    /// 订单详情对象
    #[serde(default)]
    pub order_info: Option<OrderInfo>,
}
