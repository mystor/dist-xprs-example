//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMemory.idl
//


#[repr(C)]
pub struct nsIMemory {
    vtable: *const nsIMemoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMemory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1e004834, 0x6d8f, 0x425a,
            [0xbc, 0x9c, 0xa2, 0x81, 0x2e, 0xd4, 0x3b, 0xb7])
    }
}

unsafe impl RefCounted for nsIMemory {
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
pub trait nsIMemoryCoerce {
    fn coerce_from(v: &nsIMemory) -> &Self;
}

impl nsIMemoryCoerce for nsIMemory {
    #[inline]
    fn coerce_from(v: &nsIMemory) -> &Self {
        v
    }
}

impl nsIMemory {
    #[inline]
    pub fn coerce<T: nsIMemoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMemory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMemoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMemory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMemoryVTable {
    pub __base: nsISupportsVTable,

    /* void heapMinimize (in boolean immediate); */
    pub heapMinimize: unsafe extern "C" fn (this: *const nsIMemory, immediate: bool) -> nsresult,

    /* boolean isLowMemoryPlatform (); */
    pub isLowMemoryPlatform: unsafe extern "C" fn (this: *const nsIMemory, _retval: *mut bool) -> nsresult,

}


impl nsIMemory {
    /* void heapMinimize (in boolean immediate); */
    #[inline]
    pub unsafe fn heapMinimize(&self, immediate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).heapMinimize)(self as *const _, immediate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isLowMemoryPlatform (); */
    #[inline]
    pub unsafe fn isLowMemoryPlatform(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isLowMemoryPlatform)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


