//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIKeygenThread.idl
//


#[repr(C)]
pub struct nsIKeygenThread {
    vtable: *const nsIKeygenThreadVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIKeygenThread {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8712a243, 0x5539, 0x447c,
            [0x9f, 0x47, 0x86, 0x53, 0xf4, 0x0c, 0x3a, 0x09])
    }
}

unsafe impl RefCounted for nsIKeygenThread {
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
pub trait nsIKeygenThreadCoerce {
    fn coerce_from(v: &nsIKeygenThread) -> &Self;
}

impl nsIKeygenThreadCoerce for nsIKeygenThread {
    #[inline]
    fn coerce_from(v: &nsIKeygenThread) -> &Self {
        v
    }
}

impl nsIKeygenThread {
    #[inline]
    pub fn coerce<T: nsIKeygenThreadCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIKeygenThread {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIKeygenThreadCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeygenThread) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIKeygenThreadVTable {
    pub __base: nsISupportsVTable,

    /* void startKeyGeneration (in nsIObserver observer); */
    pub startKeyGeneration: unsafe extern "C" fn (this: *const nsIKeygenThread, observer: *const nsIObserver) -> nsresult,

    /* void userCanceled (out boolean threadAlreadyClosedDialog); */
    pub userCanceled: unsafe extern "C" fn (this: *const nsIKeygenThread, threadAlreadyClosedDialog: *mut bool) -> nsresult,

}


impl nsIKeygenThread {
    /* void startKeyGeneration (in nsIObserver observer); */
    #[inline]
    pub unsafe fn startKeyGeneration(&self, observer: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).startKeyGeneration)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void userCanceled (out boolean threadAlreadyClosedDialog); */
    #[inline]
    pub unsafe fn userCanceled(&self, ) -> Result<bool, nsresult> {
        let mut threadAlreadyClosedDialog: bool = ::std::mem::zeroed();
        match ((*self.vtable).userCanceled)(self as *const _, &mut threadAlreadyClosedDialog as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(threadAlreadyClosedDialog)
    }

}


