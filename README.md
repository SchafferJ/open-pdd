# open-pdd
拼多多开放平台sdk的rust实现

### 接口参考
[拼多多开放平台 > 文档中心 > API文档](https://open.pinduoduo.com/application/document/api)

### 接口存在的问题

1.由于返回值官方文档并没有标明是否必填，所以使用了一些默认的规则，如果是普通类型（如bool、i32、i64、f32、f64、String、Vec、TreeMap）不使用Option，仅增加serde属性宏default；如果是对象类型才使用Option

2.官方文档对类型描述不够详细，有些Object类型没有属性描述,如pdd.ddk.oauth.cashgift.create接口中请求参数p_id_list类型为OBJECT[]
，但是对OBJECT的属性没有描述，只能从示例中推测出参数p_id_list的类型实际为STRING[];pdd.cloudprint.customares.get接口中的响应参数keys类型为OBJECT[],通过pop-sdk发现该类型无字段

