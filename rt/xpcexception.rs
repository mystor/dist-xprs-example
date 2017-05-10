//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpcexception.idl
//


#[repr(C)]
pub struct nsIXPCException {
    vtable: *const nsIXPCExceptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCException {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x875e6645, 0xe762, 0x4da6,
            [0x9e, 0xc8, 0xbf, 0x19, 0xab, 0x00, 0x50, 0xdf])
    }
}

unsafe impl RefCounted for nsIXPCException {
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
pub trait nsIXPCExceptionCoerce {
    fn coerce_from(v: &nsIXPCException) -> &Self;
}

impl nsIXPCExceptionCoerce for nsIXPCException {
    #[inline]
    fn coerce_from(v: &nsIXPCException) -> &Self {
        v
    }
}

impl nsIXPCException {
    #[inline]
    pub fn coerce<T: nsIXPCExceptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCException {
    type Target = nsIException;
    #[inline]
    fn deref(&self) -> &nsIException {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIExceptionCoerce> nsIXPCExceptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCException) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCExceptionVTable {
    pub __base: nsIExceptionVTable,

    /* void initialize (in AUTF8String aMessage, in nsresult aResult, in AUTF8String aName, in nsIStackFrame aLocation, in nsISupports aData); */
    pub initialize: unsafe extern "C" fn (this: *const nsIXPCException, aMessage: *const nsACString, aResult: nsresult, aName: *const nsACString, aLocation: *const nsIStackFrame, aData: *const nsISupports) -> nsresult,

}


impl nsIXPCException {
    /* void initialize (in AUTF8String aMessage, in nsresult aResult, in AUTF8String aName, in nsIStackFrame aLocation, in nsISupports aData); */
    #[inline]
    pub unsafe fn initialize(&self, aMessage: &[u8], aResult: nsresult, aName: &[u8], aLocation: Option<&nsIStackFrame>, aData: Option<&nsISupports>) -> Result<(), nsresult> {
        let aMessage = nsCString::from(aMessage);
        let aName = nsCString::from(aName);
        match ((*self.vtable).initialize)(self as *const _, &*aMessage, aResult, &*aName, aLocation.map_or(::std::ptr::null(), |x| x as *const _), aData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


