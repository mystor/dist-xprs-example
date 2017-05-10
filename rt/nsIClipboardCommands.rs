//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClipboardCommands.idl
//


#[repr(C)]
pub struct nsIClipboardCommands {
    vtable: *const nsIClipboardCommandsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClipboardCommands {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb8100c90, 0x73be, 0x11d2,
            [0x92, 0xa5, 0x00, 0x10, 0x5a, 0x1b, 0x0d, 0x64])
    }
}

unsafe impl RefCounted for nsIClipboardCommands {
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
pub trait nsIClipboardCommandsCoerce {
    fn coerce_from(v: &nsIClipboardCommands) -> &Self;
}

impl nsIClipboardCommandsCoerce for nsIClipboardCommands {
    #[inline]
    fn coerce_from(v: &nsIClipboardCommands) -> &Self {
        v
    }
}

impl nsIClipboardCommands {
    #[inline]
    pub fn coerce<T: nsIClipboardCommandsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClipboardCommands {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClipboardCommandsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClipboardCommands) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClipboardCommandsVTable {
    pub __base: nsISupportsVTable,

    /* boolean canCutSelection (); */
    pub canCutSelection: unsafe extern "C" fn (this: *const nsIClipboardCommands, _retval: *mut bool) -> nsresult,

    /* boolean canCopySelection (); */
    pub canCopySelection: unsafe extern "C" fn (this: *const nsIClipboardCommands, _retval: *mut bool) -> nsresult,

    /* boolean canCopyLinkLocation (); */
    pub canCopyLinkLocation: unsafe extern "C" fn (this: *const nsIClipboardCommands, _retval: *mut bool) -> nsresult,

    /* boolean canCopyImageLocation (); */
    pub canCopyImageLocation: unsafe extern "C" fn (this: *const nsIClipboardCommands, _retval: *mut bool) -> nsresult,

    /* boolean canCopyImageContents (); */
    pub canCopyImageContents: unsafe extern "C" fn (this: *const nsIClipboardCommands, _retval: *mut bool) -> nsresult,

    /* boolean canPaste (); */
    pub canPaste: unsafe extern "C" fn (this: *const nsIClipboardCommands, _retval: *mut bool) -> nsresult,

    /* void cutSelection (); */
    pub cutSelection: unsafe extern "C" fn (this: *const nsIClipboardCommands) -> nsresult,

    /* void copySelection (); */
    pub copySelection: unsafe extern "C" fn (this: *const nsIClipboardCommands) -> nsresult,

    /* void copyLinkLocation (); */
    pub copyLinkLocation: unsafe extern "C" fn (this: *const nsIClipboardCommands) -> nsresult,

    /* void copyImageLocation (); */
    pub copyImageLocation: unsafe extern "C" fn (this: *const nsIClipboardCommands) -> nsresult,

    /* void copyImageContents (); */
    pub copyImageContents: unsafe extern "C" fn (this: *const nsIClipboardCommands) -> nsresult,

    /* void paste (); */
    pub paste: unsafe extern "C" fn (this: *const nsIClipboardCommands) -> nsresult,

    /* void selectAll (); */
    pub selectAll: unsafe extern "C" fn (this: *const nsIClipboardCommands) -> nsresult,

    /* void selectNone (); */
    pub selectNone: unsafe extern "C" fn (this: *const nsIClipboardCommands) -> nsresult,

}


impl nsIClipboardCommands {
    /* boolean canCutSelection (); */
    #[inline]
    pub unsafe fn canCutSelection(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canCutSelection)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canCopySelection (); */
    #[inline]
    pub unsafe fn canCopySelection(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canCopySelection)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canCopyLinkLocation (); */
    #[inline]
    pub unsafe fn canCopyLinkLocation(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canCopyLinkLocation)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canCopyImageLocation (); */
    #[inline]
    pub unsafe fn canCopyImageLocation(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canCopyImageLocation)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canCopyImageContents (); */
    #[inline]
    pub unsafe fn canCopyImageContents(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canCopyImageContents)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canPaste (); */
    #[inline]
    pub unsafe fn canPaste(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canPaste)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void cutSelection (); */
    #[inline]
    pub unsafe fn cutSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cutSelection)(self as *const _, ) {
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

    /* void copyLinkLocation (); */
    #[inline]
    pub unsafe fn copyLinkLocation(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).copyLinkLocation)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void copyImageLocation (); */
    #[inline]
    pub unsafe fn copyImageLocation(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).copyImageLocation)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void copyImageContents (); */
    #[inline]
    pub unsafe fn copyImageContents(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).copyImageContents)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void paste (); */
    #[inline]
    pub unsafe fn paste(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).paste)(self as *const _, ) {
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

    /* void selectNone (); */
    #[inline]
    pub unsafe fn selectNone(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).selectNone)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


