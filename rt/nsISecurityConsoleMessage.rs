//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecurityConsoleMessage.idl
//


#[repr(C)]
pub struct nsISecurityConsoleMessage {
    vtable: *const nsISecurityConsoleMessageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISecurityConsoleMessage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfe9fc9b6, 0xdde2, 0x11e2,
            [0xa8, 0xf1, 0x0a, 0x32, 0x61, 0x88, 0x70, 0x9b])
    }
}

unsafe impl RefCounted for nsISecurityConsoleMessage {
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
pub trait nsISecurityConsoleMessageCoerce {
    fn coerce_from(v: &nsISecurityConsoleMessage) -> &Self;
}

impl nsISecurityConsoleMessageCoerce for nsISecurityConsoleMessage {
    #[inline]
    fn coerce_from(v: &nsISecurityConsoleMessage) -> &Self {
        v
    }
}

impl nsISecurityConsoleMessage {
    #[inline]
    pub fn coerce<T: nsISecurityConsoleMessageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISecurityConsoleMessage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISecurityConsoleMessageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecurityConsoleMessage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISecurityConsoleMessageVTable {
    pub __base: nsISupportsVTable,

    /* attribute AString tag; */
    pub get_tag: unsafe extern "C" fn (this: *const nsISecurityConsoleMessage, aTag: *mut nsAString) -> nsresult,
    pub set_tag: unsafe extern "C" fn (this: *const nsISecurityConsoleMessage, aTag: *const nsAString) -> nsresult,

    /* attribute AString category; */
    pub get_category: unsafe extern "C" fn (this: *const nsISecurityConsoleMessage, aCategory: *mut nsAString) -> nsresult,
    pub set_category: unsafe extern "C" fn (this: *const nsISecurityConsoleMessage, aCategory: *const nsAString) -> nsresult,

}


impl nsISecurityConsoleMessage {
    /* attribute AString tag; */
    #[inline]
    pub unsafe fn get_tag(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_tag)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_tag(&self, aTag: &[u16]) -> Result<(), nsresult> {
        let aTag = nsString::from(aTag);
        match ((*self.vtable).set_tag)(self as *const _, &*aTag) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString category; */
    #[inline]
    pub unsafe fn get_category(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_category)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_category(&self, aCategory: &[u16]) -> Result<(), nsresult> {
        let aCategory = nsString::from(aCategory);
        match ((*self.vtable).set_category)(self as *const _, &*aCategory) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


