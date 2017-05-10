//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICookieService.idl
//


pub mod nsICookieService_consts {
    pub const BEHAVIOR_ACCEPT: i64 = 0;
    pub const BEHAVIOR_REJECT_FOREIGN: i64 = 1;
    pub const BEHAVIOR_REJECT: i64 = 2;
    pub const BEHAVIOR_LIMIT_FOREIGN: i64 = 3;
    pub const ACCEPT_NORMALLY: i64 = 0;
    pub const ACCEPT_SESSION: i64 = 2;
    pub const ACCEPT_FOR_N_DAYS: i64 = 3;
}


#[repr(C)]
pub struct nsICookieService {
    vtable: *const nsICookieServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICookieService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1e94e283, 0x2811, 0x4f43,
            [0xb9, 0x47, 0xd2, 0x2b, 0x15, 0x49, 0xd8, 0x24])
    }
}

unsafe impl RefCounted for nsICookieService {
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
pub trait nsICookieServiceCoerce {
    fn coerce_from(v: &nsICookieService) -> &Self;
}

impl nsICookieServiceCoerce for nsICookieService {
    #[inline]
    fn coerce_from(v: &nsICookieService) -> &Self {
        v
    }
}

impl nsICookieService {
    #[inline]
    pub fn coerce<T: nsICookieServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICookieService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICookieServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICookieService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICookieServiceVTable {
    pub __base: nsISupportsVTable,

    /* string getCookieString (in nsIURI aURI, in nsIChannel aChannel); */
    pub getCookieString: unsafe extern "C" fn (this: *const nsICookieService, aURI: *const nsIURI, aChannel: *const nsIChannel, _retval: *mut *const libc::c_char) -> nsresult,

    /* string getCookieStringFromHttp (in nsIURI aURI, in nsIURI aFirstURI, in nsIChannel aChannel); */
    pub getCookieStringFromHttp: unsafe extern "C" fn (this: *const nsICookieService, aURI: *const nsIURI, aFirstURI: *const nsIURI, aChannel: *const nsIChannel, _retval: *mut *const libc::c_char) -> nsresult,

    /* void setCookieString (in nsIURI aURI, in nsIPrompt aPrompt, in string aCookie, in nsIChannel aChannel); */
    pub setCookieString: unsafe extern "C" fn (this: *const nsICookieService, aURI: *const nsIURI, aPrompt: *const nsIPrompt, aCookie: *const libc::c_char, aChannel: *const nsIChannel) -> nsresult,

    /* void setCookieStringFromHttp (in nsIURI aURI, in nsIURI aFirstURI, in nsIPrompt aPrompt, in string aCookie, in string aServerTime, in nsIChannel aChannel); */
    pub setCookieStringFromHttp: unsafe extern "C" fn (this: *const nsICookieService, aURI: *const nsIURI, aFirstURI: *const nsIURI, aPrompt: *const nsIPrompt, aCookie: *const libc::c_char, aServerTime: *const libc::c_char, aChannel: *const nsIChannel) -> nsresult,

}


impl nsICookieService {
    /* string getCookieString (in nsIURI aURI, in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn getCookieString(&self, aURI: Option<&nsIURI>, aChannel: Option<&nsIChannel>) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getCookieString)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aChannel.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* string getCookieStringFromHttp (in nsIURI aURI, in nsIURI aFirstURI, in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn getCookieStringFromHttp(&self, aURI: Option<&nsIURI>, aFirstURI: Option<&nsIURI>, aChannel: Option<&nsIChannel>) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getCookieStringFromHttp)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aFirstURI.map_or(::std::ptr::null(), |x| x as *const _), aChannel.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setCookieString (in nsIURI aURI, in nsIPrompt aPrompt, in string aCookie, in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn setCookieString(&self, aURI: Option<&nsIURI>, aPrompt: Option<&nsIPrompt>, aCookie: *const libc::c_char, aChannel: Option<&nsIChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).setCookieString)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aPrompt.map_or(::std::ptr::null(), |x| x as *const _), aCookie, aChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCookieStringFromHttp (in nsIURI aURI, in nsIURI aFirstURI, in nsIPrompt aPrompt, in string aCookie, in string aServerTime, in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn setCookieStringFromHttp(&self, aURI: Option<&nsIURI>, aFirstURI: Option<&nsIURI>, aPrompt: Option<&nsIPrompt>, aCookie: *const libc::c_char, aServerTime: *const libc::c_char, aChannel: Option<&nsIChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).setCookieStringFromHttp)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aFirstURI.map_or(::std::ptr::null(), |x| x as *const _), aPrompt.map_or(::std::ptr::null(), |x| x as *const _), aCookie, aServerTime, aChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


