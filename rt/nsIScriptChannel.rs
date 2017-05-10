//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptChannel.idl
//


pub mod nsIScriptChannel_consts {
    pub const NO_EXECUTION: i64 = 0;
    pub const EXECUTE_NORMAL: i64 = 2;
}


#[repr(C)]
pub struct nsIScriptChannel {
    vtable: *const nsIScriptChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x33234b99, 0x9588, 0x4c7d,
            [0x9d, 0xa6, 0x86, 0xb8, 0xb7, 0xcb, 0xa5, 0x65])
    }
}

unsafe impl RefCounted for nsIScriptChannel {
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
pub trait nsIScriptChannelCoerce {
    fn coerce_from(v: &nsIScriptChannel) -> &Self;
}

impl nsIScriptChannelCoerce for nsIScriptChannel {
    #[inline]
    fn coerce_from(v: &nsIScriptChannel) -> &Self {
        v
    }
}

impl nsIScriptChannel {
    #[inline]
    pub fn coerce<T: nsIScriptChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScriptChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptChannelVTable {
    pub __base: nsISupportsVTable,

    /* attribute unsigned long executionPolicy; */
    pub get_executionPolicy: unsafe extern "C" fn (this: *const nsIScriptChannel, aExecutionPolicy: *mut libc::uint32_t) -> nsresult,
    pub set_executionPolicy: unsafe extern "C" fn (this: *const nsIScriptChannel, aExecutionPolicy: libc::uint32_t) -> nsresult,

    /* attribute boolean executeAsync; */
    pub get_executeAsync: unsafe extern "C" fn (this: *const nsIScriptChannel, aExecuteAsync: *mut bool) -> nsresult,
    pub set_executeAsync: unsafe extern "C" fn (this: *const nsIScriptChannel, aExecuteAsync: bool) -> nsresult,

}


impl nsIScriptChannel {
    /* attribute unsigned long executionPolicy; */
    #[inline]
    pub unsafe fn get_executionPolicy(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_executionPolicy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_executionPolicy(&self, aExecutionPolicy: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_executionPolicy)(self as *const _, aExecutionPolicy) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean executeAsync; */
    #[inline]
    pub unsafe fn get_executeAsync(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_executeAsync)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_executeAsync(&self, aExecuteAsync: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_executeAsync)(self as *const _, aExecuteAsync) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


