//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITimeService.idl
//


#[repr(C)]
pub struct nsITimeService {
    vtable: *const nsITimeServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITimeService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1fc7fde2, 0x0090, 0x11e2,
            [0xbd, 0xd6, 0x0f, 0xea, 0x4b, 0x9f, 0x41, 0xf8])
    }
}

unsafe impl RefCounted for nsITimeService {
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
pub trait nsITimeServiceCoerce {
    fn coerce_from(v: &nsITimeService) -> &Self;
}

impl nsITimeServiceCoerce for nsITimeService {
    #[inline]
    fn coerce_from(v: &nsITimeService) -> &Self {
        v
    }
}

impl nsITimeService {
    #[inline]
    pub fn coerce<T: nsITimeServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITimeService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITimeServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITimeService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITimeServiceVTable {
    pub __base: nsISupportsVTable,

    /* void set (in int64_t aTimeInMS); */
    pub set: unsafe extern "C" fn (this: *const nsITimeService, aTimeInMS: int64_t) -> nsresult,

}


impl nsITimeService {
    /* void set (in int64_t aTimeInMS); */
    #[inline]
    pub unsafe fn set(&self, aTimeInMS: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set)(self as *const _, aTimeInMS) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


