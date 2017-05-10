//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPromptService2.idl
//


#[repr(C)]
pub struct nsIPromptService2 {
    vtable: *const nsIPromptService2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPromptService2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3775ad32, 0x8326, 0x422b,
            [0x9f, 0xf3, 0x87, 0xef, 0x1d, 0x3f, 0x9f, 0x0e])
    }
}

unsafe impl RefCounted for nsIPromptService2 {
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
pub trait nsIPromptService2Coerce {
    fn coerce_from(v: &nsIPromptService2) -> &Self;
}

impl nsIPromptService2Coerce for nsIPromptService2 {
    #[inline]
    fn coerce_from(v: &nsIPromptService2) -> &Self {
        v
    }
}

impl nsIPromptService2 {
    #[inline]
    pub fn coerce<T: nsIPromptService2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPromptService2 {
    type Target = nsIPromptService;
    #[inline]
    fn deref(&self) -> &nsIPromptService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPromptServiceCoerce> nsIPromptService2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIPromptService2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPromptService2VTable {
    pub __base: nsIPromptServiceVTable,

    /* boolean promptAuth (in mozIDOMWindowProxy aParent, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue); */
    pub promptAuth: unsafe extern "C" fn (this: *const nsIPromptService2, aParent: *const mozIDOMWindowProxy, aChannel: *const nsIChannel, level: uint32_t, authInfo: *const nsIAuthInformation, checkboxLabel: *const libc::int16_t, checkValue: *mut bool, _retval: *mut bool) -> nsresult,

    /* nsICancelable asyncPromptAuth (in mozIDOMWindowProxy aParent, in nsIChannel aChannel, in nsIAuthPromptCallback aCallback, in nsISupports aContext, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue); */
    pub asyncPromptAuth: unsafe extern "C" fn (this: *const nsIPromptService2, aParent: *const mozIDOMWindowProxy, aChannel: *const nsIChannel, aCallback: *const nsIAuthPromptCallback, aContext: *const nsISupports, level: uint32_t, authInfo: *const nsIAuthInformation, checkboxLabel: *const libc::int16_t, checkValue: *mut bool, _retval: *mut *const nsICancelable) -> nsresult,

}


impl nsIPromptService2 {
    /* boolean promptAuth (in mozIDOMWindowProxy aParent, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue); */
    #[inline]
    pub unsafe fn promptAuth(&self, aParent: Option<&mozIDOMWindowProxy>, aChannel: Option<&nsIChannel>, level: uint32_t, authInfo: Option<&nsIAuthInformation>, checkboxLabel: *const libc::int16_t) -> Result<(bool, bool), nsresult> {
        let mut checkValue: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).promptAuth)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aChannel.map_or(::std::ptr::null(), |x| x as *const _), level, authInfo.map_or(::std::ptr::null(), |x| x as *const _), checkboxLabel, &mut checkValue as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((checkValue, _retval))
    }

    /* nsICancelable asyncPromptAuth (in mozIDOMWindowProxy aParent, in nsIChannel aChannel, in nsIAuthPromptCallback aCallback, in nsISupports aContext, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue); */
    #[inline]
    pub unsafe fn asyncPromptAuth(&self, aParent: Option<&mozIDOMWindowProxy>, aChannel: Option<&nsIChannel>, aCallback: Option<&nsIAuthPromptCallback>, aContext: Option<&nsISupports>, level: uint32_t, authInfo: Option<&nsIAuthInformation>, checkboxLabel: *const libc::int16_t) -> Result<(bool, Option<RefPtr<nsICancelable>>), nsresult> {
        let mut checkValue: bool = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).asyncPromptAuth)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aChannel.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), level, authInfo.map_or(::std::ptr::null(), |x| x as *const _), checkboxLabel, &mut checkValue as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((checkValue, _retval.refptr()))
    }

}


