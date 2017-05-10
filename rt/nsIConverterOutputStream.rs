//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIConverterOutputStream.idl
//


#[repr(C)]
pub struct nsIConverterOutputStream {
    vtable: *const nsIConverterOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIConverterOutputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4b71113a, 0xcb0d, 0x479f,
            [0x8e, 0xd5, 0x01, 0xda, 0xeb, 0xa2, 0xe8, 0xd4])
    }
}

unsafe impl RefCounted for nsIConverterOutputStream {
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
pub trait nsIConverterOutputStreamCoerce {
    fn coerce_from(v: &nsIConverterOutputStream) -> &Self;
}

impl nsIConverterOutputStreamCoerce for nsIConverterOutputStream {
    #[inline]
    fn coerce_from(v: &nsIConverterOutputStream) -> &Self {
        v
    }
}

impl nsIConverterOutputStream {
    #[inline]
    pub fn coerce<T: nsIConverterOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIConverterOutputStream {
    type Target = nsIUnicharOutputStream;
    #[inline]
    fn deref(&self) -> &nsIUnicharOutputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIUnicharOutputStreamCoerce> nsIConverterOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConverterOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIConverterOutputStreamVTable {
    pub __base: nsIUnicharOutputStreamVTable,

    /* void init (in nsIOutputStream aOutStream, in string aCharset, in unsigned long aBufferSize, in char16_t aReplacementCharacter); */
    pub init: unsafe extern "C" fn (this: *const nsIConverterOutputStream, aOutStream: *const nsIOutputStream, aCharset: *const libc::c_char, aBufferSize: libc::uint32_t, aReplacementCharacter: char16_t) -> nsresult,

}


impl nsIConverterOutputStream {
    /* void init (in nsIOutputStream aOutStream, in string aCharset, in unsigned long aBufferSize, in char16_t aReplacementCharacter); */
    #[inline]
    pub unsafe fn init(&self, aOutStream: Option<&nsIOutputStream>, aCharset: *const libc::c_char, aBufferSize: libc::uint32_t, aReplacementCharacter: char16_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aOutStream.map_or(::std::ptr::null(), |x| x as *const _), aCharset, aBufferSize, aReplacementCharacter) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


