//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRequest.idl
//


pub type nsLoadFlags = libc::uint32_t;


pub mod nsIRequest_consts {
    pub const LOAD_REQUESTMASK: i64 = 65535;
    pub const LOAD_NORMAL: i64 = 0;
    pub const LOAD_BACKGROUND: i64 = 1;
    pub const LOAD_HTML_OBJECT_DATA: i64 = 2;
    pub const INHIBIT_CACHING: i64 = 128;
    pub const INHIBIT_PERSISTENT_CACHING: i64 = 256;
    pub const LOAD_BYPASS_CACHE: i64 = 512;
    pub const LOAD_FROM_CACHE: i64 = 1024;
    pub const VALIDATE_ALWAYS: i64 = 2048;
    pub const VALIDATE_NEVER: i64 = 4096;
    pub const VALIDATE_ONCE_PER_SESSION: i64 = 8192;
    pub const LOAD_ANONYMOUS: i64 = 16384;
    pub const LOAD_FRESH_CONNECTION: i64 = 32768;
}


#[repr(C)]
pub struct nsIRequest {
    vtable: *const nsIRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xef6bfbd2, 0xfd46, 0x48d8,
            [0x96, 0xb7, 0x9f, 0x8f, 0x0f, 0xd3, 0x87, 0xfe])
    }
}

unsafe impl RefCounted for nsIRequest {
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
pub trait nsIRequestCoerce {
    fn coerce_from(v: &nsIRequest) -> &Self;
}

impl nsIRequestCoerce for nsIRequest {
    #[inline]
    fn coerce_from(v: &nsIRequest) -> &Self {
        v
    }
}

impl nsIRequest {
    #[inline]
    pub fn coerce<T: nsIRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRequestVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIRequest, aName: *mut nsACString) -> nsresult,

    /* boolean isPending (); */
    pub isPending: unsafe extern "C" fn (this: *const nsIRequest, _retval: *mut bool) -> nsresult,

    /* readonly attribute nsresult status; */
    pub get_status: unsafe extern "C" fn (this: *const nsIRequest, aStatus: *mut nsresult) -> nsresult,

    /* void cancel (in nsresult aStatus); */
    pub cancel: unsafe extern "C" fn (this: *const nsIRequest, aStatus: nsresult) -> nsresult,

    /* void suspend (); */
    pub suspend: unsafe extern "C" fn (this: *const nsIRequest) -> nsresult,

    /* void resume (); */
    pub resume: unsafe extern "C" fn (this: *const nsIRequest) -> nsresult,

    /* attribute nsILoadGroup loadGroup; */
    pub get_loadGroup: unsafe extern "C" fn (this: *const nsIRequest, aLoadGroup: *mut *const nsILoadGroup) -> nsresult,
    pub set_loadGroup: unsafe extern "C" fn (this: *const nsIRequest, aLoadGroup: *const nsILoadGroup) -> nsresult,

    /* attribute nsLoadFlags loadFlags; */
    pub get_loadFlags: unsafe extern "C" fn (this: *const nsIRequest, aLoadFlags: *mut nsLoadFlags) -> nsresult,
    pub set_loadFlags: unsafe extern "C" fn (this: *const nsIRequest, aLoadFlags: nsLoadFlags) -> nsresult,

}


impl nsIRequest {
    /* readonly attribute AUTF8String name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isPending (); */
    #[inline]
    pub unsafe fn isPending(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isPending)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsresult status; */
    #[inline]
    pub unsafe fn get_status(&self, ) -> Result<nsresult, nsresult> {
        let mut _retval: nsresult = ::std::mem::zeroed();
        match ((*self.vtable).get_status)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void cancel (in nsresult aStatus); */
    #[inline]
    pub unsafe fn cancel(&self, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void suspend (); */
    #[inline]
    pub unsafe fn suspend(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).suspend)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resume (); */
    #[inline]
    pub unsafe fn resume(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resume)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsILoadGroup loadGroup; */
    #[inline]
    pub unsafe fn get_loadGroup(&self, ) -> Result<Option<RefPtr<nsILoadGroup>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadGroup)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_loadGroup(&self, aLoadGroup: Option<&nsILoadGroup>) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadGroup)(self as *const _, aLoadGroup.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsLoadFlags loadFlags; */
    #[inline]
    pub unsafe fn get_loadFlags(&self, ) -> Result<nsLoadFlags, nsresult> {
        let mut _retval: nsLoadFlags = ::std::mem::zeroed();
        match ((*self.vtable).get_loadFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_loadFlags(&self, aLoadFlags: nsLoadFlags) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadFlags)(self as *const _, aLoadFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


