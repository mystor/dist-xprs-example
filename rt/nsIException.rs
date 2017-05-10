//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIException.idl
//


#[repr(C)]
pub struct nsIStackFrame {
    vtable: *const nsIStackFrameVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStackFrame {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x28bfb2a2, 0x5ea6, 0x4738,
            [0x91, 0x8b, 0x04, 0x9d, 0xc4, 0xd5, 0x1f, 0x0b])
    }
}

unsafe impl RefCounted for nsIStackFrame {
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
pub trait nsIStackFrameCoerce {
    fn coerce_from(v: &nsIStackFrame) -> &Self;
}

impl nsIStackFrameCoerce for nsIStackFrame {
    #[inline]
    fn coerce_from(v: &nsIStackFrame) -> &Self {
        v
    }
}

impl nsIStackFrame {
    #[inline]
    pub fn coerce<T: nsIStackFrameCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStackFrame {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStackFrameCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStackFrame) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStackFrameVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t language; */
    pub get_language: unsafe extern "C" fn (this: *const nsIStackFrame, aLanguage: *mut uint32_t) -> nsresult,

    /* readonly attribute AUTF8String languageName; */
    pub get_languageName: unsafe extern "C" fn (this: *const nsIStackFrame, aLanguageName: *mut nsACString) -> nsresult,

    /* [implicit_jscontext] readonly attribute AString filename; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_filename: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute AString name; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_name: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute int32_t lineNumber; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_lineNumber: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute int32_t columnNumber; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_columnNumber: *const ::libc::c_void,

    /* readonly attribute AUTF8String sourceLine; */
    pub get_sourceLine: unsafe extern "C" fn (this: *const nsIStackFrame, aSourceLine: *mut nsACString) -> nsresult,

    /* [implicit_jscontext] readonly attribute AString asyncCause; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_asyncCause: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute nsIStackFrame asyncCaller; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_asyncCaller: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute nsIStackFrame caller; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_caller: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute AString formattedStack; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_formattedStack: *const ::libc::c_void,

    /* readonly attribute jsval nativeSavedFrame; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_nativeSavedFrame: *const ::libc::c_void,

    /* [implicit_jscontext] AUTF8String toString (); */
    /// Unable to call function as its signature contains a non-rust type
    pub toString: *const ::libc::c_void,

}


impl nsIStackFrame {
    /* readonly attribute uint32_t language; */
    #[inline]
    pub unsafe fn get_language(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_language)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String languageName; */
    #[inline]
    pub unsafe fn get_languageName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_languageName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute AString filename; */


    /* [implicit_jscontext] readonly attribute AString name; */


    /* [implicit_jscontext] readonly attribute int32_t lineNumber; */


    /* [implicit_jscontext] readonly attribute int32_t columnNumber; */


    /* readonly attribute AUTF8String sourceLine; */
    #[inline]
    pub unsafe fn get_sourceLine(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_sourceLine)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute AString asyncCause; */


    /* [implicit_jscontext] readonly attribute nsIStackFrame asyncCaller; */


    /* [implicit_jscontext] readonly attribute nsIStackFrame caller; */


    /* [implicit_jscontext] readonly attribute AString formattedStack; */


    /* readonly attribute jsval nativeSavedFrame; */


    /* [implicit_jscontext] AUTF8String toString (); */


}


#[repr(C)]
pub struct nsIException {
    vtable: *const nsIExceptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIException {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4371b5bf, 0x6845, 0x487f,
            [0x8d, 0x9d, 0x3f, 0x1e, 0x4a, 0x9b, 0xad, 0xd2])
    }
}

unsafe impl RefCounted for nsIException {
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
pub trait nsIExceptionCoerce {
    fn coerce_from(v: &nsIException) -> &Self;
}

impl nsIExceptionCoerce for nsIException {
    #[inline]
    fn coerce_from(v: &nsIException) -> &Self {
        v
    }
}

impl nsIException {
    #[inline]
    pub fn coerce<T: nsIExceptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIException {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIExceptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIException) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIExceptionVTable {
    pub __base: nsISupportsVTable,

    /* [binaryname(MessageMoz)] readonly attribute AUTF8String message; */
    pub get_MessageMoz: unsafe extern "C" fn (this: *const nsIException, aMessage: *mut nsACString) -> nsresult,

    /* readonly attribute nsresult result; */
    pub get_result: unsafe extern "C" fn (this: *const nsIException, aResult: *mut nsresult) -> nsresult,

    /* readonly attribute AUTF8String name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIException, aName: *mut nsACString) -> nsresult,

    /* [implicit_jscontext] readonly attribute AString filename; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_filename: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute uint32_t lineNumber; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_lineNumber: *const ::libc::c_void,

    /* readonly attribute uint32_t columnNumber; */
    pub get_columnNumber: unsafe extern "C" fn (this: *const nsIException, aColumnNumber: *mut uint32_t) -> nsresult,

    /* readonly attribute nsIStackFrame location; */
    pub get_location: unsafe extern "C" fn (this: *const nsIException, aLocation: *mut *const nsIStackFrame) -> nsresult,

    /* readonly attribute nsISupports data; */
    pub get_data: unsafe extern "C" fn (this: *const nsIException, aData: *mut *const nsISupports) -> nsresult,

    /* [implicit_jscontext] AUTF8String toString (); */
    /// Unable to call function as its signature contains a non-rust type
    pub toString: *const ::libc::c_void,

}


impl nsIException {
    /* [binaryname(MessageMoz)] readonly attribute AUTF8String message; */
    #[inline]
    pub unsafe fn get_MessageMoz(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_MessageMoz)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsresult result; */
    #[inline]
    pub unsafe fn get_result(&self, ) -> Result<nsresult, nsresult> {
        let mut _retval: nsresult = ::std::mem::zeroed();
        match ((*self.vtable).get_result)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute AString filename; */


    /* [implicit_jscontext] readonly attribute uint32_t lineNumber; */


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

    /* readonly attribute nsIStackFrame location; */
    #[inline]
    pub unsafe fn get_location(&self, ) -> Result<Option<RefPtr<nsIStackFrame>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_location)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsISupports data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_data)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] AUTF8String toString (); */


}


