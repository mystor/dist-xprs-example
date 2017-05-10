//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandLine.idl
//


pub mod nsICommandLine_consts {
    pub const STATE_INITIAL_LAUNCH: i64 = 0;
    pub const STATE_REMOTE_AUTO: i64 = 1;
    pub const STATE_REMOTE_EXPLICIT: i64 = 2;
}


#[repr(C)]
pub struct nsICommandLine {
    vtable: *const nsICommandLineVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICommandLine {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbc3173bd, 0xaa46, 0x46a0,
            [0x9d, 0x25, 0xd9, 0x86, 0x7a, 0x96, 0x59, 0xb6])
    }
}

unsafe impl RefCounted for nsICommandLine {
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
pub trait nsICommandLineCoerce {
    fn coerce_from(v: &nsICommandLine) -> &Self;
}

impl nsICommandLineCoerce for nsICommandLine {
    #[inline]
    fn coerce_from(v: &nsICommandLine) -> &Self {
        v
    }
}

impl nsICommandLine {
    #[inline]
    pub fn coerce<T: nsICommandLineCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICommandLine {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICommandLineCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandLine) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICommandLineVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsICommandLine, aLength: *mut libc::int32_t) -> nsresult,

    /* AString getArgument (in long aIndex); */
    pub getArgument: unsafe extern "C" fn (this: *const nsICommandLine, aIndex: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* long findFlag (in AString aFlag, in boolean aCaseSensitive); */
    pub findFlag: unsafe extern "C" fn (this: *const nsICommandLine, aFlag: *const nsAString, aCaseSensitive: bool, _retval: *mut libc::int32_t) -> nsresult,

    /* void removeArguments (in long aStart, in long aEnd); */
    pub removeArguments: unsafe extern "C" fn (this: *const nsICommandLine, aStart: libc::int32_t, aEnd: libc::int32_t) -> nsresult,

    /* boolean handleFlag (in AString aFlag, in boolean aCaseSensitive); */
    pub handleFlag: unsafe extern "C" fn (this: *const nsICommandLine, aFlag: *const nsAString, aCaseSensitive: bool, _retval: *mut bool) -> nsresult,

    /* AString handleFlagWithParam (in AString aFlag, in boolean aCaseSensitive); */
    pub handleFlagWithParam: unsafe extern "C" fn (this: *const nsICommandLine, aFlag: *const nsAString, aCaseSensitive: bool, _retval: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned long state; */
    pub get_state: unsafe extern "C" fn (this: *const nsICommandLine, aState: *mut libc::uint32_t) -> nsresult,

    /* attribute boolean preventDefault; */
    pub get_preventDefault: unsafe extern "C" fn (this: *const nsICommandLine, aPreventDefault: *mut bool) -> nsresult,
    pub set_preventDefault: unsafe extern "C" fn (this: *const nsICommandLine, aPreventDefault: bool) -> nsresult,

    /* readonly attribute nsIFile workingDirectory; */
    pub get_workingDirectory: unsafe extern "C" fn (this: *const nsICommandLine, aWorkingDirectory: *mut *const nsIFile) -> nsresult,

    /* readonly attribute nsIDOMWindow windowContext; */
    pub get_windowContext: unsafe extern "C" fn (this: *const nsICommandLine, aWindowContext: *mut *const nsIDOMWindow) -> nsresult,

    /* nsIFile resolveFile (in AString aArgument); */
    pub resolveFile: unsafe extern "C" fn (this: *const nsICommandLine, aArgument: *const nsAString, _retval: *mut *const nsIFile) -> nsresult,

    /* nsIURI resolveURI (in AString aArgument); */
    pub resolveURI: unsafe extern "C" fn (this: *const nsICommandLine, aArgument: *const nsAString, _retval: *mut *const nsIURI) -> nsresult,

}


impl nsICommandLine {
    /* readonly attribute long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getArgument (in long aIndex); */
    #[inline]
    pub unsafe fn getArgument(&self, aIndex: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getArgument)(self as *const _, aIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long findFlag (in AString aFlag, in boolean aCaseSensitive); */
    #[inline]
    pub unsafe fn findFlag(&self, aFlag: &[u16], aCaseSensitive: bool) -> Result<libc::int32_t, nsresult> {
        let aFlag = nsString::from(aFlag);
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).findFlag)(self as *const _, &*aFlag, aCaseSensitive, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void removeArguments (in long aStart, in long aEnd); */
    #[inline]
    pub unsafe fn removeArguments(&self, aStart: libc::int32_t, aEnd: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeArguments)(self as *const _, aStart, aEnd) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean handleFlag (in AString aFlag, in boolean aCaseSensitive); */
    #[inline]
    pub unsafe fn handleFlag(&self, aFlag: &[u16], aCaseSensitive: bool) -> Result<bool, nsresult> {
        let aFlag = nsString::from(aFlag);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).handleFlag)(self as *const _, &*aFlag, aCaseSensitive, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString handleFlagWithParam (in AString aFlag, in boolean aCaseSensitive); */
    #[inline]
    pub unsafe fn handleFlagWithParam(&self, aFlag: &[u16], aCaseSensitive: bool) -> Result<nsString, nsresult> {
        let aFlag = nsString::from(aFlag);
        let mut _retval = nsString::new();
        match ((*self.vtable).handleFlagWithParam)(self as *const _, &*aFlag, aCaseSensitive, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_state)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean preventDefault; */
    #[inline]
    pub unsafe fn get_preventDefault(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_preventDefault)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_preventDefault(&self, aPreventDefault: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_preventDefault)(self as *const _, aPreventDefault) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIFile workingDirectory; */
    #[inline]
    pub unsafe fn get_workingDirectory(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_workingDirectory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMWindow windowContext; */
    #[inline]
    pub unsafe fn get_windowContext(&self, ) -> Result<Option<RefPtr<nsIDOMWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_windowContext)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIFile resolveFile (in AString aArgument); */
    #[inline]
    pub unsafe fn resolveFile(&self, aArgument: &[u16]) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let aArgument = nsString::from(aArgument);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).resolveFile)(self as *const _, &*aArgument, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIURI resolveURI (in AString aArgument); */
    #[inline]
    pub unsafe fn resolveURI(&self, aArgument: &[u16]) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let aArgument = nsString::from(aArgument);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).resolveURI)(self as *const _, &*aArgument, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


