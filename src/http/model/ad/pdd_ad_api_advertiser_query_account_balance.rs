//! 查询广告主账户余额

use crate::common::ApiPlatform;
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddAdApiAdvertiserQueryAccountBalanceRequest;

impl RequestType for PddAdApiAdvertiserQueryAccountBalanceRequest {
    type Response = PddAdApiAdvertiserQueryAccountBalanceResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ad.api.advertiser.query.account.balance"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::OPEN
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountsBalance {
    /// 账户类型。1表示现金余额，2表示通用红包余额，3表示奖励金余额，7表示聚焦展位CPM专用红包余额，10表示直播专用红包余额。
    #[serde(default, rename = "accountType")]
    pub account_type: i32,
    /// 账户余额，单位厘
    #[serde(default, rename = "rawBalance")]
    pub raw_balance: i64,
    /// 可消费余额，单位厘
    #[serde(default, rename = "spendableBalance")]
    pub spendable_balance: i64,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    /// 账户余额明细列表
    #[serde(default, rename = "accountsBalance")]
    pub accounts_balance: Option<Vec<AccountsBalance>>,
    /// 总余额，单位厘
    #[serde(default)]
    pub balance: i64,
    /// 广告主Id
    #[serde(default, rename = "mallId")]
    pub mall_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PddAdApiAdvertiserQueryAccountBalanceResponse {
    #[serde(default, rename = "errorCode")]
    pub error_code: i32,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: String,
    #[serde(default)]
    pub result: Option<Result>,
    #[serde(default)]
    pub success: bool,
}
