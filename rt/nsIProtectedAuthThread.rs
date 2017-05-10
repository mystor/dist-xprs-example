//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtectedAuthThread.idl
//


#[repr(C)]
pub struct nsIProtectedAuthThread {
    vtable: *const nsIProtectedAuthThreadVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProtectedAuthThread {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4bb27cb7, 0x8984, 0x4cee,
            [0x8c, 0xe7, 0x9b, 0x01, 0x4c, 0x3d, 0x09, 0x1b])
    }
}

unsafe impl RefCounted for nsIProtectedAuthThread {
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
pub trait nsIProtectedAuthThreadCoerce {
    fn coerce_from(v: &nsIProtectedAuthThread) -> &Self;
}

impl nsIProtectedAuthThreadCoerce for nsIProtectedAuthThread {
    #[inline]
    fn coerce_from(v: &nsIProtectedAuthThread) -> &Self {
        v
    }
}

impl nsIProtectedAuthThread {
    #[inline]
    pub fn coerce<T: nsIProtectedAuthThreadCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProtectedAuthThread {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProtectedAuthThreadCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtectedAuthThread) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProtectedAuthThreadVTable {
    pub __base: nsISupportsVTable,

    /* void login (in nsIObserver observer); */
    pub login: unsafe extern "C" fn (this: *const nsIProtectedAuthThread, observer: *const nsIObserver) -> nsresult,

    /* readonly attribute nsIPKCS11Slot slot; */
    pub get_slot: unsafe extern "C" fn (this: *const nsIProtectedAuthThread, aSlot: *mut *const nsIPKCS11Slot) -> nsresult,

    /* AString getTokenName (); */
    pub getTokenName: unsafe extern "C" fn (this: *const nsIProtectedAuthThread, _retval: *mut nsAString) -> nsresult,

}


impl nsIProtectedAuthThread {
    /* void login (in nsIObserver observer); */
    #[inline]
    pub unsafe fn login(&self, observer: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).login)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIPKCS11Slot slot; */
    #[inline]
    pub unsafe fn get_slot(&self, ) -> Result<Option<RefPtr<nsIPKCS11Slot>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_slot)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AString getTokenName (); */
    #[inline]
    pub unsafe fn getTokenName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getTokenName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


