//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleDocument.idl
//


#[repr(C)]
pub struct nsIAccessibleDocument {
    vtable: *const nsIAccessibleDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleDocument {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5cad5f91, 0xfcce, 0x40e7,
            [0x91, 0x3e, 0x46, 0x71, 0x70, 0x1d, 0x19, 0xb4])
    }
}

unsafe impl RefCounted for nsIAccessibleDocument {
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
pub trait nsIAccessibleDocumentCoerce {
    fn coerce_from(v: &nsIAccessibleDocument) -> &Self;
}

impl nsIAccessibleDocumentCoerce for nsIAccessibleDocument {
    #[inline]
    fn coerce_from(v: &nsIAccessibleDocument) -> &Self {
        v
    }
}

impl nsIAccessibleDocument {
    #[inline]
    pub fn coerce<T: nsIAccessibleDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleDocument {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleDocument) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleDocumentVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString URL; */
    pub get_URL: unsafe extern "C" fn (this: *const nsIAccessibleDocument, aURL: *mut nsAString) -> nsresult,

    /* readonly attribute AString title; */
    pub get_title: unsafe extern "C" fn (this: *const nsIAccessibleDocument, aTitle: *mut nsAString) -> nsresult,

    /* readonly attribute AString mimeType; */
    pub get_mimeType: unsafe extern "C" fn (this: *const nsIAccessibleDocument, aMimeType: *mut nsAString) -> nsresult,

    /* readonly attribute AString docType; */
    pub get_docType: unsafe extern "C" fn (this: *const nsIAccessibleDocument, aDocType: *mut nsAString) -> nsresult,

    /* readonly attribute nsIDOMDocument DOMDocument; */
    pub get_DOMDocument: unsafe extern "C" fn (this: *const nsIAccessibleDocument, aDOMDocument: *mut *const nsIDOMDocument) -> nsresult,

    /* readonly attribute mozIDOMWindowProxy window; */
    pub get_window: unsafe extern "C" fn (this: *const nsIAccessibleDocument, aWindow: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute nsIAccessibleDocument parentDocument; */
    pub get_parentDocument: unsafe extern "C" fn (this: *const nsIAccessibleDocument, aParentDocument: *mut *const nsIAccessibleDocument) -> nsresult,

    /* readonly attribute unsigned long childDocumentCount; */
    pub get_childDocumentCount: unsafe extern "C" fn (this: *const nsIAccessibleDocument, aChildDocumentCount: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIAccessiblePivot virtualCursor; */
    pub get_virtualCursor: unsafe extern "C" fn (this: *const nsIAccessibleDocument, aVirtualCursor: *mut *const nsIAccessiblePivot) -> nsresult,

    /* nsIAccessibleDocument getChildDocumentAt (in unsigned long index); */
    pub getChildDocumentAt: unsafe extern "C" fn (this: *const nsIAccessibleDocument, index: libc::uint32_t, _retval: *mut *const nsIAccessibleDocument) -> nsresult,

}


impl nsIAccessibleDocument {
    /* readonly attribute AString URL; */
    #[inline]
    pub unsafe fn get_URL(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_URL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_title)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString mimeType; */
    #[inline]
    pub unsafe fn get_mimeType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_mimeType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString docType; */
    #[inline]
    pub unsafe fn get_docType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_docType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMDocument DOMDocument; */
    #[inline]
    pub unsafe fn get_DOMDocument(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_DOMDocument)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute mozIDOMWindowProxy window; */
    #[inline]
    pub unsafe fn get_window(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_window)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAccessibleDocument parentDocument; */
    #[inline]
    pub unsafe fn get_parentDocument(&self, ) -> Result<Option<RefPtr<nsIAccessibleDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentDocument)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long childDocumentCount; */
    #[inline]
    pub unsafe fn get_childDocumentCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_childDocumentCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIAccessiblePivot virtualCursor; */
    #[inline]
    pub unsafe fn get_virtualCursor(&self, ) -> Result<Option<RefPtr<nsIAccessiblePivot>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_virtualCursor)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAccessibleDocument getChildDocumentAt (in unsigned long index); */
    #[inline]
    pub unsafe fn getChildDocumentAt(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIAccessibleDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChildDocumentAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


