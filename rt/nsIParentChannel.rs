//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIParentChannel.idl
//


#[repr(C)]
pub struct nsIParentChannel {
    vtable: *const nsIParentChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIParentChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe0fc4801, 0x6030, 0x4653,
            [0xa5, 0x9f, 0x1f, 0xb2, 0x82, 0xbd, 0x1a, 0x04])
    }
}

unsafe impl RefCounted for nsIParentChannel {
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
pub trait nsIParentChannelCoerce {
    fn coerce_from(v: &nsIParentChannel) -> &Self;
}

impl nsIParentChannelCoerce for nsIParentChannel {
    #[inline]
    fn coerce_from(v: &nsIParentChannel) -> &Self {
        v
    }
}

impl nsIParentChannel {
    #[inline]
    pub fn coerce<T: nsIParentChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIParentChannel {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsIParentChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIParentChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIParentChannelVTable {
    pub __base: nsIStreamListenerVTable,

    /* [noscript] void setParentListener (in HttpChannelParentListener listener); */
    /// Unable to call function as its signature contains a non-rust type
    pub setParentListener: *const ::libc::c_void,

    /* [noscript] void notifyTrackingProtectionDisabled (); */
    pub notifyTrackingProtectionDisabled: unsafe extern "C" fn (this: *const nsIParentChannel) -> nsresult,

    /* [noscript] void setClassifierMatchedInfo (in ACString aList, in ACString aProvider, in ACString aPrefix); */
    pub setClassifierMatchedInfo: unsafe extern "C" fn (this: *const nsIParentChannel, aList: *const nsACString, aProvider: *const nsACString, aPrefix: *const nsACString) -> nsresult,

    /* [noscript] void notifyTrackingResource (); */
    pub notifyTrackingResource: unsafe extern "C" fn (this: *const nsIParentChannel) -> nsresult,

    /* void delete (); */
    pub delete: unsafe extern "C" fn (this: *const nsIParentChannel) -> nsresult,

}


impl nsIParentChannel {
    /* [noscript] void setParentListener (in HttpChannelParentListener listener); */


    /* [noscript] void notifyTrackingProtectionDisabled (); */
    #[inline]
    pub unsafe fn notifyTrackingProtectionDisabled(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).notifyTrackingProtectionDisabled)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setClassifierMatchedInfo (in ACString aList, in ACString aProvider, in ACString aPrefix); */
    #[inline]
    pub unsafe fn setClassifierMatchedInfo(&self, aList: &[u8], aProvider: &[u8], aPrefix: &[u8]) -> Result<(), nsresult> {
        let aList = nsCString::from(aList);
        let aProvider = nsCString::from(aProvider);
        let aPrefix = nsCString::from(aPrefix);
        match ((*self.vtable).setClassifierMatchedInfo)(self as *const _, &*aList, &*aProvider, &*aPrefix) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void notifyTrackingResource (); */
    #[inline]
    pub unsafe fn notifyTrackingResource(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).notifyTrackingResource)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void delete (); */
    #[inline]
    pub unsafe fn delete(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).delete)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


