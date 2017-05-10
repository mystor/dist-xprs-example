//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecurityUITelemetry.idl
//


pub mod nsISecurityUITelemetry_consts {
    pub const WARNING_ADDON_ASKING_PREVENTED: i64 = 1;
    pub const WARNING_ADDON_ASKING_PREVENTED_CLICK_THROUGH: i64 = 2;
    pub const WARNING_CONFIRM_ADDON_INSTALL: i64 = 3;
    pub const WARNING_CONFIRM_ADDON_INSTALL_CLICK_THROUGH: i64 = 4;
    pub const WARNING_CONFIRM_POST_TO_INSECURE_FROM_SECURE: i64 = 9;
    pub const WARNING_CONFIRM_POST_TO_INSECURE_FROM_SECURE_CLICK_THROUGH: i64 = 10;
    pub const WARNING_BAD_CERT_ADD_EXCEPTION_BASE: i64 = 30;
    pub const WARNING_BAD_CERT_ADD_EXCEPTION_FLAG_UNTRUSTED: i64 = 1;
    pub const WARNING_BAD_CERT_ADD_EXCEPTION_FLAG_DOMAIN: i64 = 2;
    pub const WARNING_BAD_CERT_ADD_EXCEPTION_FLAG_TIME: i64 = 4;
    pub const WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_BASE: i64 = 38;
    pub const WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_FLAG_UNTRUSTED: i64 = 1;
    pub const WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_FLAG_DOMAIN: i64 = 2;
    pub const WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_FLAG_TIME: i64 = 4;
    pub const WARNING_GEOLOCATION_REQUEST: i64 = 46;
    pub const WARNING_GEOLOCATION_REQUEST_SHARE_LOCATION: i64 = 47;
    pub const WARNING_GEOLOCATION_REQUEST_ALWAYS_SHARE: i64 = 48;
    pub const WARNING_GEOLOCATION_REQUEST_NEVER_SHARE: i64 = 49;
    pub const WARNING_MALWARE_PAGE_TOP: i64 = 52;
    pub const WARNING_MALWARE_PAGE_TOP_WHY_BLOCKED: i64 = 53;
    pub const WARNING_MALWARE_PAGE_TOP_GET_ME_OUT_OF_HERE: i64 = 54;
    pub const WARNING_MALWARE_PAGE_TOP_IGNORE_WARNING: i64 = 55;
    pub const WARNING_PHISHING_PAGE_TOP: i64 = 56;
    pub const WARNING_PHISHING_PAGE_TOP_WHY_BLOCKED: i64 = 57;
    pub const WARNING_PHISHING_PAGE_TOP_GET_ME_OUT_OF_HERE: i64 = 58;
    pub const WARNING_PHISHING_PAGE_TOP_IGNORE_WARNING: i64 = 59;
    pub const WARNING_MALWARE_PAGE_FRAME: i64 = 60;
    pub const WARNING_MALWARE_PAGE_FRAME_WHY_BLOCKED: i64 = 61;
    pub const WARNING_MALWARE_PAGE_FRAME_GET_ME_OUT_OF_HERE: i64 = 62;
    pub const WARNING_MALWARE_PAGE_FRAME_IGNORE_WARNING: i64 = 63;
    pub const WARNING_PHISHING_PAGE_FRAME: i64 = 64;
    pub const WARNING_PHISHING_PAGE_FRAME_WHY_BLOCKED: i64 = 65;
    pub const WARNING_PHISHING_PAGE_FRAME_GET_ME_OUT_OF_HERE: i64 = 66;
    pub const WARNING_PHISHING_PAGE_FRAME_IGNORE_WARNING: i64 = 67;
    pub const WARNING_BAD_CERT_TOP: i64 = 68;
    pub const WARNING_BAD_CERT_TOP_STS: i64 = 69;
    pub const WARNING_BAD_CERT_TOP_CLICK_ADD_EXCEPTION: i64 = 70;
    pub const WARNING_BAD_CERT_TOP_CLICK_VIEW_CERT: i64 = 71;
    pub const WARNING_BAD_CERT_TOP_DONT_REMEMBER_EXCEPTION: i64 = 72;
    pub const WARNING_BAD_CERT_TOP_GET_ME_OUT_OF_HERE: i64 = 73;
    pub const WARNING_BAD_CERT_TOP_UNDERSTAND_RISKS: i64 = 74;
    pub const WARNING_BAD_CERT_TOP_ADD_EXCEPTION_BASE: i64 = 76;
    pub const WARNING_BAD_CERT_TOP_ADD_EXCEPTION_FLAG_UNTRUSTED: i64 = 1;
    pub const WARNING_BAD_CERT_TOP_ADD_EXCEPTION_FLAG_DOMAIN: i64 = 2;
    pub const WARNING_BAD_CERT_TOP_ADD_EXCEPTION_FLAG_TIME: i64 = 4;
    pub const WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_BASE: i64 = 84;
    pub const WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_FLAG_UNTRUSTED: i64 = 1;
    pub const WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_FLAG_DOMAIN: i64 = 2;
    pub const WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_FLAG_TIME: i64 = 4;
    pub const WARNING_UNWANTED_PAGE_TOP: i64 = 92;
    pub const WARNING_UNWANTED_PAGE_TOP_WHY_BLOCKED: i64 = 93;
    pub const WARNING_UNWANTED_PAGE_TOP_GET_ME_OUT_OF_HERE: i64 = 94;
    pub const WARNING_UNWANTED_PAGE_TOP_IGNORE_WARNING: i64 = 95;
    pub const WARNING_UNWANTED_PAGE_FRAME: i64 = 96;
    pub const WARNING_UNWANTED_PAGE_FRAME_WHY_BLOCKED: i64 = 97;
    pub const WARNING_UNWANTED_PAGE_FRAME_GET_ME_OUT_OF_HERE: i64 = 98;
    pub const WARNING_UNWANTED_PAGE_FRAME_IGNORE_WARNING: i64 = 99;
}


#[repr(C)]
pub struct nsISecurityUITelemetry {
    vtable: *const nsISecurityUITelemetryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISecurityUITelemetry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5d1acf82, 0x223a, 0x46fb,
            [0xa8, 0xf3, 0xa1, 0xb1, 0x6e, 0x2c, 0xeb, 0x04])
    }
}

unsafe impl RefCounted for nsISecurityUITelemetry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait nsISecurityUITelemetryCoerce {
    fn coerce_from(v: &nsISecurityUITelemetry) -> &Self;
}

impl nsISecurityUITelemetryCoerce for nsISecurityUITelemetry {
    #[inline]
    fn coerce_from(v: &nsISecurityUITelemetry) -> &Self {
        v
    }
}

impl nsISecurityUITelemetry {
    #[inline]
    pub fn coerce<T: nsISecurityUITelemetryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISecurityUITelemetry {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISecurityUITelemetryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecurityUITelemetry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISecurityUITelemetryVTable {
    pub __base: nsISupportsVTable,

}


impl nsISecurityUITelemetry {
}


