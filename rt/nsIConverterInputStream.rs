//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIConverterInputStream.idl
//


pub mod nsIConverterInputStream_consts {
    pub const DEFAULT_REPLACEMENT_CHARACTER: i64 = 65533;
}


#[repr(C)]
pub struct nsIConverterInputStream {
    vtable: *const nsIConverterInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIConverterInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfc66ffb6, 0x5404, 0x4908,
            [0xa4, 0xa3, 0x27, 0xf9, 0x2f, 0xa0, 0x57, 0x9d])
    }
}

unsafe impl RefCounted for nsIConverterInputStream {
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
pub trait nsIConverterInputStreamCoerce {
    fn coerce_from(v: &nsIConverterInputStream) -> &Self;
}

impl nsIConverterInputStreamCoerce for nsIConverterInputStream {
    #[inline]
    fn coerce_from(v: &nsIConverterInputStream) -> &Self {
        v
    }
}

impl nsIConverterInputStream {
    #[inline]
    pub fn coerce<T: nsIConverterInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIConverterInputStream {
    type Target = nsIUnicharInputStream;
    #[inline]
    fn deref(&self) -> &nsIUnicharInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIUnicharInputStreamCoerce> nsIConverterInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConverterInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIConverterInputStreamVTable {
    pub __base: nsIUnicharInputStreamVTable,

    /* void init (in nsIInputStream aStream, in string aCharset, in long aBufferSize, in char16_t aReplacementChar); */
    pub init: unsafe extern "C" fn (this: *const nsIConverterInputStream, aStream: *const nsIInputStream, aCharset: *const libc::c_char, aBufferSize: libc::int32_t, aReplacementChar: char16_t) -> nsresult,

}


impl nsIConverterInputStream {
    /* void init (in nsIInputStream aStream, in string aCharset, in long aBufferSize, in char16_t aReplacementChar); */
    #[inline]
    pub unsafe fn init(&self, aStream: Option<&nsIInputStream>, aCharset: *const libc::c_char, aBufferSize: libc::int32_t, aReplacementChar: char16_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), aCharset, aBufferSize, aReplacementChar) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


