use std::error::Error;

/// 第三方平台消息通知
pub enum ThirdPartyMsg {
    // 企业微信
    Wework(WeworkMsg),
    // 钉钉
    Dingtalk(DingtalkMsg),
    // 飞书
    Feishu(FeishuMsg),
}

impl ThirdPartyMsg {
    pub fn new_wework_msg() -> Self {
        Self::Wework(WeworkMsg {})
    }
    pub fn new_dingtalk_msg() -> Self {
        Self::Dingtalk(DingtalkMsg {})
    }
    pub fn new_feishu_msg() -> Self {
        Self::Feishu(FeishuMsg {})
    }
}

/// 企业微信消息
/// https://developer.work.weixin.qq.com/document/path/90372
/// 具体里面本次就不实现了，这里是结构体还是枚举影响不大，可参考 https://isyin.cn/rust/2023-04-20-%E4%BC%81%E4%B8%9A%E5%BE%AE%E4%BF%A1%E6%B6%88%E6%81%AF%E9%80%9A%E7%9F%A5%E8%A6%81%E6%80%8E%E4%B9%88%E5%A3%B0%E6%98%8E-rust-%E7%BB%93%E6%9E%84%E4%BD%93/
pub struct WeworkMsg {}

impl WeworkMsg {
    pub fn new() -> Self {
        Self {}
    }
}

/// 钉钉消息
/// https://open.dingtalk.com/document/orgapp/message-types-and-data-format
/// 具体里面本次就不实现了，这里是结构体还是枚举影响不大，可参考 https://isyin.cn/rust/2023-04-19-%E9%92%89%E9%92%89%E6%B6%88%E6%81%AF%E9%80%9A%E7%9F%A5%E8%A6%81%E6%80%8E%E4%B9%88%E5%A3%B0%E6%98%8E-rust-%E7%BB%93%E6%9E%84%E4%BD%93/
pub struct DingtalkMsg {}

impl DingtalkMsg {
    pub fn new() -> Self {
        Self {}
    }
}

/// 飞书消息
/// https://open.feishu.cn/document/server-docs/im-v1/message-content-description/create_json
/// 具体里面本次就不实现了，这里是结构体还是枚举影响不大，可参考 https://isyin.cn/rust/2023-04-21-%E9%A3%9E%E4%B9%A6%E6%B6%88%E6%81%AF%E9%80%9A%E7%9F%A5%E8%A6%81%E6%80%8E%E4%B9%88%E5%A3%B0%E6%98%8E-rust-%E7%BB%93%E6%9E%84%E4%BD%93/
pub struct FeishuMsg {}

impl FeishuMsg {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait TheThirdPartyMsg {}

impl TheThirdPartyMsg for WeworkMsg {}

impl TheThirdPartyMsg for DingtalkMsg {}

impl TheThirdPartyMsg for FeishuMsg {}

/// 业务场景消息模版
pub enum Template {
    // 企业微信
    Wework(WeworkTpl),
    // 钉钉
    Dingtalk(DingtalkTpl),
    // 飞书
    Feishu(FeishuTpl),
}

impl Template {
    pub fn new_wework_tpl() -> Self {
        Self::Wework(WeworkTpl {})
    }
    pub fn new_dingtalk_tpl() -> Self {
        Self::Dingtalk(DingtalkTpl {})
    }
    pub fn new_feishu_tpl() -> Self {
        Self::Feishu(FeishuTpl {})
    }
}

/// 业务场景消息模版 - 企业微信渠道
pub struct WeworkTpl {}

impl WeworkTpl {
    pub fn new() -> Self {
        Self {}
    }
}

/// 业务场景消息模版 - 钉钉渠道
pub struct DingtalkTpl {}

impl DingtalkTpl {
    pub fn new() -> Self {
        Self {}
    }
}

/// 业务场景消息模版 - 飞书渠道
pub struct FeishuTpl {}


impl FeishuTpl {
    pub fn new() -> Self {
        Self {}
    }
}

/// Trait 方式实现
pub trait ToThirdPartyMsg2 {
    fn to_third_party_msg(&self) -> Result<Box<dyn TheThirdPartyMsg>, &dyn Error>;
}

impl ToThirdPartyMsg2 for Template {
    fn to_third_party_msg(&self) -> Result<Box<dyn TheThirdPartyMsg>, &dyn Error> {
        match self {
            Template::Wework(tpl) => tpl.to_third_party_msg(),
            Template::Dingtalk(tpl) => tpl.to_third_party_msg(),
            Template::Feishu(tpl) => tpl.to_third_party_msg(),
        }
    }
}

impl ToThirdPartyMsg2 for WeworkTpl {
    fn to_third_party_msg(&self) -> Result<Box<dyn TheThirdPartyMsg>, &dyn Error> {
        println!("WeworkTpl to_third_party_msg");
        Ok(Box::new(WeworkMsg::new()))
    }
}

impl ToThirdPartyMsg2 for DingtalkTpl {
    fn to_third_party_msg(&self) -> Result<Box<dyn TheThirdPartyMsg>, &dyn Error> {
        println!("DingtalkTpl to_third_party_msg");
        Ok(Box::new(DingtalkMsg::new()))
    }
}

impl ToThirdPartyMsg2 for FeishuTpl {
    fn to_third_party_msg(&self) -> Result<Box<dyn TheThirdPartyMsg>, &dyn Error> {
        println!("FeishuTpl to_third_party_msg");
        Ok(Box::new(FeishuMsg::new()))
    }
}