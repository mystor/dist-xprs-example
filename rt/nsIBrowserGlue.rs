//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserGlue.idl
//


#[repr(C)]
pub struct nsIBrowserGlue {
    vtable: *const nsIBrowserGlueVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowserGlue {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb0e7c156, 0xd00c, 0x4605,
            [0xa7, 0x7d, 0x27, 0xc7, 0x41, 0x8f, 0x23, 0xae])
    }
}

unsafe impl RefCounted for nsIBrowserGlue {
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
pub trait nsIBrowserGlueCoerce {
    fn coerce_from(v: &nsIBrowserGlue) -> &Self;
}

impl nsIBrowserGlueCoerce for nsIBrowserGlue {
    #[inline]
    fn coerce_from(v: &nsIBrowserGlue) -> &Self {
        v
    }
}

impl nsIBrowserGlue {
    #[inline]
    pub fn coerce<T: nsIBrowserGlueCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowserGlue {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBrowserGlueCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserGlue) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserGlueVTable {
    pub __base: nsISupportsVTable,

    /* void sanitize (in nsIDOMWindow aParentWindow); */
    pub sanitize: unsafe extern "C" fn (this: *const nsIBrowserGlue, aParentWindow: *const nsIDOMWindow) -> nsresult,

}


impl nsIBrowserGlue {
    /* void sanitize (in nsIDOMWindow aParentWindow); */
    #[inline]
    pub unsafe fn sanitize(&self, aParentWindow: Option<&nsIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).sanitize)(self as *const _, aParentWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


