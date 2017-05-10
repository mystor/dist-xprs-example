//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoadGroupChild.idl
//


#[repr(C)]
pub struct nsILoadGroupChild {
    vtable: *const nsILoadGroupChildVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoadGroupChild {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x02efe8e2, 0xfbbc, 0x4718,
            [0xa2, 0x99, 0xb8, 0xa0, 0x9c, 0x60, 0xbf, 0x6b])
    }
}

unsafe impl RefCounted for nsILoadGroupChild {
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
pub trait nsILoadGroupChildCoerce {
    fn coerce_from(v: &nsILoadGroupChild) -> &Self;
}

impl nsILoadGroupChildCoerce for nsILoadGroupChild {
    #[inline]
    fn coerce_from(v: &nsILoadGroupChild) -> &Self {
        v
    }
}

impl nsILoadGroupChild {
    #[inline]
    pub fn coerce<T: nsILoadGroupChildCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoadGroupChild {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoadGroupChildCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadGroupChild) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoadGroupChildVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsILoadGroup parentLoadGroup; */
    pub get_parentLoadGroup: unsafe extern "C" fn (this: *const nsILoadGroupChild, aParentLoadGroup: *mut *const nsILoadGroup) -> nsresult,
    pub set_parentLoadGroup: unsafe extern "C" fn (this: *const nsILoadGroupChild, aParentLoadGroup: *const nsILoadGroup) -> nsresult,

    /* readonly attribute nsILoadGroup childLoadGroup; */
    pub get_childLoadGroup: unsafe extern "C" fn (this: *const nsILoadGroupChild, aChildLoadGroup: *mut *const nsILoadGroup) -> nsresult,

    /* readonly attribute nsILoadGroup rootLoadGroup; */
    pub get_rootLoadGroup: unsafe extern "C" fn (this: *const nsILoadGroupChild, aRootLoadGroup: *mut *const nsILoadGroup) -> nsresult,

}


impl nsILoadGroupChild {
    /* attribute nsILoadGroup parentLoadGroup; */
    #[inline]
    pub unsafe fn get_parentLoadGroup(&self, ) -> Result<Option<RefPtr<nsILoadGroup>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentLoadGroup)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_parentLoadGroup(&self, aParentLoadGroup: Option<&nsILoadGroup>) -> Result<(), nsresult> {

        match ((*self.vtable).set_parentLoadGroup)(self as *const _, aParentLoadGroup.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsILoadGroup childLoadGroup; */
    #[inline]
    pub unsafe fn get_childLoadGroup(&self, ) -> Result<Option<RefPtr<nsILoadGroup>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_childLoadGroup)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsILoadGroup rootLoadGroup; */
    #[inline]
    pub unsafe fn get_rootLoadGroup(&self, ) -> Result<Option<RefPtr<nsILoadGroup>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rootLoadGroup)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


