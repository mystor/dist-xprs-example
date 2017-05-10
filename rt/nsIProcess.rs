//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProcess.idl
//


#[repr(C)]
pub struct nsIProcess {
    vtable: *const nsIProcessVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProcess {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x609610de, 0x9954, 0x4a63,
            [0x8a, 0x7c, 0x34, 0x63, 0x50, 0xa8, 0x64, 0x03])
    }
}

unsafe impl RefCounted for nsIProcess {
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
pub trait nsIProcessCoerce {
    fn coerce_from(v: &nsIProcess) -> &Self;
}

impl nsIProcessCoerce for nsIProcess {
    #[inline]
    fn coerce_from(v: &nsIProcess) -> &Self {
        v
    }
}

impl nsIProcess {
    #[inline]
    pub fn coerce<T: nsIProcessCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProcess {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProcessCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProcess) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProcessVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIFile executable); */
    pub init: unsafe extern "C" fn (this: *const nsIProcess, executable: *const nsIFile) -> nsresult,

    /* void kill (); */
    pub kill: unsafe extern "C" fn (this: *const nsIProcess) -> nsresult,

    /* void run (in boolean blocking, [array, size_is (count)] in string args, in unsigned long count); */
    /// Unable to call function as its signature contains a non-rust type
    pub run: *const ::libc::c_void,

    /* void runAsync ([array, size_is (count)] in string args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak); */
    /// Unable to call function as its signature contains a non-rust type
    pub runAsync: *const ::libc::c_void,

    /* void runw (in boolean blocking, [array, size_is (count)] in wstring args, in unsigned long count); */
    /// Unable to call function as its signature contains a non-rust type
    pub runw: *const ::libc::c_void,

    /* void runwAsync ([array, size_is (count)] in wstring args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak); */
    /// Unable to call function as its signature contains a non-rust type
    pub runwAsync: *const ::libc::c_void,

    /* readonly attribute unsigned long pid; */
    pub get_pid: unsafe extern "C" fn (this: *const nsIProcess, aPid: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute long exitValue; */
    pub get_exitValue: unsafe extern "C" fn (this: *const nsIProcess, aExitValue: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean isRunning; */
    pub get_isRunning: unsafe extern "C" fn (this: *const nsIProcess, aIsRunning: *mut bool) -> nsresult,

}


impl nsIProcess {
    /* void init (in nsIFile executable); */
    #[inline]
    pub unsafe fn init(&self, executable: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, executable.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void kill (); */
    #[inline]
    pub unsafe fn kill(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).kill)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void run (in boolean blocking, [array, size_is (count)] in string args, in unsigned long count); */


    /* void runAsync ([array, size_is (count)] in string args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak); */


    /* void runw (in boolean blocking, [array, size_is (count)] in wstring args, in unsigned long count); */


    /* void runwAsync ([array, size_is (count)] in wstring args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak); */


    /* readonly attribute unsigned long pid; */
    #[inline]
    pub unsafe fn get_pid(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_pid)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long exitValue; */
    #[inline]
    pub unsafe fn get_exitValue(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_exitValue)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isRunning; */
    #[inline]
    pub unsafe fn get_isRunning(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isRunning)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


