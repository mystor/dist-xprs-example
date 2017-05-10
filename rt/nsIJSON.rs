//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIJSON.idl
//


#[repr(C)]
pub struct nsIJSON {
    vtable: *const nsIJSONVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIJSON {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x083aebb0, 0x7790, 0x43b2,
            [0xae, 0x81, 0x9e, 0x40, 0x4e, 0x62, 0x62, 0x36])
    }
}

unsafe impl RefCounted for nsIJSON {
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
pub trait nsIJSONCoerce {
    fn coerce_from(v: &nsIJSON) -> &Self;
}

impl nsIJSONCoerce for nsIJSON {
    #[inline]
    fn coerce_from(v: &nsIJSON) -> &Self {
        v
    }
}

impl nsIJSON {
    #[inline]
    pub fn coerce<T: nsIJSONCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIJSON {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIJSONCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSON) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIJSONVTable {
    pub __base: nsISupportsVTable,

    /* [deprecated,implicit_jscontext,optional_argc] AString encode ([optional] in jsval value); */
    /// Unable to call function as its signature contains a non-rust type
    pub encode: *const ::libc::c_void,

    /* [deprecated,implicit_jscontext,optional_argc] void encodeToStream (in nsIOutputStream stream, in string charset, in boolean writeBOM, [optional] in jsval value); */
    /// Unable to call function as its signature contains a non-rust type
    pub encodeToStream: *const ::libc::c_void,

    /* [deprecated,implicit_jscontext] jsval decode (in AString str); */
    /// Unable to call function as its signature contains a non-rust type
    pub decode: *const ::libc::c_void,

    /* [implicit_jscontext] jsval decodeFromStream (in nsIInputStream stream, in long contentLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub decodeFromStream: *const ::libc::c_void,

    /* [noscript] AString encodeFromJSVal (in JSValPtr value, in JSContext cx); */
    /// Unable to call function as its signature contains a non-rust type
    pub encodeFromJSVal: *const ::libc::c_void,

    /* [noscript] jsval decodeToJSVal (in AString str, in JSContext cx); */
    /// Unable to call function as its signature contains a non-rust type
    pub decodeToJSVal: *const ::libc::c_void,

}


impl nsIJSON {
    /* [deprecated,implicit_jscontext,optional_argc] AString encode ([optional] in jsval value); */


    /* [deprecated,implicit_jscontext,optional_argc] void encodeToStream (in nsIOutputStream stream, in string charset, in boolean writeBOM, [optional] in jsval value); */


    /* [deprecated,implicit_jscontext] jsval decode (in AString str); */


    /* [implicit_jscontext] jsval decodeFromStream (in nsIInputStream stream, in long contentLength); */


    /* [noscript] AString encodeFromJSVal (in JSValPtr value, in JSContext cx); */


    /* [noscript] jsval decodeToJSVal (in AString str, in JSContext cx); */


}


