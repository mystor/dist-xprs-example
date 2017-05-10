//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITabSource.idl
//


#[repr(C)]
pub struct nsITabSource {
    vtable: *const nsITabSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITabSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0feba7f2, 0x800d, 0x4fe5,
            [0xb2, 0x8d, 0xe3, 0xf1, 0x7a, 0x7a, 0x73, 0x22])
    }
}

unsafe impl RefCounted for nsITabSource {
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
pub trait nsITabSourceCoerce {
    fn coerce_from(v: &nsITabSource) -> &Self;
}

impl nsITabSourceCoerce for nsITabSource {
    #[inline]
    fn coerce_from(v: &nsITabSource) -> &Self {
        v
    }
}

impl nsITabSource {
    #[inline]
    pub fn coerce<T: nsITabSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITabSource {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITabSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITabSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITabSourceVTable {
    pub __base: nsISupportsVTable,

    /* mozIDOMWindowProxy getTabToStream (); */
    pub getTabToStream: unsafe extern "C" fn (this: *const nsITabSource, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* void notifyStreamStart (in mozIDOMWindowProxy window); */
    pub notifyStreamStart: unsafe extern "C" fn (this: *const nsITabSource, window: *const mozIDOMWindowProxy) -> nsresult,

    /* void notifyStreamStop (in mozIDOMWindowProxy window); */
    pub notifyStreamStop: unsafe extern "C" fn (this: *const nsITabSource, window: *const mozIDOMWindowProxy) -> nsresult,

}


impl nsITabSource {
    /* mozIDOMWindowProxy getTabToStream (); */
    #[inline]
    pub unsafe fn getTabToStream(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getTabToStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void notifyStreamStart (in mozIDOMWindowProxy window); */
    #[inline]
    pub unsafe fn notifyStreamStart(&self, window: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).notifyStreamStart)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyStreamStop (in mozIDOMWindowProxy window); */
    #[inline]
    pub unsafe fn notifyStreamStop(&self, window: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).notifyStreamStop)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


