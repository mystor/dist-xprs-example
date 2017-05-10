//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURLFormatter.idl
//


#[repr(C)]
pub struct nsIURLFormatter {
    vtable: *const nsIURLFormatterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURLFormatter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4ab31d30, 0x372d, 0x11db,
            [0xa9, 0x8b, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIURLFormatter {
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
pub trait nsIURLFormatterCoerce {
    fn coerce_from(v: &nsIURLFormatter) -> &Self;
}

impl nsIURLFormatterCoerce for nsIURLFormatter {
    #[inline]
    fn coerce_from(v: &nsIURLFormatter) -> &Self {
        v
    }
}

impl nsIURLFormatter {
    #[inline]
    pub fn coerce<T: nsIURLFormatterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURLFormatter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURLFormatterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURLFormatter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURLFormatterVTable {
    pub __base: nsISupportsVTable,

    /* AString formatURL (in AString aFormat); */
    pub formatURL: unsafe extern "C" fn (this: *const nsIURLFormatter, aFormat: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* AString formatURLPref (in AString aPref); */
    pub formatURLPref: unsafe extern "C" fn (this: *const nsIURLFormatter, aPref: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* AString trimSensitiveURLs (in AString aMsg); */
    pub trimSensitiveURLs: unsafe extern "C" fn (this: *const nsIURLFormatter, aMsg: *const nsAString, _retval: *mut nsAString) -> nsresult,

}


impl nsIURLFormatter {
    /* AString formatURL (in AString aFormat); */
    #[inline]
    pub unsafe fn formatURL(&self, aFormat: &[u16]) -> Result<nsString, nsresult> {
        let aFormat = nsString::from(aFormat);
        let mut _retval = nsString::new();
        match ((*self.vtable).formatURL)(self as *const _, &*aFormat, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString formatURLPref (in AString aPref); */
    #[inline]
    pub unsafe fn formatURLPref(&self, aPref: &[u16]) -> Result<nsString, nsresult> {
        let aPref = nsString::from(aPref);
        let mut _retval = nsString::new();
        match ((*self.vtable).formatURLPref)(self as *const _, &*aPref, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString trimSensitiveURLs (in AString aMsg); */
    #[inline]
    pub unsafe fn trimSensitiveURLs(&self, aMsg: &[u16]) -> Result<nsString, nsresult> {
        let aMsg = nsString::from(aMsg);
        let mut _retval = nsString::new();
        match ((*self.vtable).trimSensitiveURLs)(self as *const _, &*aMsg, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


