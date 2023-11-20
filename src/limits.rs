use crate::requests::tasks_data::*;
use std::marker::PhantomData;
use std::time::Duration;
use crate::requests::tasks_names::TaskReqTrait;

pub(crate) const REQUEST_TIMEOUT: Duration = Duration::from_secs(21);
pub(crate) const HTTP2_KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(30);

pub(crate) const TASK_REQUEST_MAX: u8 = 120;

// request intervals in ms
const ITT_REQUEST_INTERVAL: Duration = Duration::from_millis(200); // for ImageToTextTask
const RC_REQUEST_INTERVAL: Duration = Duration::from_secs(3);
const FC_REQUEST_INTERVAL: Duration = Duration::from_secs(1);
const HC_REQUEST_INTERVAL: Duration = Duration::from_secs(3);
const GT_REQUEST_INTERVAL: Duration = Duration::from_secs(1);
const T_REQUEST_INTERVAL: Duration = Duration::from_secs(1);

// result timeouts
const ITT_RESULT_TIMEOUT: Duration = Duration::from_secs(10);
const RC_RESULT_TIMEOUT: Duration = Duration::from_secs(180);
const FC_RESULT_TIMEOUT: Duration = Duration::from_secs(80);
const HC_RESULT_TIMEOUT: Duration = Duration::from_secs(80);
const GT_RESULT_TIMEOUT: Duration = Duration::from_secs(80);
const T_RESULT_TIMEOUT: Duration = Duration::from_secs(80);

pub(crate) struct Limits<T: TaskReqTrait> {
    request_interval: Duration,
    result_timeout: Duration,
    #[allow(non_snake_case)]
    __: PhantomData<T>,
}

impl<T: TaskReqTrait> Limits<T> {
    pub(crate) fn request_interval(&self) -> Duration {
        self.request_interval
    }
    pub(crate) fn result_timeout(&self) -> Duration {
        self.result_timeout
    }
}

pub(crate) trait LimitsTrait {
    fn new() -> Self;
}

impl LimitsTrait for Limits<ImageToTextTaskReq> {
    fn new() -> Self {
        Self {
            request_interval: ITT_REQUEST_INTERVAL,
            result_timeout: ITT_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<NoCaptchaTaskProxylessReq> {
    fn new() -> Self {
        Self {
            request_interval: RC_REQUEST_INTERVAL,
            result_timeout: RC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<NoCaptchaTaskReq> {
    fn new() -> Self {
        Self {
            request_interval: RC_REQUEST_INTERVAL,
            result_timeout: RC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<RecaptchaV3TaskProxylessReq> {
    fn new() -> Self {
        Self {
            request_interval: RC_REQUEST_INTERVAL,
            result_timeout: RC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<RecaptchaV2EnterpriseTaskReq> {
    fn new() -> Self {
        Self {
            request_interval: RC_REQUEST_INTERVAL,
            result_timeout: RC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<RecaptchaV2EnterpriseTaskProxylessReq> {
    fn new() -> Self {
        Self {
            request_interval: RC_REQUEST_INTERVAL,
            result_timeout: RC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<FunCaptchaTaskReq> {
    fn new() -> Self {
        Self {
            request_interval: FC_REQUEST_INTERVAL,
            result_timeout: FC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<FunCaptchaTaskProxylessReq> {
    fn new() -> Self {
        Self {
            request_interval: FC_REQUEST_INTERVAL,
            result_timeout: FC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<HCaptchaTaskReq> {
    fn new() -> Self {
        Self {
            request_interval: HC_REQUEST_INTERVAL,
            result_timeout: HC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<HCaptchaTaskProxylessReq> {
    fn new() -> Self {
        Self {
            request_interval: HC_REQUEST_INTERVAL,
            result_timeout: HC_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<GeeTestTaskReq> {
    fn new() -> Self {
        Self {
            request_interval: GT_REQUEST_INTERVAL,
            result_timeout: GT_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<GeeTestTaskProxylessReq> {
    fn new() -> Self {
        Self {
            request_interval: GT_REQUEST_INTERVAL,
            result_timeout: GT_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<TurnstileTaskReq> {
    fn new() -> Self {
        Self {
            request_interval: T_REQUEST_INTERVAL,
            result_timeout: T_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}

impl LimitsTrait for Limits<TurnstileTaskProxylessReq> {
    fn new() -> Self {
        Self {
            request_interval: T_REQUEST_INTERVAL,
            result_timeout: T_RESULT_TIMEOUT,
            __: PhantomData,
        }
    }
}
