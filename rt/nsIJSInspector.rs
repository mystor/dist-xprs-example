//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIJSInspector.idl
//


#[repr(C)]
pub struct nsIJSInspector {
    vtable: *const nsIJSInspectorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIJSInspector {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6758d0d7, 0xe96a, 0x4c5c,
            [0xbc, 0xa8, 0x3b, 0xcb, 0xe5, 0xa1, 0x59, 0x43])
    }
}

unsafe impl RefCounted for nsIJSInspector {
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
pub trait nsIJSInspectorCoerce {
    fn coerce_from(v: &nsIJSInspector) -> &Self;
}

impl nsIJSInspectorCoerce for nsIJSInspector {
    #[inline]
    fn coerce_from(v: &nsIJSInspector) -> &Self {
        v
    }
}

impl nsIJSInspector {
    #[inline]
    pub fn coerce<T: nsIJSInspectorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIJSInspector {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIJSInspectorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSInspector) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIJSInspectorVTable {
    pub __base: nsISupportsVTable,

    /* unsigned long enterNestedEventLoop (in jsval requestor); */
    /// Unable to call function as its signature contains a non-rust type
    pub enterNestedEventLoop: *const ::libc::c_void,

    /* unsigned long exitNestedEventLoop (); */
    pub exitNestedEventLoop: unsafe extern "C" fn (this: *const nsIJSInspector, _retval: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long eventLoopNestLevel; */
    pub get_eventLoopNestLevel: unsafe extern "C" fn (this: *const nsIJSInspector, aEventLoopNestLevel: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute jsval lastNestRequestor; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_lastNestRequestor: *const ::libc::c_void,

}


impl nsIJSInspector {
    /* unsigned long enterNestedEventLoop (in jsval requestor); */


    /* unsigned long exitNestedEventLoop (); */
    #[inline]
    pub unsafe fn exitNestedEventLoop(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).exitNestedEventLoop)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long eventLoopNestLevel; */
    #[inline]
    pub unsafe fn get_eventLoopNestLevel(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_eventLoopNestLevel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute jsval lastNestRequestor; */


}


