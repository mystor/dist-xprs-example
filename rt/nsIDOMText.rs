//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMText.idl
//


#[repr(C)]
pub struct nsIDOMText {
    vtable: *const nsIDOMTextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMText {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x67273994, 0x6aff, 0x4091,
            [0x9d, 0xe9, 0xb7, 0x88, 0xa2, 0x49, 0xf7, 0x83])
    }
}

unsafe impl RefCounted for nsIDOMText {
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
pub trait nsIDOMTextCoerce {
    fn coerce_from(v: &nsIDOMText) -> &Self;
}

impl nsIDOMTextCoerce for nsIDOMText {
    #[inline]
    fn coerce_from(v: &nsIDOMText) -> &Self {
        v
    }
}

impl nsIDOMText {
    #[inline]
    pub fn coerce<T: nsIDOMTextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMText {
    type Target = nsIDOMCharacterData;
    #[inline]
    fn deref(&self) -> &nsIDOMCharacterData {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMCharacterDataCoerce> nsIDOMTextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMText) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMTextVTable {
    pub __base: nsIDOMCharacterDataVTable,

    /* nsIDOMText splitText (in unsigned long offset) raises (DOMException); */
    pub splitText: unsafe extern "C" fn (this: *const nsIDOMText, offset: libc::uint32_t, _retval: *mut *const nsIDOMText) -> nsresult,

    /* readonly attribute DOMString wholeText; */
    pub get_wholeText: unsafe extern "C" fn (this: *const nsIDOMText, aWholeText: *mut nsAString) -> nsresult,

}


impl nsIDOMText {
    /* nsIDOMText splitText (in unsigned long offset) raises (DOMException); */
    #[inline]
    pub unsafe fn splitText(&self, offset: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMText>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).splitText)(self as *const _, offset, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString wholeText; */
    #[inline]
    pub unsafe fn get_wholeText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_wholeText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


