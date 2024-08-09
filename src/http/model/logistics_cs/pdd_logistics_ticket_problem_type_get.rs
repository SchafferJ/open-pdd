//! # 快递公司工单问题类型列表接口
//!
//! 快递公司通过此接口同步多多所有物流工单问题类型
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddLogisticsTicketProblemTypeGetRequest;

impl RequestType for PddLogisticsTicketProblemTypeGetRequest {
    type Response = PddLogisticsTicketProblemTypeGetResponse;

    fn get_type(&self) -> &'static str {
        "pdd.logistics.ticket.problem.type.get"
    }
}

#[derive(Debug, Deserialize)]
pub struct LogisticsProblemType {
    /// 问题类型id
    #[serde(default)]
    pub id: i64,
    /// 问题类型描述
    #[serde(default)]
    pub type_desc: String,
}

#[derive(Debug, Deserialize)]
pub struct PddLogisticsTicketProblemTypeGetResponse {
    /// 问题类型list
    #[serde(default)]
    pub logistics_problem_type_list: Option<Vec<LogisticsProblemType>>,
}
