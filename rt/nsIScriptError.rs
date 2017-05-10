//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptError.idl
//


#[repr(C)]
pub struct nsIScriptErrorNote {
    vtable: *const nsIScriptErrorNoteVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptErrorNote {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe8933fc9, 0xc302, 0x4e12,
            [0xa5, 0x5b, 0x4f, 0x88, 0x61, 0x1d, 0x9c, 0x6c])
    }
}

unsafe impl RefCounted for nsIScriptErrorNote {
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
pub trait nsIScriptErrorNoteCoerce {
    fn coerce_from(v: &nsIScriptErrorNote) -> &Self;
}

impl nsIScriptErrorNoteCoerce for nsIScriptErrorNote {
    #[inline]
    fn coerce_from(v: &nsIScriptErrorNote) -> &Self {
        v
    }
}

impl nsIScriptErrorNote {
    #[inline]
    pub fn coerce<T: nsIScriptErrorNoteCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptErrorNote {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScriptErrorNoteCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptErrorNote) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptErrorNoteVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString errorMessage; */
    pub get_errorMessage: unsafe extern "C" fn (this: *const nsIScriptErrorNote, aErrorMessage: *mut nsAString) -> nsresult,

    /* readonly attribute AString sourceName; */
    pub get_sourceName: unsafe extern "C" fn (this: *const nsIScriptErrorNote, aSourceName: *mut nsAString) -> nsresult,

    /* readonly attribute uint32_t lineNumber; */
    pub get_lineNumber: unsafe extern "C" fn (this: *const nsIScriptErrorNote, aLineNumber: *mut uint32_t) -> nsresult,

    /* readonly attribute uint32_t columnNumber; */
    pub get_columnNumber: unsafe extern "C" fn (this: *const nsIScriptErrorNote, aColumnNumber: *mut uint32_t) -> nsresult,

    /* AUTF8String toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsIScriptErrorNote, _retval: *mut nsACString) -> nsresult,

}


impl nsIScriptErrorNote {
    /* readonly attribute AString errorMessage; */
    #[inline]
    pub unsafe fn get_errorMessage(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_errorMessage)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString sourceName; */
    #[inline]
    pub unsafe fn get_sourceName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_sourceName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t lineNumber; */
    #[inline]
    pub unsafe fn get_lineNumber(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lineNumber)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t columnNumber; */
    #[inline]
    pub unsafe fn get_columnNumber(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_columnNumber)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).toString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsIScriptError_consts {
    pub const errorFlag: i64 = 0;
    pub const warningFlag: i64 = 1;
    pub const exceptionFlag: i64 = 2;
    pub const strictFlag: i64 = 4;
    pub const infoFlag: i64 = 8;
}


#[repr(C)]
pub struct nsIScriptError {
    vtable: *const nsIScriptErrorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptError {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x63eb4d3e, 0x7d99, 0x4150,
            [0xb4, 0xf3, 0x11, 0x31, 0x4f, 0x9d, 0x82, 0xa9])
    }
}

unsafe impl RefCounted for nsIScriptError {
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
pub trait nsIScriptErrorCoerce {
    fn coerce_from(v: &nsIScriptError) -> &Self;
}

impl nsIScriptErrorCoerce for nsIScriptError {
    #[inline]
    fn coerce_from(v: &nsIScriptError) -> &Self {
        v
    }
}

impl nsIScriptError {
    #[inline]
    pub fn coerce<T: nsIScriptErrorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptError {
    type Target = nsIConsoleMessage;
    #[inline]
    fn deref(&self) -> &nsIConsoleMessage {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIConsoleMessageCoerce> nsIScriptErrorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptError) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptErrorVTable {
    pub __base: nsIConsoleMessageVTable,

    /* readonly attribute AString errorMessage; */
    pub get_errorMessage: unsafe extern "C" fn (this: *const nsIScriptError, aErrorMessage: *mut nsAString) -> nsresult,

    /* readonly attribute AString sourceName; */
    pub get_sourceName: unsafe extern "C" fn (this: *const nsIScriptError, aSourceName: *mut nsAString) -> nsresult,

    /* readonly attribute AString sourceLine; */
    pub get_sourceLine: unsafe extern "C" fn (this: *const nsIScriptError, aSourceLine: *mut nsAString) -> nsresult,

    /* readonly attribute uint32_t lineNumber; */
    pub get_lineNumber: unsafe extern "C" fn (this: *const nsIScriptError, aLineNumber: *mut uint32_t) -> nsresult,

    /* readonly attribute uint32_t columnNumber; */
    pub get_columnNumber: unsafe extern "C" fn (this: *const nsIScriptError, aColumnNumber: *mut uint32_t) -> nsresult,

    /* readonly attribute uint32_t flags; */
    pub get_flags: unsafe extern "C" fn (this: *const nsIScriptError, aFlags: *mut uint32_t) -> nsresult,

    /* readonly attribute string category; */
    pub get_category: unsafe extern "C" fn (this: *const nsIScriptError, aCategory: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute unsigned long long outerWindowID; */
    pub get_outerWindowID: unsafe extern "C" fn (this: *const nsIScriptError, aOuterWindowID: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long long innerWindowID; */
    pub get_innerWindowID: unsafe extern "C" fn (this: *const nsIScriptError, aInnerWindowID: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute boolean isFromPrivateWindow; */
    pub get_isFromPrivateWindow: unsafe extern "C" fn (this: *const nsIScriptError, aIsFromPrivateWindow: *mut bool) -> nsresult,

    /* attribute jsval stack; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_stack: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_stack: *const ::libc::c_void,

    /* attribute AString errorMessageName; */
    pub get_errorMessageName: unsafe extern "C" fn (this: *const nsIScriptError, aErrorMessageName: *mut nsAString) -> nsresult,
    pub set_errorMessageName: unsafe extern "C" fn (this: *const nsIScriptError, aErrorMessageName: *const nsAString) -> nsresult,

    /* readonly attribute nsIArray notes; */
    pub get_notes: unsafe extern "C" fn (this: *const nsIScriptError, aNotes: *mut *const nsIArray) -> nsresult,

    /* void init (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in string category); */
    pub init: unsafe extern "C" fn (this: *const nsIScriptError, message: *const nsAString, sourceName: *const nsAString, sourceLine: *const nsAString, lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const libc::c_char) -> nsresult,

    /* void initWithWindowID (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID); */
    pub initWithWindowID: unsafe extern "C" fn (this: *const nsIScriptError, message: *const nsAString, sourceName: *const nsAString, sourceLine: *const nsAString, lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const nsACString, innerWindowID: libc::uint64_t) -> nsresult,

}


impl nsIScriptError {
    /* readonly attribute AString errorMessage; */
    #[inline]
    pub unsafe fn get_errorMessage(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_errorMessage)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString sourceName; */
    #[inline]
    pub unsafe fn get_sourceName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_sourceName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString sourceLine; */
    #[inline]
    pub unsafe fn get_sourceLine(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_sourceLine)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t lineNumber; */
    #[inline]
    pub unsafe fn get_lineNumber(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lineNumber)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t columnNumber; */
    #[inline]
    pub unsafe fn get_columnNumber(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_columnNumber)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t flags; */
    #[inline]
    pub unsafe fn get_flags(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_flags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string category; */
    #[inline]
    pub unsafe fn get_category(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_category)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long outerWindowID; */
    #[inline]
    pub unsafe fn get_outerWindowID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_outerWindowID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long innerWindowID; */
    #[inline]
    pub unsafe fn get_innerWindowID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_innerWindowID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isFromPrivateWindow; */
    #[inline]
    pub unsafe fn get_isFromPrivateWindow(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isFromPrivateWindow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute jsval stack; */



    /* attribute AString errorMessageName; */
    #[inline]
    pub unsafe fn get_errorMessageName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_errorMessageName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_errorMessageName(&self, aErrorMessageName: &[u16]) -> Result<(), nsresult> {
        let aErrorMessageName = nsString::from(aErrorMessageName);
        match ((*self.vtable).set_errorMessageName)(self as *const _, &*aErrorMessageName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIArray notes; */
    #[inline]
    pub unsafe fn get_notes(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_notes)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void init (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in string category); */
    #[inline]
    pub unsafe fn init(&self, message: &[u16], sourceName: &[u16], sourceLine: &[u16], lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const libc::c_char) -> Result<(), nsresult> {
        let message = nsString::from(message);
        let sourceName = nsString::from(sourceName);
        let sourceLine = nsString::from(sourceLine);
        match ((*self.vtable).init)(self as *const _, &*message, &*sourceName, &*sourceLine, lineNumber, columnNumber, flags, category) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void initWithWindowID (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID); */
    #[inline]
    pub unsafe fn initWithWindowID(&self, message: &[u16], sourceName: &[u16], sourceLine: &[u16], lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: &[u8], innerWindowID: libc::uint64_t) -> Result<(), nsresult> {
        let message = nsString::from(message);
        let sourceName = nsString::from(sourceName);
        let sourceLine = nsString::from(sourceLine);
        let category = nsCString::from(category);
        match ((*self.vtable).initWithWindowID)(self as *const _, &*message, &*sourceName, &*sourceLine, lineNumber, columnNumber, flags, &*category, innerWindowID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


