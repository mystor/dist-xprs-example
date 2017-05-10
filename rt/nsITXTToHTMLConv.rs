//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITXTToHTMLConv.idl
//


#[repr(C)]
pub struct nsITXTToHTMLConv {
    vtable: *const nsITXTToHTMLConvVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITXTToHTMLConv {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x933355f6, 0x1dd2, 0x11b2,
            [0xa9, 0xb0, 0xd3, 0x35, 0xb9, 0xe3, 0x59, 0x83])
    }
}

unsafe impl RefCounted for nsITXTToHTMLConv {
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
pub trait nsITXTToHTMLConvCoerce {
    fn coerce_from(v: &nsITXTToHTMLConv) -> &Self;
}

impl nsITXTToHTMLConvCoerce for nsITXTToHTMLConv {
    #[inline]
    fn coerce_from(v: &nsITXTToHTMLConv) -> &Self {
        v
    }
}

impl nsITXTToHTMLConv {
    #[inline]
    pub fn coerce<T: nsITXTToHTMLConvCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITXTToHTMLConv {
    type Target = nsIStreamConverter;
    #[inline]
    fn deref(&self) -> &nsIStreamConverter {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamConverterCoerce> nsITXTToHTMLConvCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITXTToHTMLConv) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITXTToHTMLConvVTable {
    pub __base: nsIStreamConverterVTable,

    /* void setTitle (in wstring text); */
    pub setTitle: unsafe extern "C" fn (this: *const nsITXTToHTMLConv, text: *const libc::int16_t) -> nsresult,

    /* void preFormatHTML (in boolean value); */
    pub preFormatHTML: unsafe extern "C" fn (this: *const nsITXTToHTMLConv, value: bool) -> nsresult,

}


impl nsITXTToHTMLConv {
    /* void setTitle (in wstring text); */
    #[inline]
    pub unsafe fn setTitle(&self, text: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setTitle)(self as *const _, text) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void preFormatHTML (in boolean value); */
    #[inline]
    pub unsafe fn preFormatHTML(&self, value: bool) -> Result<(), nsresult> {

        match ((*self.vtable).preFormatHTML)(self as *const _, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


