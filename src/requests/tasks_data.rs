#![allow(non_snake_case)]

use serde::Serialize;
use serde_with_macros::skip_serializing_none;
use std::fmt::Debug;

use crate::modules::Modules;
use crate::proxy_type::ProxyType;

// #[derive(Serialize)]
// pub(crate) enum TaskType {
//     ImageToTextTask(ImageToTextTaskReq),
//     NoCaptchaTaskProxyless(NoCaptchaTaskProxylessReq),
//     NoCaptchaTask(NoCaptchaTaskReq),
//     RecaptchaV3TaskProxyless(RecaptchaV3TaskProxylessReq),
//     RecaptchaV2EnterpriseTask(RecaptchaV2EnterpriseTaskReq),
//     RecaptchaV2EnterpriseTaskProxyless(RecaptchaV2EnterpriseTaskProxylessReq),
//     FunCaptchaTask(FunCaptchaTaskReq),
//     FunCaptchaTaskProxyless(FunCaptchaTaskProxylessReq),
//     HCaptchaTask(HCaptchaTaskReq),
//     HCaptchaTaskProxyless(HCaptchaTaskProxylessReq),
//     GeeTestTask(GeeTestTaskReq),
//     GeeTestTaskProxyless(GeeTestTaskProxylessReq),
// }

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct ImageToTextTaskReq {
    pub body: String,
    pub CapMonsterModule: Option<Modules>,
    pub recognizingThreshold: Option<u8>,
    pub Case: Option<bool>,
    pub numeric: Option<u8>,
    pub math: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct NoCaptchaTaskProxylessReq {
    pub websiteURL: String,
    pub websiteKey: String,
    pub recaptchaDataSValue: Option<String>,
    pub userAgent: Option<String>,
    pub cookies: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct NoCaptchaTaskReq {
    pub websiteURL: String,
    pub websiteKey: String,
    pub recaptchaDataSValue: Option<String>,
    pub proxyType: ProxyType,
    pub proxyAddress: String,
    pub proxyPort: u16,
    pub proxyLogin: Option<String>,
    pub proxyPassword: Option<String>,
    pub userAgent: Option<String>,
    pub cookies: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct RecaptchaV3TaskProxylessReq {
    pub websiteURL: String,
    pub websiteKey: String,
    pub minScore: Option<f32>,
    pub pageAction: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct RecaptchaV2EnterpriseTaskReq {
    pub websiteURL: String,
    pub websiteKey: String,
    pub enterprisePayload: Option<String>,
    pub apiDomain: Option<String>,
    pub proxyType: ProxyType,
    pub proxyAddress: String,
    pub proxyPort: u16,
    pub proxyLogin: Option<String>,
    pub proxyPassword: Option<String>,
    pub userAgent: Option<String>,
    pub cookies: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct RecaptchaV2EnterpriseTaskProxylessReq {
    pub websiteURL: String,
    pub websiteKey: String,
    pub enterprisePayload: Option<String>,
    pub apiDomain: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct FunCaptchaTaskReq {
    pub websiteURL: String,
    pub funcaptchaApiJSSubdomain: Option<String>,
    pub websitePublicKey: String,
    pub data: Option<String>,
    pub proxyType: ProxyType,
    pub proxyAddress: String,
    pub proxyPort: u16,
    pub proxyLogin: Option<String>,
    pub proxyPassword: Option<String>,
    pub userAgent: Option<String>,
    pub cookies: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct FunCaptchaTaskProxylessReq {
    pub websiteURL: String,
    pub funcaptchaApiJSSubdomain: Option<String>,
    pub websitePublicKey: String,
    pub data: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct HCaptchaTaskReq {
    pub websiteURL: String,
    pub websiteKey: String,
    pub isInvisible: Option<bool>,
    pub data: Option<String>,
    pub proxyType: ProxyType,
    pub proxyAddress: String,
    pub proxyPort: u16,
    pub proxyLogin: Option<String>,
    pub proxyPassword: Option<String>,
    pub userAgent: Option<String>,
    pub cookies: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct HCaptchaTaskProxylessReq {
    pub websiteURL: String,
    pub websiteKey: String,
    pub isInvisible: Option<bool>,
    pub data: Option<String>,
    pub userAgent: Option<String>,
    pub cookies: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct GeeTestTaskReq {
    pub websiteURL: String,
    pub gt: String,
    pub challenge: String,
    pub geetestApiServerSubdomain: String,
    pub geetestGetLib: String,
    pub proxyType: ProxyType,
    pub proxyAddress: String,
    pub proxyPort: u16,
    pub proxyLogin: Option<String>,
    pub proxyPassword: Option<String>,
    pub userAgent: Option<String>,
    pub cookies: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct GeeTestTaskProxylessReq {
    pub websiteURL: String,
    pub gt: String,
    pub challenge: String,
    pub geetestApiServerSubdomain: String,
    pub geetestGetLib: String,
    pub userAgent: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct TurnstileTaskReq {
    pub websiteURL: String,
    pub websiteKey: String,
    pub proxyType: ProxyType,
    pub proxyAddress: String,
    pub proxyPort: u16,
    pub proxyLogin: Option<String>,
    pub proxyPassword: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default, Clone, Debug)]
pub struct TurnstileTaskProxylessReq {
    pub websiteURL: String,
    pub websiteKey: String,
}