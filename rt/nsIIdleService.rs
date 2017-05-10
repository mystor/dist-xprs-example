//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIdleService.idl
//


#[repr(C)]
pub struct nsIIdleService {
    vtable: *const nsIIdleServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIdleService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcc52f19a, 0x63ae, 0x4a1c,
            [0x9c, 0xc3, 0xe7, 0x9e, 0xac, 0xe0, 0xb4, 0x71])
    }
}

unsafe impl RefCounted for nsIIdleService {
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
pub trait nsIIdleServiceCoerce {
    fn coerce_from(v: &nsIIdleService) -> &Self;
}

impl nsIIdleServiceCoerce for nsIIdleService {
    #[inline]
    fn coerce_from(v: &nsIIdleService) -> &Self {
        v
    }
}

impl nsIIdleService {
    #[inline]
    pub fn coerce<T: nsIIdleServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIdleService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIdleServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdleService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIdleServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long idleTime; */
    pub get_idleTime: unsafe extern "C" fn (this: *const nsIIdleService, aIdleTime: *mut libc::uint32_t) -> nsresult,

    /* void addIdleObserver (in nsIObserver observer, in unsigned long time); */
    pub addIdleObserver: unsafe extern "C" fn (this: *const nsIIdleService, observer: *const nsIObserver, time: libc::uint32_t) -> nsresult,

    /* void removeIdleObserver (in nsIObserver observer, in unsigned long time); */
    pub removeIdleObserver: unsafe extern "C" fn (this: *const nsIIdleService, observer: *const nsIObserver, time: libc::uint32_t) -> nsresult,

}


impl nsIIdleService {
    /* readonly attribute unsigned long idleTime; */
    #[inline]
    pub unsafe fn get_idleTime(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_idleTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addIdleObserver (in nsIObserver observer, in unsigned long time); */
    #[inline]
    pub unsafe fn addIdleObserver(&self, observer: Option<&nsIObserver>, time: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).addIdleObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _), time) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeIdleObserver (in nsIObserver observer, in unsigned long time); */
    #[inline]
    pub unsafe fn removeIdleObserver(&self, observer: Option<&nsIObserver>, time: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeIdleObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _), time) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


