//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMultiPartChannel.idl
//


#[repr(C)]
pub struct nsIMultiPartChannel {
    vtable: *const nsIMultiPartChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMultiPartChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4fefb490, 0x5567, 0x11e5,
            [0xa8, 0x37, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIMultiPartChannel {
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
pub trait nsIMultiPartChannelCoerce {
    fn coerce_from(v: &nsIMultiPartChannel) -> &Self;
}

impl nsIMultiPartChannelCoerce for nsIMultiPartChannel {
    #[inline]
    fn coerce_from(v: &nsIMultiPartChannel) -> &Self {
        v
    }
}

impl nsIMultiPartChannel {
    #[inline]
    pub fn coerce<T: nsIMultiPartChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMultiPartChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMultiPartChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMultiPartChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMultiPartChannelVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIChannel baseChannel; */
    pub get_baseChannel: unsafe extern "C" fn (this: *const nsIMultiPartChannel, aBaseChannel: *mut *const nsIChannel) -> nsresult,

    /* readonly attribute uint32_t partID; */
    pub get_partID: unsafe extern "C" fn (this: *const nsIMultiPartChannel, aPartID: *mut uint32_t) -> nsresult,

    /* readonly attribute boolean isLastPart; */
    pub get_isLastPart: unsafe extern "C" fn (this: *const nsIMultiPartChannel, aIsLastPart: *mut bool) -> nsresult,

}


impl nsIMultiPartChannel {
    /* readonly attribute nsIChannel baseChannel; */
    #[inline]
    pub unsafe fn get_baseChannel(&self, ) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_baseChannel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute uint32_t partID; */
    #[inline]
    pub unsafe fn get_partID(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_partID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isLastPart; */
    #[inline]
    pub unsafe fn get_isLastPart(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isLastPart)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


