//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozITXTToHTMLConv.idl
//


pub mod mozITXTToHTMLConv_consts {
    pub const kEntities: i64 = 0;
    pub const kURLs: i64 = 2;
    pub const kGlyphSubstitution: i64 = 4;
    pub const kStructPhrase: i64 = 8;
}


#[repr(C)]
pub struct mozITXTToHTMLConv {
    vtable: *const mozITXTToHTMLConvVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozITXTToHTMLConv {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x77c0e42a, 0x1dd2, 0x11b2,
            [0x8e, 0xbf, 0xed, 0xc6, 0x60, 0x6f, 0x2f, 0x4b])
    }
}

unsafe impl RefCounted for mozITXTToHTMLConv {
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
pub trait mozITXTToHTMLConvCoerce {
    fn coerce_from(v: &mozITXTToHTMLConv) -> &Self;
}

impl mozITXTToHTMLConvCoerce for mozITXTToHTMLConv {
    #[inline]
    fn coerce_from(v: &mozITXTToHTMLConv) -> &Self {
        v
    }
}

impl mozITXTToHTMLConv {
    #[inline]
    pub fn coerce<T: mozITXTToHTMLConvCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozITXTToHTMLConv {
    type Target = nsIStreamConverter;
    #[inline]
    fn deref(&self) -> &nsIStreamConverter {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamConverterCoerce> mozITXTToHTMLConvCoerce for T {
    #[inline]
    fn coerce_from(v: &mozITXTToHTMLConv) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozITXTToHTMLConvVTable {
    pub __base: nsIStreamConverterVTable,

    /* wstring scanTXT (in wstring text, in unsigned long whattodo); */
    pub scanTXT: unsafe extern "C" fn (this: *const mozITXTToHTMLConv, text: *const libc::int16_t, whattodo: libc::uint32_t, _retval: *mut *const libc::int16_t) -> nsresult,

    /* wstring scanHTML (in wstring text, in unsigned long whattodo); */
    pub scanHTML: unsafe extern "C" fn (this: *const mozITXTToHTMLConv, text: *const libc::int16_t, whattodo: libc::uint32_t, _retval: *mut *const libc::int16_t) -> nsresult,

    /* unsigned long citeLevelTXT (in wstring line, out unsigned long logLineStart); */
    pub citeLevelTXT: unsafe extern "C" fn (this: *const mozITXTToHTMLConv, line: *const libc::int16_t, logLineStart: *mut libc::uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* void findURLInPlaintext (in wstring text, in long aLength, in long aPos, out long aStartPos, out long aEndPos); */
    pub findURLInPlaintext: unsafe extern "C" fn (this: *const mozITXTToHTMLConv, text: *const libc::int16_t, aLength: libc::int32_t, aPos: libc::int32_t, aStartPos: *mut libc::int32_t, aEndPos: *mut libc::int32_t) -> nsresult,

}


impl mozITXTToHTMLConv {
    /* wstring scanTXT (in wstring text, in unsigned long whattodo); */
    #[inline]
    pub unsafe fn scanTXT(&self, text: *const libc::int16_t, whattodo: libc::uint32_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).scanTXT)(self as *const _, text, whattodo, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* wstring scanHTML (in wstring text, in unsigned long whattodo); */
    #[inline]
    pub unsafe fn scanHTML(&self, text: *const libc::int16_t, whattodo: libc::uint32_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).scanHTML)(self as *const _, text, whattodo, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long citeLevelTXT (in wstring line, out unsigned long logLineStart); */
    #[inline]
    pub unsafe fn citeLevelTXT(&self, line: *const libc::int16_t) -> Result<(libc::uint32_t, libc::uint32_t), nsresult> {
        let mut logLineStart: libc::uint32_t = ::std::mem::zeroed();
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).citeLevelTXT)(self as *const _, line, &mut logLineStart as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((logLineStart, _retval))
    }

    /* void findURLInPlaintext (in wstring text, in long aLength, in long aPos, out long aStartPos, out long aEndPos); */
    #[inline]
    pub unsafe fn findURLInPlaintext(&self, text: *const libc::int16_t, aLength: libc::int32_t, aPos: libc::int32_t) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut aStartPos: libc::int32_t = ::std::mem::zeroed();
        let mut aEndPos: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).findURLInPlaintext)(self as *const _, text, aLength, aPos, &mut aStartPos as *mut _, &mut aEndPos as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aStartPos, aEndPos))
    }

}


