//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMSerializer.idl
//


#[repr(C)]
pub struct nsIDOMSerializer {
    vtable: *const nsIDOMSerializerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMSerializer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9fd4ba15, 0xe67c, 0x4c98,
            [0xb5, 0x2c, 0x77, 0x15, 0xf6, 0x2c, 0x91, 0x96])
    }
}

unsafe impl RefCounted for nsIDOMSerializer {
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
pub trait nsIDOMSerializerCoerce {
    fn coerce_from(v: &nsIDOMSerializer) -> &Self;
}

impl nsIDOMSerializerCoerce for nsIDOMSerializer {
    #[inline]
    fn coerce_from(v: &nsIDOMSerializer) -> &Self {
        v
    }
}

impl nsIDOMSerializer {
    #[inline]
    pub fn coerce<T: nsIDOMSerializerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMSerializer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMSerializerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMSerializer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMSerializerVTable {
    pub __base: nsISupportsVTable,

    /* AString serializeToString (in nsIDOMNode root); */
    pub serializeToString: unsafe extern "C" fn (this: *const nsIDOMSerializer, root: *const nsIDOMNode, _retval: *mut nsAString) -> nsresult,

    /* void serializeToStream (in nsIDOMNode root, in nsIOutputStream stream, in AUTF8String charset); */
    pub serializeToStream: unsafe extern "C" fn (this: *const nsIDOMSerializer, root: *const nsIDOMNode, stream: *const nsIOutputStream, charset: *const nsACString) -> nsresult,

}


impl nsIDOMSerializer {
    /* AString serializeToString (in nsIDOMNode root); */
    #[inline]
    pub unsafe fn serializeToString(&self, root: Option<&nsIDOMNode>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).serializeToString)(self as *const _, root.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void serializeToStream (in nsIDOMNode root, in nsIOutputStream stream, in AUTF8String charset); */
    #[inline]
    pub unsafe fn serializeToStream(&self, root: Option<&nsIDOMNode>, stream: Option<&nsIOutputStream>, charset: &[u8]) -> Result<(), nsresult> {
        let charset = nsCString::from(charset);
        match ((*self.vtable).serializeToStream)(self as *const _, root.map_or(::std::ptr::null(), |x| x as *const _), stream.map_or(::std::ptr::null(), |x| x as *const _), &*charset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


