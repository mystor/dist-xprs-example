//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandLineRunner.idl
//


#[repr(C)]
pub struct nsICommandLineRunner {
    vtable: *const nsICommandLineRunnerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICommandLineRunner {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc9f2996c, 0xb25a, 0x4d3d,
            [0x82, 0x1f, 0x4c, 0xd0, 0xc4, 0xbc, 0x8a, 0xfb])
    }
}

unsafe impl RefCounted for nsICommandLineRunner {
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
pub trait nsICommandLineRunnerCoerce {
    fn coerce_from(v: &nsICommandLineRunner) -> &Self;
}

impl nsICommandLineRunnerCoerce for nsICommandLineRunner {
    #[inline]
    fn coerce_from(v: &nsICommandLineRunner) -> &Self {
        v
    }
}

impl nsICommandLineRunner {
    #[inline]
    pub fn coerce<T: nsICommandLineRunnerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICommandLineRunner {
    type Target = nsICommandLine;
    #[inline]
    fn deref(&self) -> &nsICommandLine {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsICommandLineCoerce> nsICommandLineRunnerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandLineRunner) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICommandLineRunnerVTable {
    pub __base: nsICommandLineVTable,

    /* void init (in long argc, in nsArgvArray argv, in nsIFile workingDir, in unsigned long state); */
    /// Unable to call function as its signature contains a non-rust type
    pub init: *const ::libc::c_void,

    /* void setWindowContext (in nsIDOMWindow aWindow); */
    pub setWindowContext: unsafe extern "C" fn (this: *const nsICommandLineRunner, aWindow: *const nsIDOMWindow) -> nsresult,

    /* void run (); */
    pub run: unsafe extern "C" fn (this: *const nsICommandLineRunner) -> nsresult,

    /* readonly attribute AUTF8String helpText; */
    pub get_helpText: unsafe extern "C" fn (this: *const nsICommandLineRunner, aHelpText: *mut nsACString) -> nsresult,

}


impl nsICommandLineRunner {
    /* void init (in long argc, in nsArgvArray argv, in nsIFile workingDir, in unsigned long state); */


    /* void setWindowContext (in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn setWindowContext(&self, aWindow: Option<&nsIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).setWindowContext)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void run (); */
    #[inline]
    pub unsafe fn run(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).run)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute AUTF8String helpText; */
    #[inline]
    pub unsafe fn get_helpText(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_helpText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


