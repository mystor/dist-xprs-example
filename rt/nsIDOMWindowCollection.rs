//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMWindowCollection.idl
//


#[repr(C)]
pub struct nsIDOMWindowCollection {
    vtable: *const nsIDOMWindowCollectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMWindowCollection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8d64f457, 0xfb8c, 0x49ea,
            [0xa3, 0x59, 0xce, 0xf3, 0x0e, 0xed, 0x97, 0x74])
    }
}

unsafe impl RefCounted for nsIDOMWindowCollection {
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
pub trait nsIDOMWindowCollectionCoerce {
    fn coerce_from(v: &nsIDOMWindowCollection) -> &Self;
}

impl nsIDOMWindowCollectionCoerce for nsIDOMWindowCollection {
    #[inline]
    fn coerce_from(v: &nsIDOMWindowCollection) -> &Self {
        v
    }
}

impl nsIDOMWindowCollection {
    #[inline]
    pub fn coerce<T: nsIDOMWindowCollectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMWindowCollection {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMWindowCollectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMWindowCollection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMWindowCollectionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMWindowCollection, aLength: *mut libc::uint32_t) -> nsresult,

    /* mozIDOMWindowProxy item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMWindowCollection, index: libc::uint32_t, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* mozIDOMWindowProxy namedItem (in DOMString name); */
    pub namedItem: unsafe extern "C" fn (this: *const nsIDOMWindowCollection, name: *const nsAString, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

}


impl nsIDOMWindowCollection {
    /* readonly attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* mozIDOMWindowProxy item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).item)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIDOMWindowProxy namedItem (in DOMString name); */
    #[inline]
    pub unsafe fn namedItem(&self, name: &[u16]) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).namedItem)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


