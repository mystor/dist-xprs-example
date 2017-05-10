//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMWebGLRenderingContext.idl
//


#[repr(C)]
pub struct nsIDOMWebGLRenderingContext {
    vtable: *const nsIDOMWebGLRenderingContextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMWebGLRenderingContext {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf1a2fd3a, 0xc6ac, 0x4ee2,
            [0xa7, 0x00, 0x5d, 0x28, 0x5d, 0x5e, 0x3f, 0xff])
    }
}

unsafe impl RefCounted for nsIDOMWebGLRenderingContext {
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
pub trait nsIDOMWebGLRenderingContextCoerce {
    fn coerce_from(v: &nsIDOMWebGLRenderingContext) -> &Self;
}

impl nsIDOMWebGLRenderingContextCoerce for nsIDOMWebGLRenderingContext {
    #[inline]
    fn coerce_from(v: &nsIDOMWebGLRenderingContext) -> &Self {
        v
    }
}

impl nsIDOMWebGLRenderingContext {
    #[inline]
    pub fn coerce<T: nsIDOMWebGLRenderingContextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMWebGLRenderingContext {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMWebGLRenderingContextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMWebGLRenderingContext) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMWebGLRenderingContextVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] DOMString mozGetUnderlyingParamString (in unsigned long pname); */
    pub mozGetUnderlyingParamString: unsafe extern "C" fn (this: *const nsIDOMWebGLRenderingContext, pname: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

}


impl nsIDOMWebGLRenderingContext {
    /* [noscript] DOMString mozGetUnderlyingParamString (in unsigned long pname); */
    #[inline]
    pub unsafe fn mozGetUnderlyingParamString(&self, pname: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).mozGetUnderlyingParamString)(self as *const _, pname, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


