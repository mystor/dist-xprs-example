//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoadContextInfo.idl
//


pub mod nsILoadContextInfo_consts {
    pub const NO_APP_ID: i64 = 0;
    pub const UNKNOWN_APP_ID: i64 = 4294967295;
}


#[repr(C)]
pub struct nsILoadContextInfo {
    vtable: *const nsILoadContextInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoadContextInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x555e2f8a, 0xa1f6, 0x41dd,
            [0x88, 0xca, 0xed, 0x4e, 0xd6, 0xb9, 0x8a, 0x22])
    }
}

unsafe impl RefCounted for nsILoadContextInfo {
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
pub trait nsILoadContextInfoCoerce {
    fn coerce_from(v: &nsILoadContextInfo) -> &Self;
}

impl nsILoadContextInfoCoerce for nsILoadContextInfo {
    #[inline]
    fn coerce_from(v: &nsILoadContextInfo) -> &Self {
        v
    }
}

impl nsILoadContextInfo {
    #[inline]
    pub fn coerce<T: nsILoadContextInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoadContextInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoadContextInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadContextInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoadContextInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isPrivate; */
    pub get_isPrivate: unsafe extern "C" fn (this: *const nsILoadContextInfo, aIsPrivate: *mut bool) -> nsresult,

    /* readonly attribute boolean isAnonymous; */
    pub get_isAnonymous: unsafe extern "C" fn (this: *const nsILoadContextInfo, aIsAnonymous: *mut bool) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_originAttributes: *const ::libc::c_void,

    /* [binaryname(OriginAttributesPtr),noscript,nostdcall,notxpcom] OriginAttributesNativePtr binaryOriginAttributesPtr (); */
    /// Unable to call function as its signature contains a non-rust type
    pub OriginAttributesPtr: *const ::libc::c_void,

}


impl nsILoadContextInfo {
    /* readonly attribute boolean isPrivate; */
    #[inline]
    pub unsafe fn get_isPrivate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isPrivate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isAnonymous; */
    #[inline]
    pub unsafe fn get_isAnonymous(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isAnonymous)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */


    /* [binaryname(OriginAttributesPtr),noscript,nostdcall,notxpcom] OriginAttributesNativePtr binaryOriginAttributesPtr (); */


}


#[repr(C)]
pub struct nsILoadContextInfoFactory {
    vtable: *const nsILoadContextInfoFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoadContextInfoFactory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc1c7023d, 0x4318, 0x4f99,
            [0x83, 0x07, 0xb5, 0xcc, 0xf0, 0x55, 0x87, 0x93])
    }
}

unsafe impl RefCounted for nsILoadContextInfoFactory {
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
pub trait nsILoadContextInfoFactoryCoerce {
    fn coerce_from(v: &nsILoadContextInfoFactory) -> &Self;
}

impl nsILoadContextInfoFactoryCoerce for nsILoadContextInfoFactory {
    #[inline]
    fn coerce_from(v: &nsILoadContextInfoFactory) -> &Self {
        v
    }
}

impl nsILoadContextInfoFactory {
    #[inline]
    pub fn coerce<T: nsILoadContextInfoFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoadContextInfoFactory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoadContextInfoFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadContextInfoFactory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoadContextInfoFactoryVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsILoadContextInfo default; */
    pub get_default: unsafe extern "C" fn (this: *const nsILoadContextInfoFactory, aDefault: *mut *const nsILoadContextInfo) -> nsresult,

    /* readonly attribute nsILoadContextInfo private; */
    pub get_private: unsafe extern "C" fn (this: *const nsILoadContextInfoFactory, aPrivate: *mut *const nsILoadContextInfo) -> nsresult,

    /* readonly attribute nsILoadContextInfo anonymous; */
    pub get_anonymous: unsafe extern "C" fn (this: *const nsILoadContextInfoFactory, aAnonymous: *mut *const nsILoadContextInfo) -> nsresult,

    /* [implicit_jscontext] nsILoadContextInfo custom (in boolean aAnonymous, in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub custom: *const ::libc::c_void,

    /* nsILoadContextInfo fromLoadContext (in nsILoadContext aLoadContext, in boolean aAnonymous); */
    pub fromLoadContext: unsafe extern "C" fn (this: *const nsILoadContextInfoFactory, aLoadContext: *const nsILoadContext, aAnonymous: bool, _retval: *mut *const nsILoadContextInfo) -> nsresult,

    /* nsILoadContextInfo fromWindow (in nsIDOMWindow aWindow, in boolean aAnonymous); */
    pub fromWindow: unsafe extern "C" fn (this: *const nsILoadContextInfoFactory, aWindow: *const nsIDOMWindow, aAnonymous: bool, _retval: *mut *const nsILoadContextInfo) -> nsresult,

}


impl nsILoadContextInfoFactory {
    /* readonly attribute nsILoadContextInfo default; */
    #[inline]
    pub unsafe fn get_default(&self, ) -> Result<Option<RefPtr<nsILoadContextInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_default)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsILoadContextInfo private; */
    #[inline]
    pub unsafe fn get_private(&self, ) -> Result<Option<RefPtr<nsILoadContextInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_private)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsILoadContextInfo anonymous; */
    #[inline]
    pub unsafe fn get_anonymous(&self, ) -> Result<Option<RefPtr<nsILoadContextInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_anonymous)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] nsILoadContextInfo custom (in boolean aAnonymous, in jsval aOriginAttributes); */


    /* nsILoadContextInfo fromLoadContext (in nsILoadContext aLoadContext, in boolean aAnonymous); */
    #[inline]
    pub unsafe fn fromLoadContext(&self, aLoadContext: Option<&nsILoadContext>, aAnonymous: bool) -> Result<Option<RefPtr<nsILoadContextInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).fromLoadContext)(self as *const _, aLoadContext.map_or(::std::ptr::null(), |x| x as *const _), aAnonymous, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsILoadContextInfo fromWindow (in nsIDOMWindow aWindow, in boolean aAnonymous); */
    #[inline]
    pub unsafe fn fromWindow(&self, aWindow: Option<&nsIDOMWindow>, aAnonymous: bool) -> Result<Option<RefPtr<nsILoadContextInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).fromWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aAnonymous, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


