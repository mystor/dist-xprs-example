//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIPersonalDictionary.idl
//


#[repr(C)]
pub struct mozIPersonalDictionary {
    vtable: *const mozIPersonalDictionaryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIPersonalDictionary {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7ef52eaf, 0xb7e1, 0x462b,
            [0x87, 0xe2, 0x5d, 0x1d, 0xba, 0xca, 0x90, 0x48])
    }
}

unsafe impl RefCounted for mozIPersonalDictionary {
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
pub trait mozIPersonalDictionaryCoerce {
    fn coerce_from(v: &mozIPersonalDictionary) -> &Self;
}

impl mozIPersonalDictionaryCoerce for mozIPersonalDictionary {
    #[inline]
    fn coerce_from(v: &mozIPersonalDictionary) -> &Self {
        v
    }
}

impl mozIPersonalDictionary {
    #[inline]
    pub fn coerce<T: mozIPersonalDictionaryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIPersonalDictionary {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIPersonalDictionaryCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIPersonalDictionary) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIPersonalDictionaryVTable {
    pub __base: nsISupportsVTable,

    /* void load (); */
    pub load: unsafe extern "C" fn (this: *const mozIPersonalDictionary) -> nsresult,

    /* void save (); */
    pub save: unsafe extern "C" fn (this: *const mozIPersonalDictionary) -> nsresult,

    /* readonly attribute nsIStringEnumerator wordList; */
    pub get_wordList: unsafe extern "C" fn (this: *const mozIPersonalDictionary, aWordList: *mut *const nsIStringEnumerator) -> nsresult,

    /* boolean check (in wstring word, in wstring lang); */
    pub check: unsafe extern "C" fn (this: *const mozIPersonalDictionary, word: *const libc::int16_t, lang: *const libc::int16_t, _retval: *mut bool) -> nsresult,

    /* void addWord (in wstring word, in wstring lang); */
    pub addWord: unsafe extern "C" fn (this: *const mozIPersonalDictionary, word: *const libc::int16_t, lang: *const libc::int16_t) -> nsresult,

    /* void removeWord (in wstring word, in wstring lang); */
    pub removeWord: unsafe extern "C" fn (this: *const mozIPersonalDictionary, word: *const libc::int16_t, lang: *const libc::int16_t) -> nsresult,

    /* void ignoreWord (in wstring word); */
    pub ignoreWord: unsafe extern "C" fn (this: *const mozIPersonalDictionary, word: *const libc::int16_t) -> nsresult,

    /* void endSession (); */
    pub endSession: unsafe extern "C" fn (this: *const mozIPersonalDictionary) -> nsresult,

    /* void addCorrection (in wstring word, in wstring correction, in wstring lang); */
    pub addCorrection: unsafe extern "C" fn (this: *const mozIPersonalDictionary, word: *const libc::int16_t, correction: *const libc::int16_t, lang: *const libc::int16_t) -> nsresult,

    /* void removeCorrection (in wstring word, in wstring correction, in wstring lang); */
    pub removeCorrection: unsafe extern "C" fn (this: *const mozIPersonalDictionary, word: *const libc::int16_t, correction: *const libc::int16_t, lang: *const libc::int16_t) -> nsresult,

    /* void getCorrection (in wstring word, [array, size_is (count)] out wstring words, out uint32_t count); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCorrection: *const ::libc::c_void,

}


impl mozIPersonalDictionary {
    /* void load (); */
    #[inline]
    pub unsafe fn load(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).load)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void save (); */
    #[inline]
    pub unsafe fn save(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).save)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIStringEnumerator wordList; */
    #[inline]
    pub unsafe fn get_wordList(&self, ) -> Result<Option<RefPtr<nsIStringEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_wordList)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean check (in wstring word, in wstring lang); */
    #[inline]
    pub unsafe fn check(&self, word: *const libc::int16_t, lang: *const libc::int16_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).check)(self as *const _, word, lang, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addWord (in wstring word, in wstring lang); */
    #[inline]
    pub unsafe fn addWord(&self, word: *const libc::int16_t, lang: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).addWord)(self as *const _, word, lang) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeWord (in wstring word, in wstring lang); */
    #[inline]
    pub unsafe fn removeWord(&self, word: *const libc::int16_t, lang: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeWord)(self as *const _, word, lang) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ignoreWord (in wstring word); */
    #[inline]
    pub unsafe fn ignoreWord(&self, word: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).ignoreWord)(self as *const _, word) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endSession (); */
    #[inline]
    pub unsafe fn endSession(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endSession)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addCorrection (in wstring word, in wstring correction, in wstring lang); */
    #[inline]
    pub unsafe fn addCorrection(&self, word: *const libc::int16_t, correction: *const libc::int16_t, lang: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).addCorrection)(self as *const _, word, correction, lang) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeCorrection (in wstring word, in wstring correction, in wstring lang); */
    #[inline]
    pub unsafe fn removeCorrection(&self, word: *const libc::int16_t, correction: *const libc::int16_t, lang: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeCorrection)(self as *const _, word, correction, lang) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getCorrection (in wstring word, [array, size_is (count)] out wstring words, out uint32_t count); */


}


