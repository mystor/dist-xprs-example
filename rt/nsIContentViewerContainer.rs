//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentViewerContainer.idl
//


#[repr(C)]
pub struct nsIContentViewerContainer {
    vtable: *const nsIContentViewerContainerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentViewerContainer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xea2ce7a0, 0x5c3d, 0x11d4,
            [0x90, 0xc2, 0x00, 0x50, 0x04, 0x1c, 0xaf, 0x44])
    }
}

unsafe impl RefCounted for nsIContentViewerContainer {
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
pub trait nsIContentViewerContainerCoerce {
    fn coerce_from(v: &nsIContentViewerContainer) -> &Self;
}

impl nsIContentViewerContainerCoerce for nsIContentViewerContainer {
    #[inline]
    fn coerce_from(v: &nsIContentViewerContainer) -> &Self {
        v
    }
}

impl nsIContentViewerContainer {
    #[inline]
    pub fn coerce<T: nsIContentViewerContainerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentViewerContainer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentViewerContainerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentViewerContainer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentViewerContainerVTable {
    pub __base: nsISupportsVTable,

    /* void embed (in nsIContentViewer aDocViewer, in string aCommand, in nsISupports aExtraInfo); */
    pub embed: unsafe extern "C" fn (this: *const nsIContentViewerContainer, aDocViewer: *const nsIContentViewer, aCommand: *const libc::c_char, aExtraInfo: *const nsISupports) -> nsresult,

    /* void setIsPrinting (in boolean aIsPrinting); */
    pub setIsPrinting: unsafe extern "C" fn (this: *const nsIContentViewerContainer, aIsPrinting: bool) -> nsresult,

}


impl nsIContentViewerContainer {
    /* void embed (in nsIContentViewer aDocViewer, in string aCommand, in nsISupports aExtraInfo); */
    #[inline]
    pub unsafe fn embed(&self, aDocViewer: Option<&nsIContentViewer>, aCommand: *const libc::c_char, aExtraInfo: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).embed)(self as *const _, aDocViewer.map_or(::std::ptr::null(), |x| x as *const _), aCommand, aExtraInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setIsPrinting (in boolean aIsPrinting); */
    #[inline]
    pub unsafe fn setIsPrinting(&self, aIsPrinting: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setIsPrinting)(self as *const _, aIsPrinting) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


