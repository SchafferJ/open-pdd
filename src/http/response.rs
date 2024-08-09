use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponseWrapper {
    pub error_response: ErrorResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// 请求ID
    pub request_id: String,

    /// 错误代码
    pub error_code: i64,

    /// 错误参数
    pub error_msg: String,

    pub sub_code: Option<String>,

    pub sub_msg: Option<String>,
}

/// 枚举变体命名与标准库Result的变体区分开
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseResultWrapper<T> {
    Fail(ErrorResponseWrapper),
    Success(T),
}

impl<T> ResponseResultWrapper<T> {
    #[inline]
    pub const fn is_success(&self) -> bool {
        matches!(*self, ResponseResultWrapper::Success(_))
    }

    #[inline]
    pub const fn is_fail(&self) -> bool {
        !self.is_success()
    }

    pub fn unwrap(self) -> ResponseResult<T> {
        match self {
            ResponseResultWrapper::Fail(wrapper) => ResponseResult::Fail(wrapper.error_response),
            ResponseResultWrapper::Success(data) => ResponseResult::Success(data),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseResult<T> {
    Fail(ErrorResponse),
    Success(T),
}

#[derive(Debug, Serialize, Deserialize)]
struct Resp {
    id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let value = serde_json::json!({
          "error_response": {
            "error_msg": "公共参数错误:type",
            "sub_msg": "",
            "sub_code": null,
            "error_code": 10001,
            "request_id": "15440104776643887"
          }
        });
        // let response: ErrorResponseWrapper = serde_json::from_value(value).unwrap();
        // println!("{response:?}");

        let response: ResponseResultWrapper<Resp> = serde_json::from_value(value).unwrap();
        match response {
            ResponseResultWrapper::<_>::Fail(w) => {
                println!("{w:?}")
            }
            ResponseResultWrapper::Success(r) => {
                println!("{r:?}")
            }
        }
    }
}
