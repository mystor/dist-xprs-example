//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDiskSpaceWatcher.idl
//


#[repr(C)]
pub struct nsIDiskSpaceWatcher {
    vtable: *const nsIDiskSpaceWatcherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDiskSpaceWatcher {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3aceba74, 0x2ed5, 0x4e99,
            [0x8f, 0xe4, 0x06, 0xe9, 0x0e, 0x2b, 0x8e, 0xf0])
    }
}

unsafe impl RefCounted for nsIDiskSpaceWatcher {
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
pub trait nsIDiskSpaceWatcherCoerce {
    fn coerce_from(v: &nsIDiskSpaceWatcher) -> &Self;
}

impl nsIDiskSpaceWatcherCoerce for nsIDiskSpaceWatcher {
    #[inline]
    fn coerce_from(v: &nsIDiskSpaceWatcher) -> &Self {
        v
    }
}

impl nsIDiskSpaceWatcher {
    #[inline]
    pub fn coerce<T: nsIDiskSpaceWatcherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDiskSpaceWatcher {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDiskSpaceWatcherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDiskSpaceWatcher) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDiskSpaceWatcherVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute bool isDiskFull; */
    pub get_isDiskFull: unsafe extern "C" fn (this: *const nsIDiskSpaceWatcher, aIsDiskFull: *mut bool) -> nsresult,

    /* readonly attribute unsigned long long freeSpace; */
    pub get_freeSpace: unsafe extern "C" fn (this: *const nsIDiskSpaceWatcher, aFreeSpace: *mut libc::uint64_t) -> nsresult,

}


impl nsIDiskSpaceWatcher {
    /* readonly attribute bool isDiskFull; */
    #[inline]
    pub unsafe fn get_isDiskFull(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDiskFull)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long freeSpace; */
    #[inline]
    pub unsafe fn get_freeSpace(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_freeSpace)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


