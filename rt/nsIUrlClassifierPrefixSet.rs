//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlClassifierPrefixSet.idl
//


#[repr(C)]
pub struct nsIUrlClassifierPrefixSet {
    vtable: *const nsIUrlClassifierPrefixSetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierPrefixSet {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3d8579f0, 0x75fa, 0x4e00,
            [0xba, 0x41, 0x38, 0x66, 0x1d, 0x5b, 0x5d, 0x17])
    }
}

unsafe impl RefCounted for nsIUrlClassifierPrefixSet {
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
pub trait nsIUrlClassifierPrefixSetCoerce {
    fn coerce_from(v: &nsIUrlClassifierPrefixSet) -> &Self;
}

impl nsIUrlClassifierPrefixSetCoerce for nsIUrlClassifierPrefixSet {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierPrefixSet) -> &Self {
        v
    }
}

impl nsIUrlClassifierPrefixSet {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierPrefixSetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierPrefixSet {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierPrefixSetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierPrefixSet) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierPrefixSetVTable {
    pub __base: nsISupportsVTable,

    /* void init (in ACString aName); */
    pub init: unsafe extern "C" fn (this: *const nsIUrlClassifierPrefixSet, aName: *const nsACString) -> nsresult,

    /* void setPrefixes ([array, size_is (aLength), const] in unsigned long aPrefixes, in unsigned long aLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub setPrefixes: *const ::libc::c_void,

    /* void getPrefixes (out unsigned long aCount, [array, size_is (aCount), retval] out unsigned long aPrefixes); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPrefixes: *const ::libc::c_void,

    /* boolean contains (in unsigned long aPrefix); */
    pub contains: unsafe extern "C" fn (this: *const nsIUrlClassifierPrefixSet, aPrefix: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* boolean isEmpty (); */
    pub isEmpty: unsafe extern "C" fn (this: *const nsIUrlClassifierPrefixSet, _retval: *mut bool) -> nsresult,

    /* void loadFromFile (in nsIFile aFile); */
    pub loadFromFile: unsafe extern "C" fn (this: *const nsIUrlClassifierPrefixSet, aFile: *const nsIFile) -> nsresult,

    /* void storeToFile (in nsIFile aFile); */
    pub storeToFile: unsafe extern "C" fn (this: *const nsIUrlClassifierPrefixSet, aFile: *const nsIFile) -> nsresult,

}


impl nsIUrlClassifierPrefixSet {
    /* void init (in ACString aName); */
    #[inline]
    pub unsafe fn init(&self, aName: &[u8]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).init)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPrefixes ([array, size_is (aLength), const] in unsigned long aPrefixes, in unsigned long aLength); */


    /* void getPrefixes (out unsigned long aCount, [array, size_is (aCount), retval] out unsigned long aPrefixes); */


    /* boolean contains (in unsigned long aPrefix); */
    #[inline]
    pub unsafe fn contains(&self, aPrefix: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).contains)(self as *const _, aPrefix, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isEmpty (); */
    #[inline]
    pub unsafe fn isEmpty(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isEmpty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void loadFromFile (in nsIFile aFile); */
    #[inline]
    pub unsafe fn loadFromFile(&self, aFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).loadFromFile)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void storeToFile (in nsIFile aFile); */
    #[inline]
    pub unsafe fn storeToFile(&self, aFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).storeToFile)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


