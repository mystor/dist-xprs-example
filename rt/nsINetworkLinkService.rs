//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkLinkService.idl
//


pub mod nsINetworkLinkService_consts {
    pub const LINK_TYPE_UNKNOWN: i64 = 0;
    pub const LINK_TYPE_ETHERNET: i64 = 1;
    pub const LINK_TYPE_USB: i64 = 2;
    pub const LINK_TYPE_WIFI: i64 = 3;
    pub const LINK_TYPE_WIMAX: i64 = 4;
    pub const LINK_TYPE_2G: i64 = 5;
    pub const LINK_TYPE_3G: i64 = 6;
    pub const LINK_TYPE_4G: i64 = 7;
}


#[repr(C)]
pub struct nsINetworkLinkService {
    vtable: *const nsINetworkLinkServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINetworkLinkService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x103e5293, 0x77b3, 0x4b70,
            [0xaf, 0x59, 0x6e, 0x9e, 0x4a, 0x1f, 0x99, 0x4a])
    }
}

unsafe impl RefCounted for nsINetworkLinkService {
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
pub trait nsINetworkLinkServiceCoerce {
    fn coerce_from(v: &nsINetworkLinkService) -> &Self;
}

impl nsINetworkLinkServiceCoerce for nsINetworkLinkService {
    #[inline]
    fn coerce_from(v: &nsINetworkLinkService) -> &Self {
        v
    }
}

impl nsINetworkLinkService {
    #[inline]
    pub fn coerce<T: nsINetworkLinkServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINetworkLinkService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINetworkLinkServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkLinkService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINetworkLinkServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isLinkUp; */
    pub get_isLinkUp: unsafe extern "C" fn (this: *const nsINetworkLinkService, aIsLinkUp: *mut bool) -> nsresult,

    /* readonly attribute boolean linkStatusKnown; */
    pub get_linkStatusKnown: unsafe extern "C" fn (this: *const nsINetworkLinkService, aLinkStatusKnown: *mut bool) -> nsresult,

    /* readonly attribute unsigned long linkType; */
    pub get_linkType: unsafe extern "C" fn (this: *const nsINetworkLinkService, aLinkType: *mut libc::uint32_t) -> nsresult,

}


impl nsINetworkLinkService {
    /* readonly attribute boolean isLinkUp; */
    #[inline]
    pub unsafe fn get_isLinkUp(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isLinkUp)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean linkStatusKnown; */
    #[inline]
    pub unsafe fn get_linkStatusKnown(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_linkStatusKnown)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long linkType; */
    #[inline]
    pub unsafe fn get_linkType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_linkType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


