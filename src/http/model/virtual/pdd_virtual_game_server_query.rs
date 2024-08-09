//! # 虚拟游戏类区服列表接口
//!
//! 虚拟游戏类区服列表
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PddVirtualGameServerQueryRequest {
    /// 游戏CODE
    pub goods_config_code: String,
}

impl RequestType for PddVirtualGameServerQueryRequest {
    type Response = PddVirtualGameServerQueryResponse;

    fn get_type(&self) -> &'static str {
        "pdd.virtual.game.server.query"
    }
}

#[derive(Debug, Deserialize)]
pub struct Children {
    /// 区服ID
    #[serde(default)]
    pub id: i64,
    /// 名称
    #[serde(default)]
    pub name: String,
    /// 类型 1-区 2-服
    #[serde(default)]
    pub r#type: i32,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    /// 下级区服信息
    #[serde(default)]
    pub children: Option<Vec<Children>>,
    /// 区服ID
    #[serde(default)]
    pub id: i64,
    /// 名称
    #[serde(default)]
    pub name: String,
    /// 类型 1-区 2-服
    #[serde(default)]
    pub r#type: i32,
}

#[derive(Debug, Deserialize)]
pub struct PddVirtualGameServerQueryResponse {
    /// 游戏CODE
    #[serde(default)]
    pub goods_config_code: String,
    /// 游戏ID
    #[serde(default)]
    pub goods_config_id: i64,
    /// 游戏配置名称
    #[serde(default)]
    pub goods_config_name: String,
    /// 区服信息
    #[serde(default)]
    pub item_list: Option<Vec<Item>>,
}
