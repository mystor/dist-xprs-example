//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITextScroll.idl
//


#[repr(C)]
pub struct nsITextScroll {
    vtable: *const nsITextScrollVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITextScroll {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x067b28a0, 0x877f, 0x11d3,
            [0xaf, 0x7e, 0x00, 0xa0, 0x24, 0xff, 0xc0, 0x8c])
    }
}

unsafe impl RefCounted for nsITextScroll {
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
pub trait nsITextScrollCoerce {
    fn coerce_from(v: &nsITextScroll) -> &Self;
}

impl nsITextScrollCoerce for nsITextScroll {
    #[inline]
    fn coerce_from(v: &nsITextScroll) -> &Self {
        v
    }
}

impl nsITextScroll {
    #[inline]
    pub fn coerce<T: nsITextScrollCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITextScroll {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITextScrollCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITextScroll) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITextScrollVTable {
    pub __base: nsISupportsVTable,

    /* void scrollByLines (in long numLines); */
    pub scrollByLines: unsafe extern "C" fn (this: *const nsITextScroll, numLines: libc::int32_t) -> nsresult,

    /* void scrollByPages (in long numPages); */
    pub scrollByPages: unsafe extern "C" fn (this: *const nsITextScroll, numPages: libc::int32_t) -> nsresult,

}


impl nsITextScroll {
    /* void scrollByLines (in long numLines); */
    #[inline]
    pub unsafe fn scrollByLines(&self, numLines: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollByLines)(self as *const _, numLines) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollByPages (in long numPages); */
    #[inline]
    pub unsafe fn scrollByPages(&self, numPages: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollByPages)(self as *const _, numPages) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


