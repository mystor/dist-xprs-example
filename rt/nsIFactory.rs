//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFactory.idl
//


#[repr(C)]
pub struct nsIFactory {
    vtable: *const nsIFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFactory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x00000001, 0x0000, 0x0000,
            [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46])
    }
}

unsafe impl RefCounted for nsIFactory {
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
pub trait nsIFactoryCoerce {
    fn coerce_from(v: &nsIFactory) -> &Self;
}

impl nsIFactoryCoerce for nsIFactory {
    #[inline]
    fn coerce_from(v: &nsIFactory) -> &Self {
        v
    }
}

impl nsIFactory {
    #[inline]
    pub fn coerce<T: nsIFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFactory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFactory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFactoryVTable {
    pub __base: nsISupportsVTable,

    /* void createInstance (in nsISupports aOuter, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    pub createInstance: unsafe extern "C" fn (this: *const nsIFactory, aOuter: *const nsISupports, iid: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* void lockFactory (in boolean lock); */
    pub lockFactory: unsafe extern "C" fn (this: *const nsIFactory, lock: bool) -> nsresult,

}


impl nsIFactory {
    /* void createInstance (in nsISupports aOuter, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn createInstance<T: XpCom>(&self, aOuter: Option<&nsISupports>) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).createInstance)(self as *const _, aOuter.map_or(::std::ptr::null(), |x| x as *const _), &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* void lockFactory (in boolean lock); */
    #[inline]
    pub unsafe fn lockFactory(&self, lock: bool) -> Result<(), nsresult> {

        match ((*self.vtable).lockFactory)(self as *const _, lock) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


