//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozISpellCheckingEngine.idl
//


#[repr(C)]
pub struct mozISpellCheckingEngine {
    vtable: *const mozISpellCheckingEngineVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozISpellCheckingEngine {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8ba643a4, 0x7ddc, 0x4662,
            [0xb9, 0x76, 0x7e, 0xc1, 0x23, 0x84, 0x3f, 0x10])
    }
}

unsafe impl RefCounted for mozISpellCheckingEngine {
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
pub trait mozISpellCheckingEngineCoerce {
    fn coerce_from(v: &mozISpellCheckingEngine) -> &Self;
}

impl mozISpellCheckingEngineCoerce for mozISpellCheckingEngine {
    #[inline]
    fn coerce_from(v: &mozISpellCheckingEngine) -> &Self {
        v
    }
}

impl mozISpellCheckingEngine {
    #[inline]
    pub fn coerce<T: mozISpellCheckingEngineCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozISpellCheckingEngine {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozISpellCheckingEngineCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISpellCheckingEngine) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozISpellCheckingEngineVTable {
    pub __base: nsISupportsVTable,

    /* attribute wstring dictionary; */
    pub get_dictionary: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, aDictionary: *mut *const libc::int16_t) -> nsresult,
    pub set_dictionary: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, aDictionary: *const libc::int16_t) -> nsresult,

    /* readonly attribute wstring language; */
    pub get_language: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, aLanguage: *mut *const libc::int16_t) -> nsresult,

    /* readonly attribute boolean providesPersonalDictionary; */
    pub get_providesPersonalDictionary: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, aProvidesPersonalDictionary: *mut bool) -> nsresult,

    /* readonly attribute boolean providesWordUtils; */
    pub get_providesWordUtils: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, aProvidesWordUtils: *mut bool) -> nsresult,

    /* readonly attribute wstring name; */
    pub get_name: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, aName: *mut *const libc::int16_t) -> nsresult,

    /* readonly attribute wstring copyright; */
    pub get_copyright: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, aCopyright: *mut *const libc::int16_t) -> nsresult,

    /* attribute mozIPersonalDictionary personalDictionary; */
    pub get_personalDictionary: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, aPersonalDictionary: *mut *const mozIPersonalDictionary) -> nsresult,
    pub set_personalDictionary: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, aPersonalDictionary: *const mozIPersonalDictionary) -> nsresult,

    /* void getDictionaryList ([array, size_is (count)] out wstring dictionaries, out uint32_t count); */
    /// Unable to call function as its signature contains a non-rust type
    pub getDictionaryList: *const ::libc::c_void,

    /* boolean check (in wstring word); */
    pub check: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, word: *const libc::int16_t, _retval: *mut bool) -> nsresult,

    /* void suggest (in wstring word, [array, size_is (count)] out wstring suggestions, out uint32_t count); */
    /// Unable to call function as its signature contains a non-rust type
    pub suggest: *const ::libc::c_void,

    /* void loadDictionariesFromDir (in nsIFile dir); */
    pub loadDictionariesFromDir: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, dir: *const nsIFile) -> nsresult,

    /* void addDirectory (in nsIFile dir); */
    pub addDirectory: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, dir: *const nsIFile) -> nsresult,

    /* void removeDirectory (in nsIFile dir); */
    pub removeDirectory: unsafe extern "C" fn (this: *const mozISpellCheckingEngine, dir: *const nsIFile) -> nsresult,

}


impl mozISpellCheckingEngine {
    /* attribute wstring dictionary; */
    #[inline]
    pub unsafe fn get_dictionary(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_dictionary)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_dictionary(&self, aDictionary: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_dictionary)(self as *const _, aDictionary) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* readonly attribute boolean providesPersonalDictionary; */
    #[inline]
    pub unsafe fn get_providesPersonalDictionary(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_providesPersonalDictionary)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean providesWordUtils; */
    #[inline]
    pub unsafe fn get_providesWordUtils(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_providesWordUtils)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute wstring name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_name)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute wstring copyright; */
    #[inline]
    pub unsafe fn get_copyright(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_copyright)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute mozIPersonalDictionary personalDictionary; */
    #[inline]
    pub unsafe fn get_personalDictionary(&self, ) -> Result<Option<RefPtr<mozIPersonalDictionary>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_personalDictionary)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_personalDictionary(&self, aPersonalDictionary: Option<&mozIPersonalDictionary>) -> Result<(), nsresult> {

        match ((*self.vtable).set_personalDictionary)(self as *const _, aPersonalDictionary.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getDictionaryList ([array, size_is (count)] out wstring dictionaries, out uint32_t count); */


    /* boolean check (in wstring word); */
    #[inline]
    pub unsafe fn check(&self, word: *const libc::int16_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).check)(self as *const _, word, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void suggest (in wstring word, [array, size_is (count)] out wstring suggestions, out uint32_t count); */


    /* void loadDictionariesFromDir (in nsIFile dir); */
    #[inline]
    pub unsafe fn loadDictionariesFromDir(&self, dir: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).loadDictionariesFromDir)(self as *const _, dir.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addDirectory (in nsIFile dir); */
    #[inline]
    pub unsafe fn addDirectory(&self, dir: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).addDirectory)(self as *const _, dir.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDirectory (in nsIFile dir); */
    #[inline]
    pub unsafe fn removeDirectory(&self, dir: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).removeDirectory)(self as *const _, dir.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


