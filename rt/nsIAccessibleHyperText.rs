//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleHyperText.idl
//


#[repr(C)]
pub struct nsIAccessibleHyperText {
    vtable: *const nsIAccessibleHyperTextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleHyperText {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb33684e2, 0x090c, 0x4e1d,
            [0xa3, 0xd9, 0xf4, 0xb4, 0x6f, 0x42, 0x37, 0xb9])
    }
}

unsafe impl RefCounted for nsIAccessibleHyperText {
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
pub trait nsIAccessibleHyperTextCoerce {
    fn coerce_from(v: &nsIAccessibleHyperText) -> &Self;
}

impl nsIAccessibleHyperTextCoerce for nsIAccessibleHyperText {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHyperText) -> &Self {
        v
    }
}

impl nsIAccessibleHyperText {
    #[inline]
    pub fn coerce<T: nsIAccessibleHyperTextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleHyperText {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleHyperTextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHyperText) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleHyperTextVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long linkCount; */
    pub get_linkCount: unsafe extern "C" fn (this: *const nsIAccessibleHyperText, aLinkCount: *mut libc::int32_t) -> nsresult,

    /* nsIAccessibleHyperLink getLinkAt (in long index); */
    pub getLinkAt: unsafe extern "C" fn (this: *const nsIAccessibleHyperText, index: libc::int32_t, _retval: *mut *const nsIAccessibleHyperLink) -> nsresult,

    /* long getLinkIndex (in nsIAccessibleHyperLink link); */
    pub getLinkIndex: unsafe extern "C" fn (this: *const nsIAccessibleHyperText, link: *const nsIAccessibleHyperLink, _retval: *mut libc::int32_t) -> nsresult,

    /* long getLinkIndexAtOffset (in long offset); */
    pub getLinkIndexAtOffset: unsafe extern "C" fn (this: *const nsIAccessibleHyperText, offset: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

}


impl nsIAccessibleHyperText {
    /* readonly attribute long linkCount; */
    #[inline]
    pub unsafe fn get_linkCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_linkCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIAccessibleHyperLink getLinkAt (in long index); */
    #[inline]
    pub unsafe fn getLinkAt(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIAccessibleHyperLink>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getLinkAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* long getLinkIndex (in nsIAccessibleHyperLink link); */
    #[inline]
    pub unsafe fn getLinkIndex(&self, link: Option<&nsIAccessibleHyperLink>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getLinkIndex)(self as *const _, link.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getLinkIndexAtOffset (in long offset); */
    #[inline]
    pub unsafe fn getLinkIndexAtOffset(&self, offset: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getLinkIndexAtOffset)(self as *const _, offset, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


