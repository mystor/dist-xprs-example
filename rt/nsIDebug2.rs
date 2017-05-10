//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDebug2.idl
//


#[repr(C)]
pub struct nsIDebug2 {
    vtable: *const nsIDebug2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDebug2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9641dc15, 0x10fb, 0x42e3,
            [0xa2, 0x85, 0x18, 0xbe, 0x90, 0xa5, 0xc1, 0x0b])
    }
}

unsafe impl RefCounted for nsIDebug2 {
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
pub trait nsIDebug2Coerce {
    fn coerce_from(v: &nsIDebug2) -> &Self;
}

impl nsIDebug2Coerce for nsIDebug2 {
    #[inline]
    fn coerce_from(v: &nsIDebug2) -> &Self {
        v
    }
}

impl nsIDebug2 {
    #[inline]
    pub fn coerce<T: nsIDebug2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDebug2 {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDebug2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIDebug2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDebug2VTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isDebugBuild; */
    pub get_isDebugBuild: unsafe extern "C" fn (this: *const nsIDebug2, aIsDebugBuild: *mut bool) -> nsresult,

    /* readonly attribute long assertionCount; */
    pub get_assertionCount: unsafe extern "C" fn (this: *const nsIDebug2, aAssertionCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute bool isDebuggerAttached; */
    pub get_isDebuggerAttached: unsafe extern "C" fn (this: *const nsIDebug2, aIsDebuggerAttached: *mut bool) -> nsresult,

    /* void assertion (in string aStr, in string aExpr, in string aFile, in long aLine); */
    pub assertion: unsafe extern "C" fn (this: *const nsIDebug2, aStr: *const libc::c_char, aExpr: *const libc::c_char, aFile: *const libc::c_char, aLine: libc::int32_t) -> nsresult,

    /* void warning (in string aStr, in string aFile, in long aLine); */
    pub warning: unsafe extern "C" fn (this: *const nsIDebug2, aStr: *const libc::c_char, aFile: *const libc::c_char, aLine: libc::int32_t) -> nsresult,

    /* void break (in string aFile, in long aLine); */
    pub break_: unsafe extern "C" fn (this: *const nsIDebug2, aFile: *const libc::c_char, aLine: libc::int32_t) -> nsresult,

    /* void abort (in string aFile, in long aLine); */
    pub abort: unsafe extern "C" fn (this: *const nsIDebug2, aFile: *const libc::c_char, aLine: libc::int32_t) -> nsresult,

    /* void rustPanic (in string aMessage); */
    pub rustPanic: unsafe extern "C" fn (this: *const nsIDebug2, aMessage: *const libc::c_char) -> nsresult,

}


impl nsIDebug2 {
    /* readonly attribute boolean isDebugBuild; */
    #[inline]
    pub unsafe fn get_isDebugBuild(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDebugBuild)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long assertionCount; */
    #[inline]
    pub unsafe fn get_assertionCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_assertionCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool isDebuggerAttached; */
    #[inline]
    pub unsafe fn get_isDebuggerAttached(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDebuggerAttached)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void assertion (in string aStr, in string aExpr, in string aFile, in long aLine); */
    #[inline]
    pub unsafe fn assertion(&self, aStr: *const libc::c_char, aExpr: *const libc::c_char, aFile: *const libc::c_char, aLine: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).assertion)(self as *const _, aStr, aExpr, aFile, aLine) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void warning (in string aStr, in string aFile, in long aLine); */
    #[inline]
    pub unsafe fn warning(&self, aStr: *const libc::c_char, aFile: *const libc::c_char, aLine: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).warning)(self as *const _, aStr, aFile, aLine) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void break (in string aFile, in long aLine); */
    #[inline]
    pub unsafe fn break_(&self, aFile: *const libc::c_char, aLine: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).break_)(self as *const _, aFile, aLine) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void abort (in string aFile, in long aLine); */
    #[inline]
    pub unsafe fn abort(&self, aFile: *const libc::c_char, aLine: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).abort)(self as *const _, aFile, aLine) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void rustPanic (in string aMessage); */
    #[inline]
    pub unsafe fn rustPanic(&self, aMessage: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).rustPanic)(self as *const _, aMessage) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


