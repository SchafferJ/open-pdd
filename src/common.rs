pub struct ApiPlatform {
    protocol: &'static str,
    main_domain: &'static str,
    backup_domain: &'static str,
    router: &'static str,
    main_url: &'static str,
    backup_url: &'static str,
}

macro_rules! api_platform {
    (
        $(
            $(#[$docs:meta])*
            $konst:ident($protocol:expr,$main_domain:expr,$backup_domain:expr,$router:expr);
        )+
    ) => {
        impl ApiPlatform {
        $(
            $(#[$docs])*
            pub const  $konst: ApiPlatform = ApiPlatform{
                protocol: $protocol,
                main_domain:$main_domain,
                backup_domain: $backup_domain,
                router: $router,
                main_url: concat!($protocol, $main_domain, $router),
                backup_url: concat!($protocol, $backup_domain, $router),
            };
        )+

            pub const fn get_main_url(&self) -> &str{
                self.main_url
            }

            pub const fn get_backup_url(&self)-> &str{
                self.backup_url
            }
        }
    }
}

api_platform! {
    /// 开放平台
    OPEN("https://", "gw-api.pinduoduo.com", "opengw-api.yangkeduo.com", "/api/router");
    /// 方舟
    ARK("https://", "ark-api.pinduoduo.com", "ark-api.yangkeduo.com", "/ark/router");
    /// 文件
    FILE("https://", "gw-upload.pinduoduo.com", "opengw-upload.yangkeduo.com", "/api/upload");
    /// 消息
    MESSAGE("wss://", "message-api.pinduoduo.com", "message-api.yangkeduo.com", "/message");
}