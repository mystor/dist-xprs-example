//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInputStreamChannel.idl
//


#[repr(C)]
pub struct nsIInputStreamChannel {
    vtable: *const nsIInputStreamChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInputStreamChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xea730238, 0x4bfd, 0x4015,
            [0x84, 0x89, 0x8f, 0x26, 0x4d, 0x05, 0xb3, 0x43])
    }
}

unsafe impl RefCounted for nsIInputStreamChannel {
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
pub trait nsIInputStreamChannelCoerce {
    fn coerce_from(v: &nsIInputStreamChannel) -> &Self;
}

impl nsIInputStreamChannelCoerce for nsIInputStreamChannel {
    #[inline]
    fn coerce_from(v: &nsIInputStreamChannel) -> &Self {
        v
    }
}

impl nsIInputStreamChannel {
    #[inline]
    pub fn coerce<T: nsIInputStreamChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInputStreamChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInputStreamChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInputStreamChannelVTable {
    pub __base: nsISupportsVTable,

    /* void setURI (in nsIURI aURI); */
    pub setURI: unsafe extern "C" fn (this: *const nsIInputStreamChannel, aURI: *const nsIURI) -> nsresult,

    /* attribute nsIInputStream contentStream; */
    pub get_contentStream: unsafe extern "C" fn (this: *const nsIInputStreamChannel, aContentStream: *mut *const nsIInputStream) -> nsresult,
    pub set_contentStream: unsafe extern "C" fn (this: *const nsIInputStreamChannel, aContentStream: *const nsIInputStream) -> nsresult,

    /* attribute AString srcdocData; */
    pub get_srcdocData: unsafe extern "C" fn (this: *const nsIInputStreamChannel, aSrcdocData: *mut nsAString) -> nsresult,
    pub set_srcdocData: unsafe extern "C" fn (this: *const nsIInputStreamChannel, aSrcdocData: *const nsAString) -> nsresult,

    /* readonly attribute boolean isSrcdocChannel; */
    pub get_isSrcdocChannel: unsafe extern "C" fn (this: *const nsIInputStreamChannel, aIsSrcdocChannel: *mut bool) -> nsresult,

    /* attribute nsIURI baseURI; */
    pub get_baseURI: unsafe extern "C" fn (this: *const nsIInputStreamChannel, aBaseURI: *mut *const nsIURI) -> nsresult,
    pub set_baseURI: unsafe extern "C" fn (this: *const nsIInputStreamChannel, aBaseURI: *const nsIURI) -> nsresult,

}


impl nsIInputStreamChannel {
    /* void setURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn setURI(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).setURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIInputStream contentStream; */
    #[inline]
    pub unsafe fn get_contentStream(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contentStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_contentStream(&self, aContentStream: Option<&nsIInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).set_contentStream)(self as *const _, aContentStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString srcdocData; */
    #[inline]
    pub unsafe fn get_srcdocData(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_srcdocData)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_srcdocData(&self, aSrcdocData: &[u16]) -> Result<(), nsresult> {
        let aSrcdocData = nsString::from(aSrcdocData);
        match ((*self.vtable).set_srcdocData)(self as *const _, &*aSrcdocData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isSrcdocChannel; */
    #[inline]
    pub unsafe fn get_isSrcdocChannel(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSrcdocChannel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIURI baseURI; */
    #[inline]
    pub unsafe fn get_baseURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_baseURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_baseURI(&self, aBaseURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_baseURI)(self as *const _, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


