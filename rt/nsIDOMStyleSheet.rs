//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMStyleSheet.idl
//


#[repr(C)]
pub struct nsIDOMStyleSheet {
    vtable: *const nsIDOMStyleSheetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMStyleSheet {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa6cf9080, 0x15b3, 0x11d2,
            [0x93, 0x2e, 0x00, 0x80, 0x5f, 0x8a, 0xdd, 0x32])
    }
}

unsafe impl RefCounted for nsIDOMStyleSheet {
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
pub trait nsIDOMStyleSheetCoerce {
    fn coerce_from(v: &nsIDOMStyleSheet) -> &Self;
}

impl nsIDOMStyleSheetCoerce for nsIDOMStyleSheet {
    #[inline]
    fn coerce_from(v: &nsIDOMStyleSheet) -> &Self {
        v
    }
}

impl nsIDOMStyleSheet {
    #[inline]
    pub fn coerce<T: nsIDOMStyleSheetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMStyleSheet {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMStyleSheetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMStyleSheet) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMStyleSheetVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMStyleSheet, aType: *mut nsAString) -> nsresult,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMStyleSheet, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMStyleSheet, aDisabled: bool) -> nsresult,

    /* readonly attribute nsIDOMNode ownerNode; */
    pub get_ownerNode: unsafe extern "C" fn (this: *const nsIDOMStyleSheet, aOwnerNode: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute nsIDOMStyleSheet parentStyleSheet; */
    pub get_parentStyleSheet: unsafe extern "C" fn (this: *const nsIDOMStyleSheet, aParentStyleSheet: *mut *const nsIDOMStyleSheet) -> nsresult,

    /* readonly attribute DOMString href; */
    pub get_href: unsafe extern "C" fn (this: *const nsIDOMStyleSheet, aHref: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString title; */
    pub get_title: unsafe extern "C" fn (this: *const nsIDOMStyleSheet, aTitle: *mut nsAString) -> nsresult,

    /* readonly attribute nsIDOMMediaList media; */
    pub get_media: unsafe extern "C" fn (this: *const nsIDOMStyleSheet, aMedia: *mut *const nsIDOMMediaList) -> nsresult,

}


impl nsIDOMStyleSheet {
    /* readonly attribute DOMString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean disabled; */
    #[inline]
    pub unsafe fn get_disabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_disabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_disabled(&self, aDisabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_disabled)(self as *const _, aDisabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMNode ownerNode; */
    #[inline]
    pub unsafe fn get_ownerNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ownerNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMStyleSheet parentStyleSheet; */
    #[inline]
    pub unsafe fn get_parentStyleSheet(&self, ) -> Result<Option<RefPtr<nsIDOMStyleSheet>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentStyleSheet)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString href; */
    #[inline]
    pub unsafe fn get_href(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_href)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_title)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMMediaList media; */
    #[inline]
    pub unsafe fn get_media(&self, ) -> Result<Option<RefPtr<nsIDOMMediaList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_media)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


