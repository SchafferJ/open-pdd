//! # 多多进宝数据统计查询接口
//!
//! 本接口用于查询特定活动数据，仅限特定渠道可用。
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddDdkStatisticsDataQueryRequest {
    /// 分页数，默认值: 1
    pub page: Option<i32>,
    /// 每页结果数，默认值: 20
    pub page_size: Option<i32>,
    /// 周期类型: 1-每7天，2-自然月
    pub period_type: i32,
    /// 数据类型: 1-增量补贴数据
    pub statistics_type: i32,
    /// 查询时间点，格式: "yyyy-MM-dd"。period_type为1时，查询时间点前7天的数据；period_type为2时，查询时间点所在自然月的数据。
    pub time: String,
}

impl RequestType for PddDdkStatisticsDataQueryRequest {
    type Response = PddDdkStatisticsDataQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.ddk.statistics.data.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct Data {
    /// 自定义参数，为链接打上自定义标签；自定义参数最长限制64个字节；格式为： {"uid":"11111","sid":"22222"} ，其中 uid 用户唯一标识，可自行加密后传入，每个用户仅且对应一个标识，必填； sid 上下文信息标识，例如sessionId等，非必填。该json字符串中也可以加入其他自定义的key。（如果使用GET请求，请使用URLEncode处理参数）
    #[serde(default)]
    pub custom_parameters: String,
    /// 结束时间，格式: "yyyy-MM-dd"
    #[serde(default)]
    pub end_time: String,
    /// GMV,单位为分
    #[serde(default)]
    pub order_amount: i64,
    /// 订单数
    #[serde(default)]
    pub order_num: i64,
    /// 推广位ID
    #[serde(default)]
    pub p_id: String,
    /// 开始时间，格式: "yyyy-MM-dd"
    #[serde(default)]
    pub start_time: String,
}

#[derive(Debug, Deserialize)]
pub struct PddDdkStatisticsDataQueryResponse {
    #[serde(default)]
    pub data_list: Option<Vec<Data>>,
}
