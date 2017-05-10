//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEncodedChannel.idl
//


#[repr(C)]
pub struct nsIEncodedChannel {
    vtable: *const nsIEncodedChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEncodedChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x29c29ce6, 0x8ce4, 0x45e6,
            [0x8d, 0x60, 0x36, 0xc8, 0xfa, 0x3e, 0x25, 0x5b])
    }
}

unsafe impl RefCounted for nsIEncodedChannel {
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
pub trait nsIEncodedChannelCoerce {
    fn coerce_from(v: &nsIEncodedChannel) -> &Self;
}

impl nsIEncodedChannelCoerce for nsIEncodedChannel {
    #[inline]
    fn coerce_from(v: &nsIEncodedChannel) -> &Self {
        v
    }
}

impl nsIEncodedChannel {
    #[inline]
    pub fn coerce<T: nsIEncodedChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEncodedChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEncodedChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEncodedChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEncodedChannelVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIUTF8StringEnumerator contentEncodings; */
    pub get_contentEncodings: unsafe extern "C" fn (this: *const nsIEncodedChannel, aContentEncodings: *mut *const nsIUTF8StringEnumerator) -> nsresult,

    /* attribute boolean applyConversion; */
    pub get_applyConversion: unsafe extern "C" fn (this: *const nsIEncodedChannel, aApplyConversion: *mut bool) -> nsresult,
    pub set_applyConversion: unsafe extern "C" fn (this: *const nsIEncodedChannel, aApplyConversion: bool) -> nsresult,

    /* void doApplyContentConversions (in nsIStreamListener aNextListener, out nsIStreamListener aNewNextListener, in nsISupports aCtxt); */
    pub doApplyContentConversions: unsafe extern "C" fn (this: *const nsIEncodedChannel, aNextListener: *const nsIStreamListener, aNewNextListener: *mut *const nsIStreamListener, aCtxt: *const nsISupports) -> nsresult,

}


impl nsIEncodedChannel {
    /* readonly attribute nsIUTF8StringEnumerator contentEncodings; */
    #[inline]
    pub unsafe fn get_contentEncodings(&self, ) -> Result<Option<RefPtr<nsIUTF8StringEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contentEncodings)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean applyConversion; */
    #[inline]
    pub unsafe fn get_applyConversion(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_applyConversion)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_applyConversion(&self, aApplyConversion: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_applyConversion)(self as *const _, aApplyConversion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void doApplyContentConversions (in nsIStreamListener aNextListener, out nsIStreamListener aNewNextListener, in nsISupports aCtxt); */
    #[inline]
    pub unsafe fn doApplyContentConversions(&self, aNextListener: Option<&nsIStreamListener>, aCtxt: Option<&nsISupports>) -> Result<Option<RefPtr<nsIStreamListener>>, nsresult> {
        let mut aNewNextListener = GetterAddrefs::new();
        match ((*self.vtable).doApplyContentConversions)(self as *const _, aNextListener.map_or(::std::ptr::null(), |x| x as *const _), aNewNextListener.ptr(), aCtxt.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aNewNextListener.refptr())
    }

}


