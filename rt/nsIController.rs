//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIController.idl
//


#[repr(C)]
pub struct nsIController {
    vtable: *const nsIControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIController {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd5b61b82, 0x1da4, 0x11d3,
            [0xbf, 0x87, 0x00, 0x10, 0x5a, 0x1b, 0x06, 0x27])
    }
}

unsafe impl RefCounted for nsIController {
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
pub trait nsIControllerCoerce {
    fn coerce_from(v: &nsIController) -> &Self;
}

impl nsIControllerCoerce for nsIController {
    #[inline]
    fn coerce_from(v: &nsIController) -> &Self {
        v
    }
}

impl nsIController {
    #[inline]
    pub fn coerce<T: nsIControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIController {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIController) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIControllerVTable {
    pub __base: nsISupportsVTable,

    /* boolean isCommandEnabled (in string command); */
    pub isCommandEnabled: unsafe extern "C" fn (this: *const nsIController, command: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* boolean supportsCommand (in string command); */
    pub supportsCommand: unsafe extern "C" fn (this: *const nsIController, command: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* void doCommand (in string command); */
    pub doCommand: unsafe extern "C" fn (this: *const nsIController, command: *const libc::c_char) -> nsresult,

    /* void onEvent (in string eventName); */
    pub onEvent: unsafe extern "C" fn (this: *const nsIController, eventName: *const libc::c_char) -> nsresult,

}


impl nsIController {
    /* boolean isCommandEnabled (in string command); */
    #[inline]
    pub unsafe fn isCommandEnabled(&self, command: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCommandEnabled)(self as *const _, command, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean supportsCommand (in string command); */
    #[inline]
    pub unsafe fn supportsCommand(&self, command: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).supportsCommand)(self as *const _, command, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void doCommand (in string command); */
    #[inline]
    pub unsafe fn doCommand(&self, command: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).doCommand)(self as *const _, command) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onEvent (in string eventName); */
    #[inline]
    pub unsafe fn onEvent(&self, eventName: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).onEvent)(self as *const _, eventName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsICommandController {
    vtable: *const nsICommandControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICommandController {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xeec0b435, 0x7f53, 0x44fe,
            [0xb0, 0x0a, 0xcf, 0x3e, 0xed, 0x65, 0xc0, 0x1a])
    }
}

unsafe impl RefCounted for nsICommandController {
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
pub trait nsICommandControllerCoerce {
    fn coerce_from(v: &nsICommandController) -> &Self;
}

impl nsICommandControllerCoerce for nsICommandController {
    #[inline]
    fn coerce_from(v: &nsICommandController) -> &Self {
        v
    }
}

impl nsICommandController {
    #[inline]
    pub fn coerce<T: nsICommandControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICommandController {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICommandControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandController) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICommandControllerVTable {
    pub __base: nsISupportsVTable,

    /* void getCommandStateWithParams (in string command, in nsICommandParams aCommandParams); */
    pub getCommandStateWithParams: unsafe extern "C" fn (this: *const nsICommandController, command: *const libc::c_char, aCommandParams: *const nsICommandParams) -> nsresult,

    /* void doCommandWithParams (in string command, in nsICommandParams aCommandParams); */
    pub doCommandWithParams: unsafe extern "C" fn (this: *const nsICommandController, command: *const libc::c_char, aCommandParams: *const nsICommandParams) -> nsresult,

    /* void getSupportedCommands (out unsigned long count, [array, size_is (count), retval] out string commands); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSupportedCommands: *const ::libc::c_void,

}


impl nsICommandController {
    /* void getCommandStateWithParams (in string command, in nsICommandParams aCommandParams); */
    #[inline]
    pub unsafe fn getCommandStateWithParams(&self, command: *const libc::c_char, aCommandParams: Option<&nsICommandParams>) -> Result<(), nsresult> {

        match ((*self.vtable).getCommandStateWithParams)(self as *const _, command, aCommandParams.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void doCommandWithParams (in string command, in nsICommandParams aCommandParams); */
    #[inline]
    pub unsafe fn doCommandWithParams(&self, command: *const libc::c_char, aCommandParams: Option<&nsICommandParams>) -> Result<(), nsresult> {

        match ((*self.vtable).doCommandWithParams)(self as *const _, command, aCommandParams.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getSupportedCommands (out unsigned long count, [array, size_is (count), retval] out string commands); */


}


#[repr(C)]
pub struct nsIControllerCommandGroup {
    vtable: *const nsIControllerCommandGroupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIControllerCommandGroup {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9f82c404, 0x1c7b, 0x11d5,
            [0xa7, 0x3c, 0xec, 0xa4, 0x3c, 0xa8, 0x36, 0xfc])
    }
}

unsafe impl RefCounted for nsIControllerCommandGroup {
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
pub trait nsIControllerCommandGroupCoerce {
    fn coerce_from(v: &nsIControllerCommandGroup) -> &Self;
}

impl nsIControllerCommandGroupCoerce for nsIControllerCommandGroup {
    #[inline]
    fn coerce_from(v: &nsIControllerCommandGroup) -> &Self {
        v
    }
}

impl nsIControllerCommandGroup {
    #[inline]
    pub fn coerce<T: nsIControllerCommandGroupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIControllerCommandGroup {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIControllerCommandGroupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIControllerCommandGroup) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIControllerCommandGroupVTable {
    pub __base: nsISupportsVTable,

    /* void addCommandToGroup (in string aCommand, in string aGroup); */
    pub addCommandToGroup: unsafe extern "C" fn (this: *const nsIControllerCommandGroup, aCommand: *const libc::c_char, aGroup: *const libc::c_char) -> nsresult,

    /* void removeCommandFromGroup (in string aCommand, in string aGroup); */
    pub removeCommandFromGroup: unsafe extern "C" fn (this: *const nsIControllerCommandGroup, aCommand: *const libc::c_char, aGroup: *const libc::c_char) -> nsresult,

    /* boolean isCommandInGroup (in string aCommand, in string aGroup); */
    pub isCommandInGroup: unsafe extern "C" fn (this: *const nsIControllerCommandGroup, aCommand: *const libc::c_char, aGroup: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* nsISimpleEnumerator getGroupsEnumerator (); */
    pub getGroupsEnumerator: unsafe extern "C" fn (this: *const nsIControllerCommandGroup, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsISimpleEnumerator getEnumeratorForGroup (in string aGroup); */
    pub getEnumeratorForGroup: unsafe extern "C" fn (this: *const nsIControllerCommandGroup, aGroup: *const libc::c_char, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIControllerCommandGroup {
    /* void addCommandToGroup (in string aCommand, in string aGroup); */
    #[inline]
    pub unsafe fn addCommandToGroup(&self, aCommand: *const libc::c_char, aGroup: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).addCommandToGroup)(self as *const _, aCommand, aGroup) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeCommandFromGroup (in string aCommand, in string aGroup); */
    #[inline]
    pub unsafe fn removeCommandFromGroup(&self, aCommand: *const libc::c_char, aGroup: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).removeCommandFromGroup)(self as *const _, aCommand, aGroup) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isCommandInGroup (in string aCommand, in string aGroup); */
    #[inline]
    pub unsafe fn isCommandInGroup(&self, aCommand: *const libc::c_char, aGroup: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCommandInGroup)(self as *const _, aCommand, aGroup, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISimpleEnumerator getGroupsEnumerator (); */
    #[inline]
    pub unsafe fn getGroupsEnumerator(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getGroupsEnumerator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator getEnumeratorForGroup (in string aGroup); */
    #[inline]
    pub unsafe fn getEnumeratorForGroup(&self, aGroup: *const libc::c_char) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEnumeratorForGroup)(self as *const _, aGroup, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


