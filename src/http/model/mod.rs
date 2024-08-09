// 多多推广API
#[cfg(feature = "ad")]
pub mod ad;

// 订单API
#[cfg(feature = "order")]
pub mod order;

// 售后API
#[cfg(feature = "refund")]
pub mod refund;

// 物流API
#[cfg(feature = "logistics")]
pub mod logistics;

// 虚拟类目API
#[cfg(feature = "virtual")]
pub mod r#virtual;

// 商品API
#[cfg(feature = "goods")]
pub mod goods;

// 多多客API
#[cfg(feature = "ddk")]
pub mod ddk;

// 多多客工具API
#[cfg(feature = "ddk_oauth")]
pub mod ddk_oauth;

// 方舟物流服务商API
#[cfg(feature = "logistics_cs")]
pub mod logistics_cs;

// 营销API
#[cfg(feature = "promotion")]
pub mod promotion;

// 卡券API
#[cfg(feature = "voucher")]
pub mod voucher;

// 发票服务API
#[cfg(feature = "invoice")]
pub mod invoice;

// 店铺API
#[cfg(feature = "mall")]
pub mod mall;

// 工具API
#[cfg(feature = "tools")]
pub mod tools;

// 仓储API
#[cfg(feature = "stock")]
pub mod stock;

// 消息服务API
#[cfg(feature = "pmc")]
pub mod pmc;

// 电子面单API
#[cfg(feature = "waybill")]
pub mod waybill;

// 财务API
#[cfg(feature = "finance")]
pub mod finance;

// 短信服务API
#[cfg(feature = "open_msg")]
pub mod open_msg;

// 服务市场API
#[cfg(feature = "service_market")]
pub mod service_market;

// 短信供应商API
#[cfg(feature = "sms")]
pub mod sms;

// 电子面单代打API
#[cfg(feature = "fds")]
pub mod fds;

// 门店API
#[cfg(feature = "mall_info")]
pub mod mall_info;

// 多多国际API
#[cfg(feature = "oversea")]
pub mod oversea;

// 方舟旅游门票API
#[cfg(feature = "ticket")]
pub mod ticket;

