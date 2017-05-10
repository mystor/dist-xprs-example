//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMStyleSheetList.idl
//


#[repr(C)]
pub struct nsIDOMStyleSheetList {
    vtable: *const nsIDOMStyleSheetListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMStyleSheetList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0e424250, 0xac2a, 0x4fe2,
            [0xbc, 0xcd, 0xa4, 0x58, 0x24, 0xaf, 0x09, 0x0e])
    }
}

unsafe impl RefCounted for nsIDOMStyleSheetList {
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
pub trait nsIDOMStyleSheetListCoerce {
    fn coerce_from(v: &nsIDOMStyleSheetList) -> &Self;
}

impl nsIDOMStyleSheetListCoerce for nsIDOMStyleSheetList {
    #[inline]
    fn coerce_from(v: &nsIDOMStyleSheetList) -> &Self {
        v
    }
}

impl nsIDOMStyleSheetList {
    #[inline]
    pub fn coerce<T: nsIDOMStyleSheetListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMStyleSheetList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMStyleSheetListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMStyleSheetList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMStyleSheetListVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMStyleSheetList, aLength: *mut libc::uint32_t) -> nsresult,

    /* [binaryname(SlowItem)] nsIDOMStyleSheet item (in unsigned long index); */
    pub SlowItem: unsafe extern "C" fn (this: *const nsIDOMStyleSheetList, index: libc::uint32_t, _retval: *mut *const nsIDOMStyleSheet) -> nsresult,

}


impl nsIDOMStyleSheetList {
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

    /* [binaryname(SlowItem)] nsIDOMStyleSheet item (in unsigned long index); */
    #[inline]
    pub unsafe fn SlowItem(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMStyleSheet>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).SlowItem)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


