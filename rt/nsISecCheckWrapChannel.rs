//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecCheckWrapChannel.idl
//


#[repr(C)]
pub struct nsISecCheckWrapChannel {
    vtable: *const nsISecCheckWrapChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISecCheckWrapChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9446c5d5, 0xc9fb, 0x4a6e,
            [0xac, 0xf9, 0xca, 0x4f, 0xc6, 0x66, 0xef, 0xe0])
    }
}

unsafe impl RefCounted for nsISecCheckWrapChannel {
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
pub trait nsISecCheckWrapChannelCoerce {
    fn coerce_from(v: &nsISecCheckWrapChannel) -> &Self;
}

impl nsISecCheckWrapChannelCoerce for nsISecCheckWrapChannel {
    #[inline]
    fn coerce_from(v: &nsISecCheckWrapChannel) -> &Self {
        v
    }
}

impl nsISecCheckWrapChannel {
    #[inline]
    pub fn coerce<T: nsISecCheckWrapChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISecCheckWrapChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISecCheckWrapChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecCheckWrapChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISecCheckWrapChannelVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIChannel innerChannel; */
    pub get_innerChannel: unsafe extern "C" fn (this: *const nsISecCheckWrapChannel, aInnerChannel: *mut *const nsIChannel) -> nsresult,

}


impl nsISecCheckWrapChannel {
    /* readonly attribute nsIChannel innerChannel; */
    #[inline]
    pub unsafe fn get_innerChannel(&self, ) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_innerChannel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


