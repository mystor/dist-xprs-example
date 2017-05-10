//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentPolicyBase.idl
//


pub type nsContentPolicyType = libc::uint32_t;


pub mod nsIContentPolicyBase_consts {
    pub const TYPE_INVALID: i64 = 0;
    pub const TYPE_OTHER: i64 = 1;
    pub const TYPE_SCRIPT: i64 = 2;
    pub const TYPE_IMAGE: i64 = 3;
    pub const TYPE_STYLESHEET: i64 = 4;
    pub const TYPE_OBJECT: i64 = 5;
    pub const TYPE_DOCUMENT: i64 = 6;
    pub const TYPE_SUBDOCUMENT: i64 = 7;
    pub const TYPE_REFRESH: i64 = 8;
    pub const TYPE_XBL: i64 = 9;
    pub const TYPE_PING: i64 = 10;
    pub const TYPE_XMLHTTPREQUEST: i64 = 11;
    pub const TYPE_DATAREQUEST: i64 = 11;
    pub const TYPE_OBJECT_SUBREQUEST: i64 = 12;
    pub const TYPE_DTD: i64 = 13;
    pub const TYPE_FONT: i64 = 14;
    pub const TYPE_MEDIA: i64 = 15;
    pub const TYPE_WEBSOCKET: i64 = 16;
    pub const TYPE_CSP_REPORT: i64 = 17;
    pub const TYPE_XSLT: i64 = 18;
    pub const TYPE_BEACON: i64 = 19;
    pub const TYPE_FETCH: i64 = 20;
    pub const TYPE_IMAGESET: i64 = 21;
    pub const TYPE_WEB_MANIFEST: i64 = 22;
    pub const TYPE_INTERNAL_SCRIPT: i64 = 23;
    pub const TYPE_INTERNAL_WORKER: i64 = 24;
    pub const TYPE_INTERNAL_SHARED_WORKER: i64 = 25;
    pub const TYPE_INTERNAL_EMBED: i64 = 26;
    pub const TYPE_INTERNAL_OBJECT: i64 = 27;
    pub const TYPE_INTERNAL_FRAME: i64 = 28;
    pub const TYPE_INTERNAL_IFRAME: i64 = 29;
    pub const TYPE_INTERNAL_AUDIO: i64 = 30;
    pub const TYPE_INTERNAL_VIDEO: i64 = 31;
    pub const TYPE_INTERNAL_TRACK: i64 = 32;
    pub const TYPE_INTERNAL_XMLHTTPREQUEST: i64 = 33;
    pub const TYPE_INTERNAL_EVENTSOURCE: i64 = 34;
    pub const TYPE_INTERNAL_SERVICE_WORKER: i64 = 35;
    pub const TYPE_INTERNAL_SCRIPT_PRELOAD: i64 = 36;
    pub const TYPE_INTERNAL_IMAGE: i64 = 37;
    pub const TYPE_INTERNAL_IMAGE_PRELOAD: i64 = 38;
    pub const TYPE_INTERNAL_STYLESHEET: i64 = 39;
    pub const TYPE_INTERNAL_STYLESHEET_PRELOAD: i64 = 40;
    pub const TYPE_INTERNAL_IMAGE_FAVICON: i64 = 41;
    pub const TYPE_INTERNAL_WORKER_IMPORT_SCRIPTS: i64 = 42;
    pub const REJECT_REQUEST: i64 = -1;
    pub const REJECT_TYPE: i64 = -2;
    pub const REJECT_SERVER: i64 = -3;
    pub const REJECT_OTHER: i64 = -4;
    pub const ACCEPT: i64 = 1;
}


#[repr(C)]
pub struct nsIContentPolicyBase {
    vtable: *const nsIContentPolicyBaseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPolicyBase {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x17418187, 0xd86f, 0x48dd,
            [0x92, 0xd1, 0x23, 0x88, 0x38, 0xdf, 0x0a, 0x4e])
    }
}

unsafe impl RefCounted for nsIContentPolicyBase {
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
pub trait nsIContentPolicyBaseCoerce {
    fn coerce_from(v: &nsIContentPolicyBase) -> &Self;
}

impl nsIContentPolicyBaseCoerce for nsIContentPolicyBase {
    #[inline]
    fn coerce_from(v: &nsIContentPolicyBase) -> &Self {
        v
    }
}

impl nsIContentPolicyBase {
    #[inline]
    pub fn coerce<T: nsIContentPolicyBaseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPolicyBase {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentPolicyBaseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPolicyBase) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPolicyBaseVTable {
    pub __base: nsISupportsVTable,

}


impl nsIContentPolicyBase {
}


