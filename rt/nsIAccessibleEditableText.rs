//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleEditableText.idl
//


#[repr(C)]
pub struct nsIAccessibleEditableText {
    vtable: *const nsIAccessibleEditableTextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleEditableText {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x28915cca, 0x3366, 0x4034,
            [0xba, 0x1d, 0xb7, 0xaf, 0xb9, 0xb3, 0x76, 0x39])
    }
}

unsafe impl RefCounted for nsIAccessibleEditableText {
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
pub trait nsIAccessibleEditableTextCoerce {
    fn coerce_from(v: &nsIAccessibleEditableText) -> &Self;
}

impl nsIAccessibleEditableTextCoerce for nsIAccessibleEditableText {
    #[inline]
    fn coerce_from(v: &nsIAccessibleEditableText) -> &Self {
        v
    }
}

impl nsIAccessibleEditableText {
    #[inline]
    pub fn coerce<T: nsIAccessibleEditableTextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleEditableText {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleEditableTextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleEditableText) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleEditableTextVTable {
    pub __base: nsISupportsVTable,

    /* void setTextContents (in AString text); */
    pub setTextContents: unsafe extern "C" fn (this: *const nsIAccessibleEditableText, text: *const nsAString) -> nsresult,

    /* void insertText (in AString text, in long position); */
    pub insertText: unsafe extern "C" fn (this: *const nsIAccessibleEditableText, text: *const nsAString, position: libc::int32_t) -> nsresult,

    /* void copyText (in long startPos, in long endPos); */
    pub copyText: unsafe extern "C" fn (this: *const nsIAccessibleEditableText, startPos: libc::int32_t, endPos: libc::int32_t) -> nsresult,

    /* void cutText (in long startPos, in long endPos); */
    pub cutText: unsafe extern "C" fn (this: *const nsIAccessibleEditableText, startPos: libc::int32_t, endPos: libc::int32_t) -> nsresult,

    /* void deleteText (in long startPos, in long endPos); */
    pub deleteText: unsafe extern "C" fn (this: *const nsIAccessibleEditableText, startPos: libc::int32_t, endPos: libc::int32_t) -> nsresult,

    /* void pasteText (in long position); */
    pub pasteText: unsafe extern "C" fn (this: *const nsIAccessibleEditableText, position: libc::int32_t) -> nsresult,

}


impl nsIAccessibleEditableText {
    /* void setTextContents (in AString text); */
    #[inline]
    pub unsafe fn setTextContents(&self, text: &[u16]) -> Result<(), nsresult> {
        let text = nsString::from(text);
        match ((*self.vtable).setTextContents)(self as *const _, &*text) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertText (in AString text, in long position); */
    #[inline]
    pub unsafe fn insertText(&self, text: &[u16], position: libc::int32_t) -> Result<(), nsresult> {
        let text = nsString::from(text);
        match ((*self.vtable).insertText)(self as *const _, &*text, position) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void copyText (in long startPos, in long endPos); */
    #[inline]
    pub unsafe fn copyText(&self, startPos: libc::int32_t, endPos: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).copyText)(self as *const _, startPos, endPos) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cutText (in long startPos, in long endPos); */
    #[inline]
    pub unsafe fn cutText(&self, startPos: libc::int32_t, endPos: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).cutText)(self as *const _, startPos, endPos) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteText (in long startPos, in long endPos); */
    #[inline]
    pub unsafe fn deleteText(&self, startPos: libc::int32_t, endPos: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).deleteText)(self as *const _, startPos, endPos) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void pasteText (in long position); */
    #[inline]
    pub unsafe fn pasteText(&self, position: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).pasteText)(self as *const _, position) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


