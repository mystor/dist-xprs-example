//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIImageDocument.idl
//


#[repr(C)]
pub struct nsIImageDocument {
    vtable: *const nsIImageDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIImageDocument {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x87c27f98, 0x37dc, 0x4b64,
            [0xa8, 0xcd, 0x92, 0x00, 0x36, 0x24, 0xbc, 0xee])
    }
}

unsafe impl RefCounted for nsIImageDocument {
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
pub trait nsIImageDocumentCoerce {
    fn coerce_from(v: &nsIImageDocument) -> &Self;
}

impl nsIImageDocumentCoerce for nsIImageDocument {
    #[inline]
    fn coerce_from(v: &nsIImageDocument) -> &Self {
        v
    }
}

impl nsIImageDocument {
    #[inline]
    pub fn coerce<T: nsIImageDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIImageDocument {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIImageDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIImageDocument) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIImageDocumentVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean imageIsOverflowing; */
    pub get_imageIsOverflowing: unsafe extern "C" fn (this: *const nsIImageDocument, aImageIsOverflowing: *mut bool) -> nsresult,

    /* readonly attribute boolean imageIsResized; */
    pub get_imageIsResized: unsafe extern "C" fn (this: *const nsIImageDocument, aImageIsResized: *mut bool) -> nsresult,

    /* readonly attribute imgIRequest imageRequest; */
    pub get_imageRequest: unsafe extern "C" fn (this: *const nsIImageDocument, aImageRequest: *mut *const imgIRequest) -> nsresult,

    /* [binaryname(DOMShrinkToFit)] void shrinkToFit (); */
    pub DOMShrinkToFit: unsafe extern "C" fn (this: *const nsIImageDocument) -> nsresult,

    /* [binaryname(DOMRestoreImage)] void restoreImage (); */
    pub DOMRestoreImage: unsafe extern "C" fn (this: *const nsIImageDocument) -> nsresult,

    /* [binaryname(DOMRestoreImageTo)] void restoreImageTo (in long x, in long y); */
    pub DOMRestoreImageTo: unsafe extern "C" fn (this: *const nsIImageDocument, x: libc::int32_t, y: libc::int32_t) -> nsresult,

    /* [binaryname(DOMToggleImageSize)] void toggleImageSize (); */
    pub DOMToggleImageSize: unsafe extern "C" fn (this: *const nsIImageDocument) -> nsresult,

}


impl nsIImageDocument {
    /* readonly attribute boolean imageIsOverflowing; */
    #[inline]
    pub unsafe fn get_imageIsOverflowing(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_imageIsOverflowing)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean imageIsResized; */
    #[inline]
    pub unsafe fn get_imageIsResized(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_imageIsResized)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute imgIRequest imageRequest; */
    #[inline]
    pub unsafe fn get_imageRequest(&self, ) -> Result<Option<RefPtr<imgIRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_imageRequest)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [binaryname(DOMShrinkToFit)] void shrinkToFit (); */
    #[inline]
    pub unsafe fn DOMShrinkToFit(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).DOMShrinkToFit)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [binaryname(DOMRestoreImage)] void restoreImage (); */
    #[inline]
    pub unsafe fn DOMRestoreImage(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).DOMRestoreImage)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [binaryname(DOMRestoreImageTo)] void restoreImageTo (in long x, in long y); */
    #[inline]
    pub unsafe fn DOMRestoreImageTo(&self, x: libc::int32_t, y: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).DOMRestoreImageTo)(self as *const _, x, y) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [binaryname(DOMToggleImageSize)] void toggleImageSize (); */
    #[inline]
    pub unsafe fn DOMToggleImageSize(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).DOMToggleImageSize)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


