//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITreeContentView.idl
//


#[repr(C)]
pub struct nsITreeContentView {
    vtable: *const nsITreeContentViewVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITreeContentView {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5ef62896, 0x0c0a, 0x41f1,
            [0xbb, 0x3c, 0x44, 0xa6, 0x0f, 0x5d, 0xfd, 0xab])
    }
}

unsafe impl RefCounted for nsITreeContentView {
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
pub trait nsITreeContentViewCoerce {
    fn coerce_from(v: &nsITreeContentView) -> &Self;
}

impl nsITreeContentViewCoerce for nsITreeContentView {
    #[inline]
    fn coerce_from(v: &nsITreeContentView) -> &Self {
        v
    }
}

impl nsITreeContentView {
    #[inline]
    pub fn coerce<T: nsITreeContentViewCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITreeContentView {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITreeContentViewCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITreeContentView) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITreeContentViewVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMElement getItemAtIndex (in long index); */
    pub getItemAtIndex: unsafe extern "C" fn (this: *const nsITreeContentView, index: libc::int32_t, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* long getIndexOfItem (in nsIDOMElement item); */
    pub getIndexOfItem: unsafe extern "C" fn (this: *const nsITreeContentView, item: *const nsIDOMElement, _retval: *mut libc::int32_t) -> nsresult,

}


impl nsITreeContentView {
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


