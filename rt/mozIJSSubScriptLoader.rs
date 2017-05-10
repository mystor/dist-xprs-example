//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIJSSubScriptLoader.idl
//


#[repr(C)]
pub struct mozIJSSubScriptLoader {
    vtable: *const mozIJSSubScriptLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIJSSubScriptLoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x19533e7b, 0xf321, 0x4ef1,
            [0xbc, 0x59, 0x6e, 0x81, 0x2d, 0xc2, 0xa7, 0x33])
    }
}

unsafe impl RefCounted for mozIJSSubScriptLoader {
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
pub trait mozIJSSubScriptLoaderCoerce {
    fn coerce_from(v: &mozIJSSubScriptLoader) -> &Self;
}

impl mozIJSSubScriptLoaderCoerce for mozIJSSubScriptLoader {
    #[inline]
    fn coerce_from(v: &mozIJSSubScriptLoader) -> &Self {
        v
    }
}

impl mozIJSSubScriptLoader {
    #[inline]
    pub fn coerce<T: mozIJSSubScriptLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIJSSubScriptLoader {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIJSSubScriptLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIJSSubScriptLoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIJSSubScriptLoaderVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] jsval loadSubScript (in AString url, [optional] in jsval obj, [optional] in AString charset); */
    /// Unable to call function as its signature contains a non-rust type
    pub loadSubScript: *const ::libc::c_void,

    /* [implicit_jscontext] jsval loadSubScriptWithOptions (in AString url, in jsval options); */
    /// Unable to call function as its signature contains a non-rust type
    pub loadSubScriptWithOptions: *const ::libc::c_void,

    /* void precompileScript (in nsIURI uri, in nsIPrincipal principal, in nsIObserver observer); */
    pub precompileScript: unsafe extern "C" fn (this: *const mozIJSSubScriptLoader, uri: *const nsIURI, principal: *const nsIPrincipal, observer: *const nsIObserver) -> nsresult,

}


impl mozIJSSubScriptLoader {
    /* [implicit_jscontext] jsval loadSubScript (in AString url, [optional] in jsval obj, [optional] in AString charset); */


    /* [implicit_jscontext] jsval loadSubScriptWithOptions (in AString url, in jsval options); */


    /* void precompileScript (in nsIURI uri, in nsIPrincipal principal, in nsIObserver observer); */
    #[inline]
    pub unsafe fn precompileScript(&self, uri: Option<&nsIURI>, principal: Option<&nsIPrincipal>, observer: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).precompileScript)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), principal.map_or(::std::ptr::null(), |x| x as *const _), observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


