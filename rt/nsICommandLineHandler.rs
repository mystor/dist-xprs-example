//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandLineHandler.idl
//


#[repr(C)]
pub struct nsICommandLineHandler {
    vtable: *const nsICommandLineHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICommandLineHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd4b123df, 0x51ee, 0x48b1,
            [0xa6, 0x63, 0x00, 0x21, 0x80, 0xe6, 0x0d, 0x3b])
    }
}

unsafe impl RefCounted for nsICommandLineHandler {
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
pub trait nsICommandLineHandlerCoerce {
    fn coerce_from(v: &nsICommandLineHandler) -> &Self;
}

impl nsICommandLineHandlerCoerce for nsICommandLineHandler {
    #[inline]
    fn coerce_from(v: &nsICommandLineHandler) -> &Self {
        v
    }
}

impl nsICommandLineHandler {
    #[inline]
    pub fn coerce<T: nsICommandLineHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICommandLineHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICommandLineHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandLineHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICommandLineHandlerVTable {
    pub __base: nsISupportsVTable,

    /* void handle (in nsICommandLine aCommandLine); */
    pub handle: unsafe extern "C" fn (this: *const nsICommandLineHandler, aCommandLine: *const nsICommandLine) -> nsresult,

    /* readonly attribute AUTF8String helpInfo; */
    pub get_helpInfo: unsafe extern "C" fn (this: *const nsICommandLineHandler, aHelpInfo: *mut nsACString) -> nsresult,

}


impl nsICommandLineHandler {
    /* void handle (in nsICommandLine aCommandLine); */
    #[inline]
    pub unsafe fn handle(&self, aCommandLine: Option<&nsICommandLine>) -> Result<(), nsresult> {

        match ((*self.vtable).handle)(self as *const _, aCommandLine.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute AUTF8String helpInfo; */
    #[inline]
    pub unsafe fn get_helpInfo(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_helpInfo)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


