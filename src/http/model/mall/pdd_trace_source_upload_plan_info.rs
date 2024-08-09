//! # 溯源服务商上传正品溯源粘贴计划
//!
//! 溯源服务商上传正品溯源粘贴计划, 用于正品溯源功能
use crate::http::request::RequestType;
use serde::{Deserialize, Serialize};
use crate::common::ApiPlatform;

#[derive(Debug, Serialize)]
pub struct Goods {
    /// 防伪溯源码粘贴数量
    pub code_amount: i64,
    /// 防伪溯源码结束顺序号
    pub end_serial_no: String,
    /// 商品ID
    pub goods_id: i64,
    /// 商品备案图片
    pub goods_image_url: Option<String>,
    /// 商品备案名称
    pub goods_name: String,
    /// 原产国(地)
    pub goods_origin: String,
    /// 商品备案规格型号
    pub goods_property: String,
    /// 商品规格
    pub goods_sku_no: String,
    /// Hs编码
    pub hs_code: String,
    /// Hs名称
    pub hs_name: String,
    /// 防伪溯源码起始顺序号
    pub start_serial_no: String,
}

#[derive(Debug, Serialize)]
pub struct PddTraceSourceUploadPlanInfoRequest {
    /// 到港日期
    pub arrive_time: Option<String>,
    /// 提单号
    pub bill_no: Option<String>,
    /// 报检日期
    pub ciq_date: Option<String>,
    /// 报检单号
    pub ciq_no: Option<String>,
    /// 境内收发货人
    pub dealer_org: Option<String>,
    /// 申报单位
    pub declare_org: String,
    /// 启运地
    pub desp_port_name: String,
    /// 报关日期
    pub entry_date: String,
    /// 报关单号
    pub entry_no: String,
    /// 溯源码粘贴计划(商品维度)
    pub goods: Vec<Goods>,
    /// 清单申报日期
    pub list_date: Option<String>,
    /// 核注清单编号
    pub list_no: Option<String>,
    /// 装货港
    pub load_port: String,
    /// 粘贴计划所属店铺ID
    pub mall_id: i64,
    /// 粘贴计划所属店铺名
    pub mall_name: String,
    /// 粘贴计划单激活时间
    pub plan_active_time: String,
    /// 粘贴计划单创建时间
    pub plan_created_time: String,
    /// 粘贴计划单编号
    pub plan_no: String,
    /// 进口口岸
    pub port: String,
    /// 运输方式
    pub transport_mode: String,
    /// 粘贴计划单所属保税仓名称
    pub warehouse_name: String,
}

impl RequestType for PddTraceSourceUploadPlanInfoRequest {
    type Response = PddTraceSourceUploadPlanInfoResponse;

    fn get_type(&self) -> &'static str {
        "pdd.trace.source.upload.plan.info"
    }

    fn get_platform(&self) -> &ApiPlatform {
        &ApiPlatform::ARK
    }
}

#[derive(Debug, Deserialize)]
pub struct PddTraceSourceUploadPlanInfoResponse {
    #[serde(default)]
    pub status: i32,
}
