//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIControllerCommandTable.idl
//


#[repr(C)]
pub struct nsIControllerCommandTable {
    vtable: *const nsIControllerCommandTableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIControllerCommandTable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc847f90e, 0xb8f3, 0x49db,
            [0xa4, 0xdf, 0x88, 0x67, 0x83, 0x1f, 0x28, 0x00])
    }
}

unsafe impl RefCounted for nsIControllerCommandTable {
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
pub trait nsIControllerCommandTableCoerce {
    fn coerce_from(v: &nsIControllerCommandTable) -> &Self;
}

impl nsIControllerCommandTableCoerce for nsIControllerCommandTable {
    #[inline]
    fn coerce_from(v: &nsIControllerCommandTable) -> &Self {
        v
    }
}

impl nsIControllerCommandTable {
    #[inline]
    pub fn coerce<T: nsIControllerCommandTableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIControllerCommandTable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIControllerCommandTableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIControllerCommandTable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIControllerCommandTableVTable {
    pub __base: nsISupportsVTable,

    /* void makeImmutable (); */
    pub makeImmutable: unsafe extern "C" fn (this: *const nsIControllerCommandTable) -> nsresult,

    /* void registerCommand (in string aCommandName, in nsIControllerCommand aCommand); */
    pub registerCommand: unsafe extern "C" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommand: *const nsIControllerCommand) -> nsresult,

    /* void unregisterCommand (in string aCommandName, in nsIControllerCommand aCommand); */
    pub unregisterCommand: unsafe extern "C" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommand: *const nsIControllerCommand) -> nsresult,

    /* nsIControllerCommand findCommandHandler (in string aCommandName); */
    pub findCommandHandler: unsafe extern "C" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, _retval: *mut *const nsIControllerCommand) -> nsresult,

    /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandRefCon); */
    pub isCommandEnabled: unsafe extern "C" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports, _retval: *mut bool) -> nsresult,

    /* void updateCommandState (in string aCommandName, in nsISupports aCommandRefCon); */
    pub updateCommandState: unsafe extern "C" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports) -> nsresult,

    /* boolean supportsCommand (in string aCommandName, in nsISupports aCommandRefCon); */
    pub supportsCommand: unsafe extern "C" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports, _retval: *mut bool) -> nsresult,

    /* void doCommand (in string aCommandName, in nsISupports aCommandRefCon); */
    pub doCommand: unsafe extern "C" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aCommandRefCon: *const nsISupports) -> nsresult,

    /* void doCommandParams (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon); */
    pub doCommandParams: unsafe extern "C" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aParam: *const nsICommandParams, aCommandRefCon: *const nsISupports) -> nsresult,

    /* void getCommandState (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon); */
    pub getCommandState: unsafe extern "C" fn (this: *const nsIControllerCommandTable, aCommandName: *const libc::c_char, aParam: *const nsICommandParams, aCommandRefCon: *const nsISupports) -> nsresult,

    /* void getSupportedCommands (out unsigned long count, [array, size_is (count), retval] out string commands); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSupportedCommands: *const ::libc::c_void,

}


impl nsIControllerCommandTable {
    /* void makeImmutable (); */
    #[inline]
    pub unsafe fn makeImmutable(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).makeImmutable)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerCommand (in string aCommandName, in nsIControllerCommand aCommand); */
    #[inline]
    pub unsafe fn registerCommand(&self, aCommandName: *const libc::c_char, aCommand: Option<&nsIControllerCommand>) -> Result<(), nsresult> {

        match ((*self.vtable).registerCommand)(self as *const _, aCommandName, aCommand.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterCommand (in string aCommandName, in nsIControllerCommand aCommand); */
    #[inline]
    pub unsafe fn unregisterCommand(&self, aCommandName: *const libc::c_char, aCommand: Option<&nsIControllerCommand>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterCommand)(self as *const _, aCommandName, aCommand.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIControllerCommand findCommandHandler (in string aCommandName); */
    #[inline]
    pub unsafe fn findCommandHandler(&self, aCommandName: *const libc::c_char) -> Result<Option<RefPtr<nsIControllerCommand>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findCommandHandler)(self as *const _, aCommandName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandRefCon); */
    #[inline]
    pub unsafe fn isCommandEnabled(&self, aCommandName: *const libc::c_char, aCommandRefCon: Option<&nsISupports>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCommandEnabled)(self as *const _, aCommandName, aCommandRefCon.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void updateCommandState (in string aCommandName, in nsISupports aCommandRefCon); */
    #[inline]
    pub unsafe fn updateCommandState(&self, aCommandName: *const libc::c_char, aCommandRefCon: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).updateCommandState)(self as *const _, aCommandName, aCommandRefCon.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean supportsCommand (in string aCommandName, in nsISupports aCommandRefCon); */
    #[inline]
    pub unsafe fn supportsCommand(&self, aCommandName: *const libc::c_char, aCommandRefCon: Option<&nsISupports>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).supportsCommand)(self as *const _, aCommandName, aCommandRefCon.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void doCommand (in string aCommandName, in nsISupports aCommandRefCon); */
    #[inline]
    pub unsafe fn doCommand(&self, aCommandName: *const libc::c_char, aCommandRefCon: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).doCommand)(self as *const _, aCommandName, aCommandRefCon.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void doCommandParams (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon); */
    #[inline]
    pub unsafe fn doCommandParams(&self, aCommandName: *const libc::c_char, aParam: Option<&nsICommandParams>, aCommandRefCon: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).doCommandParams)(self as *const _, aCommandName, aParam.map_or(::std::ptr::null(), |x| x as *const _), aCommandRefCon.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getCommandState (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon); */
    #[inline]
    pub unsafe fn getCommandState(&self, aCommandName: *const libc::c_char, aParam: Option<&nsICommandParams>, aCommandRefCon: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).getCommandState)(self as *const _, aCommandName, aParam.map_or(::std::ptr::null(), |x| x as *const _), aCommandRefCon.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getSupportedCommands (out unsigned long count, [array, size_is (count), retval] out string commands); */


}


