//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIArrayExtensions.idl
//


#[repr(C)]
pub struct nsIArrayExtensions {
    vtable: *const nsIArrayExtensionsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIArrayExtensions {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x261d442e, 0x050c, 0x453d,
            [0x8a, 0xaa, 0xb3, 0xf2, 0x3b, 0xcc, 0x52, 0x8b])
    }
}

unsafe impl RefCounted for nsIArrayExtensions {
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
pub trait nsIArrayExtensionsCoerce {
    fn coerce_from(v: &nsIArrayExtensions) -> &Self;
}

impl nsIArrayExtensionsCoerce for nsIArrayExtensions {
    #[inline]
    fn coerce_from(v: &nsIArrayExtensions) -> &Self {
        v
    }
}

impl nsIArrayExtensions {
    #[inline]
    pub fn coerce<T: nsIArrayExtensionsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIArrayExtensions {
    type Target = nsIArray;
    #[inline]
    fn deref(&self) -> &nsIArray {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIArrayCoerce> nsIArrayExtensionsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIArrayExtensions) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIArrayExtensionsVTable {
    pub __base: nsIArrayVTable,

    /* uint32_t Count (); */
    pub Count: unsafe extern "C" fn (this: *const nsIArrayExtensions, _retval: *mut uint32_t) -> nsresult,

    /* nsISupports GetElementAt (in uint32_t index); */
    pub GetElementAt: unsafe extern "C" fn (this: *const nsIArrayExtensions, index: uint32_t, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsIArrayExtensions {
    /* uint32_t Count (); */
    #[inline]
    pub unsafe fn Count(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).Count)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports GetElementAt (in uint32_t index); */
    #[inline]
    pub unsafe fn GetElementAt(&self, index: uint32_t) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetElementAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


