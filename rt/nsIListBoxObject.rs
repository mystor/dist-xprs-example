//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIListBoxObject.idl
//


#[repr(C)]
pub struct nsIListBoxObject {
    vtable: *const nsIListBoxObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIListBoxObject {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaa9def4e, 0x2e59, 0x412d,
            [0xa6, 0xdf, 0xb7, 0x6f, 0x52, 0x16, 0x77, 0x95])
    }
}

unsafe impl RefCounted for nsIListBoxObject {
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
pub trait nsIListBoxObjectCoerce {
    fn coerce_from(v: &nsIListBoxObject) -> &Self;
}

impl nsIListBoxObjectCoerce for nsIListBoxObject {
    #[inline]
    fn coerce_from(v: &nsIListBoxObject) -> &Self {
        v
    }
}

impl nsIListBoxObject {
    #[inline]
    pub fn coerce<T: nsIListBoxObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIListBoxObject {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIListBoxObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIListBoxObject) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIListBoxObjectVTable {
    pub __base: nsISupportsVTable,

    /* long getRowCount (); */
    pub getRowCount: unsafe extern "C" fn (this: *const nsIListBoxObject, _retval: *mut libc::int32_t) -> nsresult,

    /* nsIDOMElement getItemAtIndex (in long index); */
    pub getItemAtIndex: unsafe extern "C" fn (this: *const nsIListBoxObject, index: libc::int32_t, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* long getIndexOfItem (in nsIDOMElement item); */
    pub getIndexOfItem: unsafe extern "C" fn (this: *const nsIListBoxObject, item: *const nsIDOMElement, _retval: *mut libc::int32_t) -> nsresult,

}


impl nsIListBoxObject {
    /* long getRowCount (); */
    #[inline]
    pub unsafe fn getRowCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRowCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMElement getItemAtIndex (in long index); */
    #[inline]
    pub unsafe fn getItemAtIndex(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getItemAtIndex)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* long getIndexOfItem (in nsIDOMElement item); */
    #[inline]
    pub unsafe fn getIndexOfItem(&self, item: Option<&nsIDOMElement>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getIndexOfItem)(self as *const _, item.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


