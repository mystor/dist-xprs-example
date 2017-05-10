//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWyciwygChannel.idl
//


#[repr(C)]
pub struct nsIWyciwygChannel {
    vtable: *const nsIWyciwygChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWyciwygChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8b8f3341, 0x46da, 0x40f5,
            [0xa1, 0x6f, 0x41, 0xa9, 0x1f, 0x5d, 0x25, 0xdd])
    }
}

unsafe impl RefCounted for nsIWyciwygChannel {
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
pub trait nsIWyciwygChannelCoerce {
    fn coerce_from(v: &nsIWyciwygChannel) -> &Self;
}

impl nsIWyciwygChannelCoerce for nsIWyciwygChannel {
    #[inline]
    fn coerce_from(v: &nsIWyciwygChannel) -> &Self {
        v
    }
}

impl nsIWyciwygChannel {
    #[inline]
    pub fn coerce<T: nsIWyciwygChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWyciwygChannel {
    type Target = nsIChannel;
    #[inline]
    fn deref(&self) -> &nsIChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIChannelCoerce> nsIWyciwygChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWyciwygChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWyciwygChannelVTable {
    pub __base: nsIChannelVTable,

    /* void writeToCacheEntry (in AString aData); */
    pub writeToCacheEntry: unsafe extern "C" fn (this: *const nsIWyciwygChannel, aData: *const nsAString) -> nsresult,

    /* void closeCacheEntry (in nsresult reason); */
    pub closeCacheEntry: unsafe extern "C" fn (this: *const nsIWyciwygChannel, reason: nsresult) -> nsresult,

    /* void setSecurityInfo (in nsISupports aSecurityInfo); */
    pub setSecurityInfo: unsafe extern "C" fn (this: *const nsIWyciwygChannel, aSecurityInfo: *const nsISupports) -> nsresult,

    /* void setCharsetAndSource (in long aSource, in ACString aCharset); */
    pub setCharsetAndSource: unsafe extern "C" fn (this: *const nsIWyciwygChannel, aSource: libc::int32_t, aCharset: *const nsACString) -> nsresult,

    /* ACString getCharsetAndSource (out long aSource); */
    pub getCharsetAndSource: unsafe extern "C" fn (this: *const nsIWyciwygChannel, aSource: *mut libc::int32_t, _retval: *mut nsACString) -> nsresult,

}


impl nsIWyciwygChannel {
    /* void writeToCacheEntry (in AString aData); */
    #[inline]
    pub unsafe fn writeToCacheEntry(&self, aData: &[u16]) -> Result<(), nsresult> {
        let aData = nsString::from(aData);
        match ((*self.vtable).writeToCacheEntry)(self as *const _, &*aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void closeCacheEntry (in nsresult reason); */
    #[inline]
    pub unsafe fn closeCacheEntry(&self, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).closeCacheEntry)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setSecurityInfo (in nsISupports aSecurityInfo); */
    #[inline]
    pub unsafe fn setSecurityInfo(&self, aSecurityInfo: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).setSecurityInfo)(self as *const _, aSecurityInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCharsetAndSource (in long aSource, in ACString aCharset); */
    #[inline]
    pub unsafe fn setCharsetAndSource(&self, aSource: libc::int32_t, aCharset: &[u8]) -> Result<(), nsresult> {
        let aCharset = nsCString::from(aCharset);
        match ((*self.vtable).setCharsetAndSource)(self as *const _, aSource, &*aCharset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* ACString getCharsetAndSource (out long aSource); */
    #[inline]
    pub unsafe fn getCharsetAndSource(&self, ) -> Result<(libc::int32_t, nsCString), nsresult> {
        let mut aSource: libc::int32_t = ::std::mem::zeroed();
        let mut _retval = nsCString::new();
        match ((*self.vtable).getCharsetAndSource)(self as *const _, &mut aSource as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aSource, _retval))
    }

}


