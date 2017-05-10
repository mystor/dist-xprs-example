//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleHyperLink.idl
//


#[repr(C)]
pub struct nsIAccessibleHyperLink {
    vtable: *const nsIAccessibleHyperLinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleHyperLink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x883643d4, 0x93a5, 0x4f32,
            [0x92, 0x2c, 0x6f, 0x06, 0xe0, 0x13, 0x63, 0xc1])
    }
}

unsafe impl RefCounted for nsIAccessibleHyperLink {
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
pub trait nsIAccessibleHyperLinkCoerce {
    fn coerce_from(v: &nsIAccessibleHyperLink) -> &Self;
}

impl nsIAccessibleHyperLinkCoerce for nsIAccessibleHyperLink {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHyperLink) -> &Self {
        v
    }
}

impl nsIAccessibleHyperLink {
    #[inline]
    pub fn coerce<T: nsIAccessibleHyperLinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleHyperLink {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleHyperLinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHyperLink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleHyperLinkVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long startIndex; */
    pub get_startIndex: unsafe extern "C" fn (this: *const nsIAccessibleHyperLink, aStartIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long endIndex; */
    pub get_endIndex: unsafe extern "C" fn (this: *const nsIAccessibleHyperLink, aEndIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean valid; */
    pub get_valid: unsafe extern "C" fn (this: *const nsIAccessibleHyperLink, aValid: *mut bool) -> nsresult,

    /* readonly attribute long anchorCount; */
    pub get_anchorCount: unsafe extern "C" fn (this: *const nsIAccessibleHyperLink, aAnchorCount: *mut libc::int32_t) -> nsresult,

    /* nsIURI getURI (in long index); */
    pub getURI: unsafe extern "C" fn (this: *const nsIAccessibleHyperLink, index: libc::int32_t, _retval: *mut *const nsIURI) -> nsresult,

    /* nsIAccessible getAnchor (in long index); */
    pub getAnchor: unsafe extern "C" fn (this: *const nsIAccessibleHyperLink, index: libc::int32_t, _retval: *mut *const nsIAccessible) -> nsresult,

}


impl nsIAccessibleHyperLink {
    /* readonly attribute long startIndex; */
    #[inline]
    pub unsafe fn get_startIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_startIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long endIndex; */
    #[inline]
    pub unsafe fn get_endIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_endIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean valid; */
    #[inline]
    pub unsafe fn get_valid(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_valid)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long anchorCount; */
    #[inline]
    pub unsafe fn get_anchorCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_anchorCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIURI getURI (in long index); */
    #[inline]
    pub unsafe fn getURI(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getURI)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAccessible getAnchor (in long index); */
    #[inline]
    pub unsafe fn getAnchor(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAnchor)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


