//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozISpellI18NUtil.idl
//


pub mod mozISpellI18NUtil_consts {
    pub const kCheck: i64 = 0;
    pub const kSuggest: i64 = 1;
}


#[repr(C)]
pub struct mozISpellI18NUtil {
    vtable: *const mozISpellI18NUtilVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozISpellI18NUtil {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb075d5dc, 0x1df1, 0x441a,
            [0xbe, 0xbf, 0x68, 0x0d, 0x8c, 0xaa, 0xa1, 0x9c])
    }
}

unsafe impl RefCounted for mozISpellI18NUtil {
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
pub trait mozISpellI18NUtilCoerce {
    fn coerce_from(v: &mozISpellI18NUtil) -> &Self;
}

impl mozISpellI18NUtilCoerce for mozISpellI18NUtil {
    #[inline]
    fn coerce_from(v: &mozISpellI18NUtil) -> &Self {
        v
    }
}

impl mozISpellI18NUtil {
    #[inline]
    pub fn coerce<T: mozISpellI18NUtilCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozISpellI18NUtil {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozISpellI18NUtilCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISpellI18NUtil) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozISpellI18NUtilVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute wstring language; */
    pub get_language: unsafe extern "C" fn (this: *const mozISpellI18NUtil, aLanguage: *mut *const libc::int16_t) -> nsresult,

    /* void getRootForm (in wstring word, in uint32_t type, [array, size_is (count)] out wstring words, out uint32_t count); */
    /// Unable to call function as its signature contains a non-rust type
    pub getRootForm: *const ::libc::c_void,

    /* void fromRootForm (in wstring word, [array, size_is (icount)] in wstring iwords, in uint32_t icount, [array, size_is (ocount)] out wstring owords, out uint32_t ocount); */
    /// Unable to call function as its signature contains a non-rust type
    pub fromRootForm: *const ::libc::c_void,

    /* void findNextWord (in wstring word, in uint32_t length, in uint32_t offset, out int32_t begin, out int32_t end); */
    pub findNextWord: unsafe extern "C" fn (this: *const mozISpellI18NUtil, word: *const libc::int16_t, length: uint32_t, offset: uint32_t, begin: *mut int32_t, end: *mut int32_t) -> nsresult,

}


impl mozISpellI18NUtil {
    /* readonly attribute wstring language; */
    #[inline]
    pub unsafe fn get_language(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_language)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getRootForm (in wstring word, in uint32_t type, [array, size_is (count)] out wstring words, out uint32_t count); */


    /* void fromRootForm (in wstring word, [array, size_is (icount)] in wstring iwords, in uint32_t icount, [array, size_is (ocount)] out wstring owords, out uint32_t ocount); */


    /* void findNextWord (in wstring word, in uint32_t length, in uint32_t offset, out int32_t begin, out int32_t end); */
    #[inline]
    pub unsafe fn findNextWord(&self, word: *const libc::int16_t, length: uint32_t, offset: uint32_t) -> Result<(int32_t, int32_t), nsresult> {
        let mut begin: int32_t = ::std::mem::zeroed();
        let mut end: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).findNextWord)(self as *const _, word, length, offset, &mut begin as *mut _, &mut end as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((begin, end))
    }

}


