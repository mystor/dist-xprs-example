//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISemanticUnitScanner.idl
//


#[repr(C)]
pub struct nsISemanticUnitScanner {
    vtable: *const nsISemanticUnitScannerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISemanticUnitScanner {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9f620be4, 0xe535, 0x11d6,
            [0xb2, 0x54, 0x00, 0x03, 0x93, 0x10, 0xa4, 0x7a])
    }
}

unsafe impl RefCounted for nsISemanticUnitScanner {
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
pub trait nsISemanticUnitScannerCoerce {
    fn coerce_from(v: &nsISemanticUnitScanner) -> &Self;
}

impl nsISemanticUnitScannerCoerce for nsISemanticUnitScanner {
    #[inline]
    fn coerce_from(v: &nsISemanticUnitScanner) -> &Self {
        v
    }
}

impl nsISemanticUnitScanner {
    #[inline]
    pub fn coerce<T: nsISemanticUnitScannerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISemanticUnitScanner {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISemanticUnitScannerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISemanticUnitScanner) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISemanticUnitScannerVTable {
    pub __base: nsISupportsVTable,

    /* void start (in string characterSet); */
    pub start: unsafe extern "C" fn (this: *const nsISemanticUnitScanner, characterSet: *const libc::c_char) -> nsresult,

    /* boolean next (in wstring text, in long length, in long pos, in boolean isLastBuffer, out long begin, out long end); */
    pub next: unsafe extern "C" fn (this: *const nsISemanticUnitScanner, text: *const libc::int16_t, length: libc::int32_t, pos: libc::int32_t, isLastBuffer: bool, begin: *mut libc::int32_t, end: *mut libc::int32_t, _retval: *mut bool) -> nsresult,

}


impl nsISemanticUnitScanner {
    /* void start (in string characterSet); */
    #[inline]
    pub unsafe fn start(&self, characterSet: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).start)(self as *const _, characterSet) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean next (in wstring text, in long length, in long pos, in boolean isLastBuffer, out long begin, out long end); */
    #[inline]
    pub unsafe fn next(&self, text: *const libc::int16_t, length: libc::int32_t, pos: libc::int32_t, isLastBuffer: bool) -> Result<(libc::int32_t, libc::int32_t, bool), nsresult> {
        let mut begin: libc::int32_t = ::std::mem::zeroed();
        let mut end: libc::int32_t = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).next)(self as *const _, text, length, pos, isLastBuffer, &mut begin as *mut _, &mut end as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((begin, end, _retval))
    }

}


