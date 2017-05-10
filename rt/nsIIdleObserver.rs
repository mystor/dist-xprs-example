//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIdleObserver.idl
//


#[repr(C)]
pub struct nsIIdleObserver {
    vtable: *const nsIIdleObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIdleObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x37916e05, 0xe062, 0x4f72,
            [0x96, 0xd5, 0x66, 0x0c, 0xfb, 0x55, 0xe9, 0xb6])
    }
}

unsafe impl RefCounted for nsIIdleObserver {
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
pub trait nsIIdleObserverCoerce {
    fn coerce_from(v: &nsIIdleObserver) -> &Self;
}

impl nsIIdleObserverCoerce for nsIIdleObserver {
    #[inline]
    fn coerce_from(v: &nsIIdleObserver) -> &Self {
        v
    }
}

impl nsIIdleObserver {
    #[inline]
    pub fn coerce<T: nsIIdleObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIdleObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIdleObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdleObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIdleObserverVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long time; */
    pub get_time: unsafe extern "C" fn (this: *const nsIIdleObserver, aTime: *mut libc::uint32_t) -> nsresult,

    /* void onidle (); */
    pub onidle: unsafe extern "C" fn (this: *const nsIIdleObserver) -> nsresult,

    /* void onactive (); */
    pub onactive: unsafe extern "C" fn (this: *const nsIIdleObserver) -> nsresult,

}


impl nsIIdleObserver {
    /* readonly attribute unsigned long time; */
    #[inline]
    pub unsafe fn get_time(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_time)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void onidle (); */
    #[inline]
    pub unsafe fn onidle(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onidle)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onactive (); */
    #[inline]
    pub unsafe fn onactive(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onactive)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


