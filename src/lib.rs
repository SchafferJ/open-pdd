mod client;
mod common;
mod errors;

pub mod http;
mod utils;

#[cfg(test)]
mod tests {
	use tracing::{error, info};
	use tracing_subscriber::layer::SubscriberExt;
	use tracing_subscriber::util::SubscriberInitExt;
	use crate::client::ClientBuilder;
	use crate::errors::Error;
	use crate::http::model::ddk::pdd_ddk_cashgift_data_query::PddDdkCashgiftDataQueryRequest;
	use crate::http::model::tools::pdd_pop_auth_token_create::PddPopAuthTokenCreateRequest;
	use crate::http::response::ResponseResult;
	use crate::utils;

	#[tokio::test]
	async fn test_create_token() -> Result<(), Error> {
		tracing_subscriber::registry()
			.with(tracing_subscriber::EnvFilter::new(
				"open_pdd=debug,reqwest=debug",
			))
			.with(tracing_subscriber::fmt::layer())
			.init();

		let client = ClientBuilder::new("".to_string(), "".to_string())
			.http()
			.default();
		let req = PddPopAuthTokenCreateRequest {
			code: "".to_string(),
		};
		match client.post(req, None).await? {
			ResponseResult::Fail(err_rsp) => {
				error!("request fail!: {err_rsp:?}");
			}
			ResponseResult::Success(rsp) => {
				info!("request success!: {rsp:?}");
			}
		};
		Ok(())
	}

	#[tokio::test]
	async fn test() -> Result<(), Error> {
		tracing_subscriber::registry()
			.with(tracing_subscriber::EnvFilter::new(
				"open_pdd=debug,reqwest=debug",
			))
			.with(tracing_subscriber::fmt::layer())
			.init();
		let client = ClientBuilder::new("".to_string(), "".to_string())
			.http()
			.default();
		let req = PddDdkCashgiftDataQueryRequest {
			cash_gift_id: None,
			end_time: None,
			page: Some(1),
			page_size: Some(10),
			start_time: None,
		};

		match client.post(req, Some("".to_string())).await? {
			ResponseResult::Fail(err_rsp) => {
				error!("request fail!: {err_rsp:?}");
			}
			ResponseResult::Success(rsp) => {
				info!("request success!: {rsp:?}");
			}
		};
		Ok(())
	}
	
	#[test]
	fn test_sign(){
		let sign = utils::md5_str("1234567890access_token88888888client_idabcdefgdata_typeJSONpage1page_size10timestamp1723190100typepdd.ddk.cashgift.data.queryversionV11234567890");
		println!("{sign}");
		println!("{}", sign == "E14A6BABEE1AF71A4B3050BBA7680D18");
	}
}