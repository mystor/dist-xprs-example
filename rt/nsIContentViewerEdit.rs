//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentViewerEdit.idl
//


pub mod nsIContentViewerEdit_consts {
    pub const COPY_IMAGE_TEXT: i64 = 1;
    pub const COPY_IMAGE_HTML: i64 = 2;
    pub const COPY_IMAGE_DATA: i64 = 4;
    pub const COPY_IMAGE_ALL: i64 = -1;
}


#[repr(C)]
pub struct nsIContentViewerEdit {
    vtable: *const nsIContentViewerEditVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentViewerEdit {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x35be2d7e, 0xf29b, 0x48ec,
            [0xbf, 0x7e, 0x80, 0xa3, 0x0a, 0x72, 0x4d, 0xe3])
    }
}

unsafe impl RefCounted for nsIContentViewerEdit {
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
pub trait nsIContentViewerEditCoerce {
    fn coerce_from(v: &nsIContentViewerEdit) -> &Self;
}

impl nsIContentViewerEditCoerce for nsIContentViewerEdit {
    #[inline]
    fn coerce_from(v: &nsIContentViewerEdit) -> &Self {
        v
    }
}

impl nsIContentViewerEdit {
    #[inline]
    pub fn coerce<T: nsIContentViewerEditCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentViewerEdit {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentViewerEditCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentViewerEdit) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentViewerEditVTable {
    pub __base: nsISupportsVTable,

    /* void clearSelection (); */
    pub clearSelection: unsafe extern "C" fn (this: *const nsIContentViewerEdit) -> nsresult,

    /* void selectAll (); */
    pub selectAll: unsafe extern "C" fn (this: *const nsIContentViewerEdit) -> nsresult,

    /* void copySelection (); */
    pub copySelection: unsafe extern "C" fn (this: *const nsIContentViewerEdit) -> nsresult,

    /* readonly attribute boolean copyable; */
    pub get_copyable: unsafe extern "C" fn (this: *const nsIContentViewerEdit, aCopyable: *mut bool) -> nsresult,

    /* void copyLinkLocation (); */
    pub copyLinkLocation: unsafe extern "C" fn (this: *const nsIContentViewerEdit) -> nsresult,

    /* readonly attribute boolean inLink; */
    pub get_inLink: unsafe extern "C" fn (this: *const nsIContentViewerEdit, aInLink: *mut bool) -> nsresult,

    /* void copyImage (in long aCopyFlags); */
    pub copyImage: unsafe extern "C" fn (this: *const nsIContentViewerEdit, aCopyFlags: libc::int32_t) -> nsresult,

    /* readonly attribute boolean inImage; */
    pub get_inImage: unsafe extern "C" fn (this: *const nsIContentViewerEdit, aInImage: *mut bool) -> nsresult,

    /* AString getContents (in string aMimeType, in boolean aSelectionOnly); */
    pub getContents: unsafe extern "C" fn (this: *const nsIContentViewerEdit, aMimeType: *const libc::c_char, aSelectionOnly: bool, _retval: *mut nsAString) -> nsresult,

    /* readonly attribute boolean canGetContents; */
    pub get_canGetContents: unsafe extern "C" fn (this: *const nsIContentViewerEdit, aCanGetContents: *mut bool) -> nsresult,

    /* void setCommandNode (in nsIDOMNode aNode); */
    pub setCommandNode: unsafe extern "C" fn (this: *const nsIContentViewerEdit, aNode: *const nsIDOMNode) -> nsresult,

}


impl nsIContentViewerEdit {
    /* void clearSelection (); */
    #[inline]
    pub unsafe fn clearSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearSelection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectAll (); */
    #[inline]
    pub unsafe fn selectAll(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).selectAll)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void copySelection (); */
    #[inline]
    pub unsafe fn copySelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).copySelection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean copyable; */
    #[inline]
    pub unsafe fn get_copyable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_copyable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void copyLinkLocation (); */
    #[inline]
    pub unsafe fn copyLinkLocation(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).copyLinkLocation)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean inLink; */
    #[inline]
    pub unsafe fn get_inLink(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inLink)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void copyImage (in long aCopyFlags); */
    #[inline]
    pub unsafe fn copyImage(&self, aCopyFlags: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).copyImage)(self as *const _, aCopyFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean inImage; */
    #[inline]
    pub unsafe fn get_inImage(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inImage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getContents (in string aMimeType, in boolean aSelectionOnly); */
    #[inline]
    pub unsafe fn getContents(&self, aMimeType: *const libc::c_char, aSelectionOnly: bool) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getContents)(self as *const _, aMimeType, aSelectionOnly, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean canGetContents; */
    #[inline]
    pub unsafe fn get_canGetContents(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canGetContents)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setCommandNode (in nsIDOMNode aNode); */
    #[inline]
    pub unsafe fn setCommandNode(&self, aNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).setCommandNode)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


