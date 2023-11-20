use std::fmt::Debug;
use crate::requests::tasks_data::*;

pub(crate) trait TaskReqTrait: Clone + Debug {
    fn get_task_name(&self) -> &'static str;
}

impl TaskReqTrait for ImageToTextTaskReq {
    fn get_task_name(&self) -> &'static str {
        "ImageToTextTask"
    }
}

impl TaskReqTrait for NoCaptchaTaskProxylessReq {
    fn get_task_name(&self) -> &'static str {
        "NoCaptchaTaskProxyless"
    }
}

impl TaskReqTrait for NoCaptchaTaskReq {
    fn get_task_name(&self) -> &'static str {
        "NoCaptchaTask"
    }
}

impl TaskReqTrait for RecaptchaV3TaskProxylessReq {
    fn get_task_name(&self) -> &'static str {
        "RecaptchaV3TaskProxyless"
    }
}

impl TaskReqTrait for RecaptchaV2EnterpriseTaskReq {
    fn get_task_name(&self) -> &'static str {
        "RecaptchaV2EnterpriseTask"
    }
}

impl TaskReqTrait for RecaptchaV2EnterpriseTaskProxylessReq {
    fn get_task_name(&self) -> &'static str {
        "RecaptchaV2EnterpriseTaskProxyless"
    }
}

impl TaskReqTrait for FunCaptchaTaskReq {
    fn get_task_name(&self) -> &'static str {
        "FunCaptchaTask"
    }
}

impl TaskReqTrait for FunCaptchaTaskProxylessReq {
    fn get_task_name(&self) -> &'static str {
        "FunCaptchaTaskProxyless"
    }
}

impl TaskReqTrait for HCaptchaTaskReq {
    fn get_task_name(&self) -> &'static str {
        "HCaptchaTask"
    }
}

impl TaskReqTrait for HCaptchaTaskProxylessReq {
    fn get_task_name(&self) -> &'static str {
        "HCaptchaTaskProxyless"
    }
}

impl TaskReqTrait for GeeTestTaskReq {
    fn get_task_name(&self) -> &'static str {
        "GeeTestTask"
    }
}

impl TaskReqTrait for GeeTestTaskProxylessReq {
    fn get_task_name(&self) -> &'static str {
        "GeeTestTaskProxyless"
    }
}

impl TaskReqTrait for TurnstileTaskReq {
    fn get_task_name(&self) -> &'static str {
        "TurnstileTask"
    }
}

impl TaskReqTrait for TurnstileTaskProxylessReq {
    fn get_task_name(&self) -> &'static str {
        "TurnstileTaskProxyless"
    }
}
