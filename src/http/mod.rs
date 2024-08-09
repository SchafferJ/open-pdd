use reqwest::multipart::Form;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::debug;
use crate::errors::Error;
use response::ResponseResultWrapper;

use crate::http::request::{FileItem, RequestType};
use crate::http::response::ResponseResult;
use crate::utils;

pub mod model;
pub mod request;
pub mod response;

pub struct HttpClient {
	client_id: String,
	client_secret: String,
	client: Client,
}

impl HttpClient {
	pub fn new(client_id: String, client_secret: String, client: Client) -> HttpClient {
		HttpClient {
			client_id,
			client_secret,
			client,
		}
	}

	pub async fn post<Request>(
		&self,
		request: Request,
		access_token: Option<String>,
	) -> Result<ResponseResult<Request::Response>, Error>
	where
		Request: RequestType + Serialize,
		Request::Response: for<'a> Deserialize<'a>,
	{
		let response = self.post_raw(request, access_token).await?;
		let result: Result<ResponseResultWrapper<Request::Response>, Error> =
			response.json().await.map_err(Error::from);
		result.map(|wrapper| wrapper.unwrap())
	}

	pub async fn post_raw<Request>(
		&self,
		request: Request,
		access_token: Option<String>,
	) -> Result<Response, Error>
	where
		Request: RequestType + Serialize,
	{
		let url = request.get_platform().get_main_url();
		let params = request.get_params();
		let multipart = request.multipart_file();
		self._post(access_token, url, params, multipart).await
	}


	async fn _post(&self,
	               access_token: Option<String>,
	               url: &str, 
	               mut params: BTreeMap<String, String>,
	               multipart: Option<FileItem>,
	) -> Result<Response, Error>
	{
		self.assemble_request_params(&mut params, access_token);
		let response = match multipart {
			None => self.client.post(url).form(&params).send().await?,
			Some(file_item) => {
				let (file_name, part) = file_item.to_part().await?;
				let form = params
					.into_iter()
					.fold(Form::new(), |form, (k, v)| form.text(k, v))
					.part(file_name, part);
				self.client.post(url).multipart(form).send().await?
			}
		};
		Ok(response)
	}

	/// 组装请求参数
	fn assemble_request_params(
		&self,
		params: &mut BTreeMap<String, String>,
		access_token: Option<String>,
	) {
		params.insert("client_id".to_owned(), self.client_id.clone());
		if access_token.is_some() {
			params.insert("access_token".to_owned(), access_token.unwrap());
		}

		let sign_src = build_sign_source(&params, &self.client_secret);
		let sign = utils::md5_str(&sign_src);
		debug!("SIGN_SRC:[{sign_src}],SIGN:[{sign}]");

		params.insert("sign".to_owned(), sign);
	}
}

fn build_sign_source(map: &BTreeMap<String, String>, secret: &str) -> String {
	let mut sign_src = String::from(secret);
	map.iter()
		.for_each(|(k, v)| {
			sign_src.push_str(k);
			sign_src.push_str(v);
		});
	sign_src.push_str(secret);
	sign_src
}
