//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIControllerCommand.idl
//


#[repr(C)]
pub struct nsIControllerCommand {
    vtable: *const nsIControllerCommandVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIControllerCommand {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0eae9a46, 0x1dd2, 0x11b2,
            [0xac, 0xa0, 0x91, 0x76, 0xf0, 0x5f, 0xe9, 0xdb])
    }
}

unsafe impl RefCounted for nsIControllerCommand {
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
pub trait nsIControllerCommandCoerce {
    fn coerce_from(v: &nsIControllerCommand) -> &Self;
}

impl nsIControllerCommandCoerce for nsIControllerCommand {
    #[inline]
    fn coerce_from(v: &nsIControllerCommand) -> &Self {
        v
    }
}

impl nsIControllerCommand {
    #[inline]
    pub fn coerce<T: nsIControllerCommandCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIControllerCommand {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIControllerCommandCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIControllerCommand) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIControllerCommandVTable {
    pub __base: nsISupportsVTable,

    /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandContext); */
    pub isCommandEnabled: unsafe extern "C" fn (this: *const nsIControllerCommand, aCommandName: *const libc::c_char, aCommandContext: *const nsISupports, _retval: *mut bool) -> nsresult,

    /* void getCommandStateParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
    pub getCommandStateParams: unsafe extern "C" fn (this: *const nsIControllerCommand, aCommandName: *const libc::c_char, aParams: *const nsICommandParams, aCommandContext: *const nsISupports) -> nsresult,

    /* void doCommand (in string aCommandName, in nsISupports aCommandContext); */
    pub doCommand: unsafe extern "C" fn (this: *const nsIControllerCommand, aCommandName: *const libc::c_char, aCommandContext: *const nsISupports) -> nsresult,

    /* void doCommandParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
    pub doCommandParams: unsafe extern "C" fn (this: *const nsIControllerCommand, aCommandName: *const libc::c_char, aParams: *const nsICommandParams, aCommandContext: *const nsISupports) -> nsresult,

}


impl nsIControllerCommand {
    /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandContext); */
    #[inline]
    pub unsafe fn isCommandEnabled(&self, aCommandName: *const libc::c_char, aCommandContext: Option<&nsISupports>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCommandEnabled)(self as *const _, aCommandName, aCommandContext.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getCommandStateParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
    #[inline]
    pub unsafe fn getCommandStateParams(&self, aCommandName: *const libc::c_char, aParams: Option<&nsICommandParams>, aCommandContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).getCommandStateParams)(self as *const _, aCommandName, aParams.map_or(::std::ptr::null(), |x| x as *const _), aCommandContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void doCommand (in string aCommandName, in nsISupports aCommandContext); */
    #[inline]
    pub unsafe fn doCommand(&self, aCommandName: *const libc::c_char, aCommandContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).doCommand)(self as *const _, aCommandName, aCommandContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void doCommandParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
    #[inline]
    pub unsafe fn doCommandParams(&self, aCommandName: *const libc::c_char, aParams: Option<&nsICommandParams>, aCommandContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).doCommandParams)(self as *const _, aCommandName, aParams.map_or(::std::ptr::null(), |x| x as *const _), aCommandContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


