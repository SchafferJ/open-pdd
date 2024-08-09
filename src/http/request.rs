use std::borrow::Cow;
use std::collections::BTreeMap;
use std::mem;
use std::path::Path;

use reqwest::multipart::Part;
use serde::Serialize;
use serde_json::Value;
use tokio::fs;

use crate::common::ApiPlatform;
use crate::errors::Error;
use crate::utils;

pub trait RequestType {
	type Response;

	fn get_type(&self) -> &'static str;

	fn is_multipart(&self) -> bool {
		self.multipart_file().is_some()
	}

	fn get_data_type(&self) -> &'static str {
		"JSON"
	}

	fn get_version(&self) -> &'static str {
		"V1"
	}

	fn get_platform(&self) -> &ApiPlatform {
		&ApiPlatform::OPEN
	}

	fn multipart_file(&self) -> Option<FileItem> {
		None
	}

	fn get_params(&self) -> BTreeMap<String, String>
	where
		Self: Sized,
		Self: Serialize,
	{
		let mut params = BTreeMap::new();
		params.insert("version".to_string(), self.get_version().to_string());
		params.insert("data_type".to_string(), self.get_data_type().to_string());
		params.insert("type".to_string(), self.get_type().to_string());
		params.insert("timestamp".to_owned(), utils::now_as_secs().to_string());

		// 跳过无字段结构体解析，如struct Foo;
		if mem::size_of::<Self>() > 0 {
			let json_str = serde_json::to_string(self).unwrap();
			let map: BTreeMap<String, Value> = serde_json::from_str(&json_str).unwrap();
			map.into_iter()
				.filter(|(_, v)| !v.is_null())
				.map(|(k, v)| (k, match v {
					Value::String(s) => s,
					v => v.to_string(),
				}))
				.for_each(|(k, v)| _ = params.insert(k, v));
		}

		params
	}
}

#[derive(Debug, Clone)]
pub struct FileItem {
	// multipart表单字段名
	pub file_name: String,
	// 文件路径
	pub file_path: String,
}

impl FileItem {
	pub async fn to_part(self) -> Result<(String, Part), Error> {
		let FileItem {
			file_name,
			file_path,
		} = self;
		let bytes = fs::read(file_path.clone()).await?;
		let part = Part::bytes(Cow::from(bytes));
		if let Some(actual_file_name) = Path::new(&file_path)
			.file_name()
			.map(|os| os.to_string_lossy().to_string())
		{
			return Ok((file_name, part.file_name(actual_file_name)));
		}
		Ok((file_name, part))
	}
}
