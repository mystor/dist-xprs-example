//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMemoryProfiler.idl
//


#[repr(C)]
pub struct nsIMemoryProfiler {
    vtable: *const nsIMemoryProfilerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMemoryProfiler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1e10e7a9, 0xbc05, 0x4878,
            [0xa6, 0x87, 0x36, 0xc9, 0xea, 0x44, 0x28, 0xb1])
    }
}

unsafe impl RefCounted for nsIMemoryProfiler {
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
pub trait nsIMemoryProfilerCoerce {
    fn coerce_from(v: &nsIMemoryProfiler) -> &Self;
}

impl nsIMemoryProfilerCoerce for nsIMemoryProfiler {
    #[inline]
    fn coerce_from(v: &nsIMemoryProfiler) -> &Self {
        v
    }
}

impl nsIMemoryProfiler {
    #[inline]
    pub fn coerce<T: nsIMemoryProfilerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMemoryProfiler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMemoryProfilerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMemoryProfiler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMemoryProfilerVTable {
    pub __base: nsISupportsVTable,

    /* void startProfiler (); */
    pub startProfiler: unsafe extern "C" fn (this: *const nsIMemoryProfiler) -> nsresult,

    /* void stopProfiler (); */
    pub stopProfiler: unsafe extern "C" fn (this: *const nsIMemoryProfiler) -> nsresult,

    /* void resetProfiler (); */
    pub resetProfiler: unsafe extern "C" fn (this: *const nsIMemoryProfiler) -> nsresult,

    /* [implicit_jscontext] jsval getResults (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getResults: *const ::libc::c_void,

}


impl nsIMemoryProfiler {
    /* void startProfiler (); */
    #[inline]
    pub unsafe fn startProfiler(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).startProfiler)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stopProfiler (); */
    #[inline]
    pub unsafe fn stopProfiler(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopProfiler)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resetProfiler (); */
    #[inline]
    pub unsafe fn resetProfiler(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resetProfiler)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] jsval getResults (); */


}


