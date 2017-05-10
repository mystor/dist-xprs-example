//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILayoutRegressionTester.idl
//


pub mod nsILayoutRegressionTester_consts {
    pub const DUMP_FLAGS_MASK_DEFAULT: i64 = 0;
    pub const DUMP_FLAGS_MASK_PRINT_MODE: i64 = 1;
    pub const DUMP_RESULT_COMPLETED: i64 = 0;
    pub const DUMP_RESULT_LOADING: i64 = 1;
    pub const DUMP_RESULT_ERROR: i64 = 2;
    pub const COMPARE_FLAGS_VERBOSE: i64 = 0;
    pub const COMPARE_FLAGS_BRIEF: i64 = 1;
}


#[repr(C)]
pub struct nsILayoutRegressionTester {
    vtable: *const nsILayoutRegressionTesterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILayoutRegressionTester {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0a065e41, 0x3a55, 0x4b5c,
            [0xbb, 0x41, 0x1e, 0x84, 0x8b, 0xb2, 0xd1, 0x0c])
    }
}

unsafe impl RefCounted for nsILayoutRegressionTester {
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
pub trait nsILayoutRegressionTesterCoerce {
    fn coerce_from(v: &nsILayoutRegressionTester) -> &Self;
}

impl nsILayoutRegressionTesterCoerce for nsILayoutRegressionTester {
    #[inline]
    fn coerce_from(v: &nsILayoutRegressionTester) -> &Self {
        v
    }
}

impl nsILayoutRegressionTester {
    #[inline]
    pub fn coerce<T: nsILayoutRegressionTesterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILayoutRegressionTester {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILayoutRegressionTesterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILayoutRegressionTester) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILayoutRegressionTesterVTable {
    pub __base: nsISupportsVTable,

    /* long dumpFrameModel (in mozIDOMWindow aWindowToDump, in nsIFile aFile, in unsigned long aFlagsMask); */
    pub dumpFrameModel: unsafe extern "C" fn (this: *const nsILayoutRegressionTester, aWindowToDump: *const mozIDOMWindow, aFile: *const nsIFile, aFlagsMask: libc::uint32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* boolean compareFrameModels (in nsIFile aBaseFile, in nsIFile aVerFile, in unsigned long aFlags); */
    pub compareFrameModels: unsafe extern "C" fn (this: *const nsILayoutRegressionTester, aBaseFile: *const nsIFile, aVerFile: *const nsIFile, aFlags: libc::uint32_t, _retval: *mut bool) -> nsresult,

}


impl nsILayoutRegressionTester {
    /* long dumpFrameModel (in mozIDOMWindow aWindowToDump, in nsIFile aFile, in unsigned long aFlagsMask); */
    #[inline]
    pub unsafe fn dumpFrameModel(&self, aWindowToDump: Option<&mozIDOMWindow>, aFile: Option<&nsIFile>, aFlagsMask: libc::uint32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).dumpFrameModel)(self as *const _, aWindowToDump.map_or(::std::ptr::null(), |x| x as *const _), aFile.map_or(::std::ptr::null(), |x| x as *const _), aFlagsMask, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean compareFrameModels (in nsIFile aBaseFile, in nsIFile aVerFile, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn compareFrameModels(&self, aBaseFile: Option<&nsIFile>, aVerFile: Option<&nsIFile>, aFlags: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).compareFrameModels)(self as *const _, aBaseFile.map_or(::std::ptr::null(), |x| x as *const _), aVerFile.map_or(::std::ptr::null(), |x| x as *const _), aFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


