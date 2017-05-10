//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowDataSource.idl
//


#[repr(C)]
pub struct nsIWindowDataSource {
    vtable: *const nsIWindowDataSourceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWindowDataSource {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3722a5b9, 0x5323, 0x4ed0,
            [0xbb, 0x1a, 0x82, 0x99, 0xf2, 0x7a, 0x4e, 0x89])
    }
}

unsafe impl RefCounted for nsIWindowDataSource {
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
pub trait nsIWindowDataSourceCoerce {
    fn coerce_from(v: &nsIWindowDataSource) -> &Self;
}

impl nsIWindowDataSourceCoerce for nsIWindowDataSource {
    #[inline]
    fn coerce_from(v: &nsIWindowDataSource) -> &Self {
        v
    }
}

impl nsIWindowDataSource {
    #[inline]
    pub fn coerce<T: nsIWindowDataSourceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWindowDataSource {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWindowDataSourceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowDataSource) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWindowDataSourceVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMWindow getWindowForResource (in string inResource); */
    pub getWindowForResource: unsafe extern "C" fn (this: *const nsIWindowDataSource, inResource: *const libc::c_char, _retval: *mut *const nsIDOMWindow) -> nsresult,

}


impl nsIWindowDataSource {
    /* nsIDOMWindow getWindowForResource (in string inResource); */
    #[inline]
    pub unsafe fn getWindowForResource(&self, inResource: *const libc::c_char) -> Result<Option<RefPtr<nsIDOMWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getWindowForResource)(self as *const _, inResource, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


