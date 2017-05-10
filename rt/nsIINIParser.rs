//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIINIParser.idl
//


#[repr(C)]
pub struct nsIINIParser {
    vtable: *const nsIINIParserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIINIParser {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7eb955f6, 0x3e78, 0x4d39,
            [0xb7, 0x2f, 0xc1, 0xbf, 0x12, 0xa9, 0x4b, 0xce])
    }
}

unsafe impl RefCounted for nsIINIParser {
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
pub trait nsIINIParserCoerce {
    fn coerce_from(v: &nsIINIParser) -> &Self;
}

impl nsIINIParserCoerce for nsIINIParser {
    #[inline]
    fn coerce_from(v: &nsIINIParser) -> &Self {
        v
    }
}

impl nsIINIParser {
    #[inline]
    pub fn coerce<T: nsIINIParserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIINIParser {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIINIParserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIINIParser) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIINIParserVTable {
    pub __base: nsISupportsVTable,

    /* nsIUTF8StringEnumerator getSections (); */
    pub getSections: unsafe extern "C" fn (this: *const nsIINIParser, _retval: *mut *const nsIUTF8StringEnumerator) -> nsresult,

    /* nsIUTF8StringEnumerator getKeys (in AUTF8String aSection); */
    pub getKeys: unsafe extern "C" fn (this: *const nsIINIParser, aSection: *const nsACString, _retval: *mut *const nsIUTF8StringEnumerator) -> nsresult,

    /* AUTF8String getString (in AUTF8String aSection, in AUTF8String aKey); */
    pub getString: unsafe extern "C" fn (this: *const nsIINIParser, aSection: *const nsACString, aKey: *const nsACString, _retval: *mut nsACString) -> nsresult,

}


impl nsIINIParser {
    /* nsIUTF8StringEnumerator getSections (); */
    #[inline]
    pub unsafe fn getSections(&self, ) -> Result<Option<RefPtr<nsIUTF8StringEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSections)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIUTF8StringEnumerator getKeys (in AUTF8String aSection); */
    #[inline]
    pub unsafe fn getKeys(&self, aSection: &[u8]) -> Result<Option<RefPtr<nsIUTF8StringEnumerator>>, nsresult> {
        let aSection = nsCString::from(aSection);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getKeys)(self as *const _, &*aSection, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AUTF8String getString (in AUTF8String aSection, in AUTF8String aKey); */
    #[inline]
    pub unsafe fn getString(&self, aSection: &[u8], aKey: &[u8]) -> Result<nsCString, nsresult> {
        let aSection = nsCString::from(aSection);
        let aKey = nsCString::from(aKey);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getString)(self as *const _, &*aSection, &*aKey, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsIINIParserWriter_consts {
    pub const WRITE_UTF16: i64 = 1;
}


#[repr(C)]
pub struct nsIINIParserWriter {
    vtable: *const nsIINIParserWriterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIINIParserWriter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb67bb24b, 0x31a3, 0x4a6a,
            [0xa5, 0xd9, 0x04, 0x85, 0xc9, 0xaf, 0x5a, 0x04])
    }
}

unsafe impl RefCounted for nsIINIParserWriter {
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
pub trait nsIINIParserWriterCoerce {
    fn coerce_from(v: &nsIINIParserWriter) -> &Self;
}

impl nsIINIParserWriterCoerce for nsIINIParserWriter {
    #[inline]
    fn coerce_from(v: &nsIINIParserWriter) -> &Self {
        v
    }
}

impl nsIINIParserWriter {
    #[inline]
    pub fn coerce<T: nsIINIParserWriterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIINIParserWriter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIINIParserWriterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIINIParserWriter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIINIParserWriterVTable {
    pub __base: nsISupportsVTable,

    /* void setString (in AUTF8String aSection, in AUTF8String aKey, in AUTF8String aValue); */
    pub setString: unsafe extern "C" fn (this: *const nsIINIParserWriter, aSection: *const nsACString, aKey: *const nsACString, aValue: *const nsACString) -> nsresult,

    /* void writeFile ([optional] in nsIFile aINIFile, [optional] in unsigned long aFlags); */
    pub writeFile: unsafe extern "C" fn (this: *const nsIINIParserWriter, aINIFile: *const nsIFile, aFlags: libc::uint32_t) -> nsresult,

}


impl nsIINIParserWriter {
    /* void setString (in AUTF8String aSection, in AUTF8String aKey, in AUTF8String aValue); */
    #[inline]
    pub unsafe fn setString(&self, aSection: &[u8], aKey: &[u8], aValue: &[u8]) -> Result<(), nsresult> {
        let aSection = nsCString::from(aSection);
        let aKey = nsCString::from(aKey);
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).setString)(self as *const _, &*aSection, &*aKey, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void writeFile ([optional] in nsIFile aINIFile, [optional] in unsigned long aFlags); */
    #[inline]
    pub unsafe fn writeFile(&self, aINIFile: Option<&nsIFile>, aFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).writeFile)(self as *const _, aINIFile.map_or(::std::ptr::null(), |x| x as *const _), aFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIINIParserFactory {
    vtable: *const nsIINIParserFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIINIParserFactory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xccae7ea5, 0x1218, 0x4b51,
            [0xae, 0xcb, 0xc2, 0xd8, 0xec, 0xd4, 0x6a, 0xf9])
    }
}

unsafe impl RefCounted for nsIINIParserFactory {
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
pub trait nsIINIParserFactoryCoerce {
    fn coerce_from(v: &nsIINIParserFactory) -> &Self;
}

impl nsIINIParserFactoryCoerce for nsIINIParserFactory {
    #[inline]
    fn coerce_from(v: &nsIINIParserFactory) -> &Self {
        v
    }
}

impl nsIINIParserFactory {
    #[inline]
    pub fn coerce<T: nsIINIParserFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIINIParserFactory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIINIParserFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIINIParserFactory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIINIParserFactoryVTable {
    pub __base: nsISupportsVTable,

    /* nsIINIParser createINIParser (in nsIFile aINIFile); */
    pub createINIParser: unsafe extern "C" fn (this: *const nsIINIParserFactory, aINIFile: *const nsIFile, _retval: *mut *const nsIINIParser) -> nsresult,

}


impl nsIINIParserFactory {
    /* nsIINIParser createINIParser (in nsIFile aINIFile); */
    #[inline]
    pub unsafe fn createINIParser(&self, aINIFile: Option<&nsIFile>) -> Result<Option<RefPtr<nsIINIParser>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createINIParser)(self as *const _, aINIFile.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


