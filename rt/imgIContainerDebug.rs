//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgIContainerDebug.idl
//


#[repr(C)]
pub struct imgIContainerDebug {
    vtable: *const imgIContainerDebugVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for imgIContainerDebug {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x52cbb839, 0x6e63, 0x4a70,
            [0xb2, 0x1a, 0x1d, 0xb4, 0xca, 0x70, 0x6c, 0x49])
    }
}

unsafe impl RefCounted for imgIContainerDebug {
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
pub trait imgIContainerDebugCoerce {
    fn coerce_from(v: &imgIContainerDebug) -> &Self;
}

impl imgIContainerDebugCoerce for imgIContainerDebug {
    #[inline]
    fn coerce_from(v: &imgIContainerDebug) -> &Self {
        v
    }
}

impl imgIContainerDebug {
    #[inline]
    pub fn coerce<T: imgIContainerDebugCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for imgIContainerDebug {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> imgIContainerDebugCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIContainerDebug) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct imgIContainerDebugVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t framesNotified; */
    pub get_framesNotified: unsafe extern "C" fn (this: *const imgIContainerDebug, aFramesNotified: *mut uint32_t) -> nsresult,

}


impl imgIContainerDebug {
    /* readonly attribute uint32_t framesNotified; */
    #[inline]
    pub unsafe fn get_framesNotified(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_framesNotified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


