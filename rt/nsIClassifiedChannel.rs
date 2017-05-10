//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClassifiedChannel.idl
//


#[repr(C)]
pub struct nsIClassifiedChannel {
    vtable: *const nsIClassifiedChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClassifiedChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x70cf6091, 0xa1de, 0x4aa8,
            [0x82, 0x24, 0x05, 0x8f, 0x89, 0x64, 0xbe, 0x31])
    }
}

unsafe impl RefCounted for nsIClassifiedChannel {
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
pub trait nsIClassifiedChannelCoerce {
    fn coerce_from(v: &nsIClassifiedChannel) -> &Self;
}

impl nsIClassifiedChannelCoerce for nsIClassifiedChannel {
    #[inline]
    fn coerce_from(v: &nsIClassifiedChannel) -> &Self {
        v
    }
}

impl nsIClassifiedChannel {
    #[inline]
    pub fn coerce<T: nsIClassifiedChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClassifiedChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClassifiedChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClassifiedChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClassifiedChannelVTable {
    pub __base: nsISupportsVTable,

    /* void setMatchedInfo (in ACString aList, in ACString aProvider, in ACString aPrefix); */
    pub setMatchedInfo: unsafe extern "C" fn (this: *const nsIClassifiedChannel, aList: *const nsACString, aProvider: *const nsACString, aPrefix: *const nsACString) -> nsresult,

    /* readonly attribute ACString matchedList; */
    pub get_matchedList: unsafe extern "C" fn (this: *const nsIClassifiedChannel, aMatchedList: *mut nsACString) -> nsresult,

    /* readonly attribute ACString matchedProvider; */
    pub get_matchedProvider: unsafe extern "C" fn (this: *const nsIClassifiedChannel, aMatchedProvider: *mut nsACString) -> nsresult,

    /* readonly attribute ACString matchedPrefix; */
    pub get_matchedPrefix: unsafe extern "C" fn (this: *const nsIClassifiedChannel, aMatchedPrefix: *mut nsACString) -> nsresult,

}


impl nsIClassifiedChannel {
    /* void setMatchedInfo (in ACString aList, in ACString aProvider, in ACString aPrefix); */
    #[inline]
    pub unsafe fn setMatchedInfo(&self, aList: &[u8], aProvider: &[u8], aPrefix: &[u8]) -> Result<(), nsresult> {
        let aList = nsCString::from(aList);
        let aProvider = nsCString::from(aProvider);
        let aPrefix = nsCString::from(aPrefix);
        match ((*self.vtable).setMatchedInfo)(self as *const _, &*aList, &*aProvider, &*aPrefix) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute ACString matchedList; */
    #[inline]
    pub unsafe fn get_matchedList(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_matchedList)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString matchedProvider; */
    #[inline]
    pub unsafe fn get_matchedProvider(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_matchedProvider)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString matchedPrefix; */
    #[inline]
    pub unsafe fn get_matchedPrefix(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_matchedPrefix)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


