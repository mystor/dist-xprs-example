//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContextMenuListener2.idl
//


pub mod nsIContextMenuListener2_consts {
    pub const CONTEXT_NONE: i64 = 0;
    pub const CONTEXT_LINK: i64 = 1;
    pub const CONTEXT_IMAGE: i64 = 2;
    pub const CONTEXT_DOCUMENT: i64 = 4;
    pub const CONTEXT_TEXT: i64 = 8;
    pub const CONTEXT_INPUT: i64 = 16;
    pub const CONTEXT_BACKGROUND_IMAGE: i64 = 32;
}


#[repr(C)]
pub struct nsIContextMenuListener2 {
    vtable: *const nsIContextMenuListener2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContextMenuListener2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7fb719b3, 0xd804, 0x4964,
            [0x95, 0x96, 0x77, 0xcf, 0x92, 0x4e, 0xe3, 0x14])
    }
}

unsafe impl RefCounted for nsIContextMenuListener2 {
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
pub trait nsIContextMenuListener2Coerce {
    fn coerce_from(v: &nsIContextMenuListener2) -> &Self;
}

impl nsIContextMenuListener2Coerce for nsIContextMenuListener2 {
    #[inline]
    fn coerce_from(v: &nsIContextMenuListener2) -> &Self {
        v
    }
}

impl nsIContextMenuListener2 {
    #[inline]
    pub fn coerce<T: nsIContextMenuListener2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContextMenuListener2 {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContextMenuListener2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIContextMenuListener2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContextMenuListener2VTable {
    pub __base: nsISupportsVTable,

    /* void onShowContextMenu (in unsigned long aContextFlags, in nsIContextMenuInfo aUtils); */
    pub onShowContextMenu: unsafe extern "C" fn (this: *const nsIContextMenuListener2, aContextFlags: libc::uint32_t, aUtils: *const nsIContextMenuInfo) -> nsresult,

}


impl nsIContextMenuListener2 {
    /* void onShowContextMenu (in unsigned long aContextFlags, in nsIContextMenuInfo aUtils); */
    #[inline]
    pub unsafe fn onShowContextMenu(&self, aContextFlags: libc::uint32_t, aUtils: Option<&nsIContextMenuInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).onShowContextMenu)(self as *const _, aContextFlags, aUtils.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIContextMenuInfo {
    vtable: *const nsIContextMenuInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContextMenuInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2f977d56, 0x5485, 0x11d4,
            [0x87, 0xe2, 0x00, 0x10, 0xa4, 0xe7, 0x5e, 0xf2])
    }
}

unsafe impl RefCounted for nsIContextMenuInfo {
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
pub trait nsIContextMenuInfoCoerce {
    fn coerce_from(v: &nsIContextMenuInfo) -> &Self;
}

impl nsIContextMenuInfoCoerce for nsIContextMenuInfo {
    #[inline]
    fn coerce_from(v: &nsIContextMenuInfo) -> &Self {
        v
    }
}

impl nsIContextMenuInfo {
    #[inline]
    pub fn coerce<T: nsIContextMenuInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContextMenuInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContextMenuInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContextMenuInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContextMenuInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMEvent mouseEvent; */
    pub get_mouseEvent: unsafe extern "C" fn (this: *const nsIContextMenuInfo, aMouseEvent: *mut *const nsIDOMEvent) -> nsresult,

    /* readonly attribute nsIDOMNode targetNode; */
    pub get_targetNode: unsafe extern "C" fn (this: *const nsIContextMenuInfo, aTargetNode: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute AString associatedLink; */
    pub get_associatedLink: unsafe extern "C" fn (this: *const nsIContextMenuInfo, aAssociatedLink: *mut nsAString) -> nsresult,

    /* readonly attribute imgIContainer imageContainer; */
    pub get_imageContainer: unsafe extern "C" fn (this: *const nsIContextMenuInfo, aImageContainer: *mut *const imgIContainer) -> nsresult,

    /* readonly attribute nsIURI imageSrc; */
    pub get_imageSrc: unsafe extern "C" fn (this: *const nsIContextMenuInfo, aImageSrc: *mut *const nsIURI) -> nsresult,

    /* readonly attribute imgIContainer backgroundImageContainer; */
    pub get_backgroundImageContainer: unsafe extern "C" fn (this: *const nsIContextMenuInfo, aBackgroundImageContainer: *mut *const imgIContainer) -> nsresult,

    /* readonly attribute nsIURI backgroundImageSrc; */
    pub get_backgroundImageSrc: unsafe extern "C" fn (this: *const nsIContextMenuInfo, aBackgroundImageSrc: *mut *const nsIURI) -> nsresult,

}


impl nsIContextMenuInfo {
    /* readonly attribute nsIDOMEvent mouseEvent; */
    #[inline]
    pub unsafe fn get_mouseEvent(&self, ) -> Result<Option<RefPtr<nsIDOMEvent>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_mouseEvent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMNode targetNode; */
    #[inline]
    pub unsafe fn get_targetNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_targetNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString associatedLink; */
    #[inline]
    pub unsafe fn get_associatedLink(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_associatedLink)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute imgIContainer imageContainer; */
    #[inline]
    pub unsafe fn get_imageContainer(&self, ) -> Result<Option<RefPtr<imgIContainer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_imageContainer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI imageSrc; */
    #[inline]
    pub unsafe fn get_imageSrc(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_imageSrc)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute imgIContainer backgroundImageContainer; */
    #[inline]
    pub unsafe fn get_backgroundImageContainer(&self, ) -> Result<Option<RefPtr<imgIContainer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_backgroundImageContainer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI backgroundImageSrc; */
    #[inline]
    pub unsafe fn get_backgroundImageSrc(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_backgroundImageSrc)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


