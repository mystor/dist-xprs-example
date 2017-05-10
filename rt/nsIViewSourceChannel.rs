//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIViewSourceChannel.idl
//


#[repr(C)]
pub struct nsIViewSourceChannel {
    vtable: *const nsIViewSourceChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIViewSourceChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3e9800f8, 0xedb7, 0x4c9a,
            [0x92, 0x85, 0x09, 0xb4, 0xf0, 0x45, 0xb0, 0x19])
    }
}

unsafe impl RefCounted for nsIViewSourceChannel {
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
pub trait nsIViewSourceChannelCoerce {
    fn coerce_from(v: &nsIViewSourceChannel) -> &Self;
}

impl nsIViewSourceChannelCoerce for nsIViewSourceChannel {
    #[inline]
    fn coerce_from(v: &nsIViewSourceChannel) -> &Self {
        v
    }
}

impl nsIViewSourceChannel {
    #[inline]
    pub fn coerce<T: nsIViewSourceChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIViewSourceChannel {
    type Target = nsIChannel;
    #[inline]
    fn deref(&self) -> &nsIChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIChannelCoerce> nsIViewSourceChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIViewSourceChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIViewSourceChannelVTable {
    pub __base: nsIChannelVTable,

    /* [must_use] attribute ACString originalContentType; */
    pub get_originalContentType: unsafe extern "C" fn (this: *const nsIViewSourceChannel, aOriginalContentType: *mut nsACString) -> nsresult,
    pub set_originalContentType: unsafe extern "C" fn (this: *const nsIViewSourceChannel, aOriginalContentType: *const nsACString) -> nsresult,

    /* [must_use] readonly attribute boolean isSrcdocChannel; */
    pub get_isSrcdocChannel: unsafe extern "C" fn (this: *const nsIViewSourceChannel, aIsSrcdocChannel: *mut bool) -> nsresult,

    /* [must_use] attribute nsIURI baseURI; */
    pub get_baseURI: unsafe extern "C" fn (this: *const nsIViewSourceChannel, aBaseURI: *mut *const nsIURI) -> nsresult,
    pub set_baseURI: unsafe extern "C" fn (this: *const nsIViewSourceChannel, aBaseURI: *const nsIURI) -> nsresult,

}


impl nsIViewSourceChannel {
    /* [must_use] attribute ACString originalContentType; */
    #[inline]
    pub unsafe fn get_originalContentType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_originalContentType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_originalContentType(&self, aOriginalContentType: &[u8]) -> Result<(), nsresult> {
        let aOriginalContentType = nsCString::from(aOriginalContentType);
        match ((*self.vtable).set_originalContentType)(self as *const _, &*aOriginalContentType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute boolean isSrcdocChannel; */
    #[inline]
    pub unsafe fn get_isSrcdocChannel(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSrcdocChannel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] attribute nsIURI baseURI; */
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


