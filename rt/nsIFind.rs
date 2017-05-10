//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFind.idl
//


#[repr(C)]
pub struct nsIFind {
    vtable: *const nsIFindVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFind {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x40aba110, 0x2a56, 0x4678,
            [0xbe, 0x90, 0xe2, 0xc1, 0x7a, 0x9a, 0xe7, 0xd7])
    }
}

unsafe impl RefCounted for nsIFind {
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
pub trait nsIFindCoerce {
    fn coerce_from(v: &nsIFind) -> &Self;
}

impl nsIFindCoerce for nsIFind {
    #[inline]
    fn coerce_from(v: &nsIFind) -> &Self {
        v
    }
}

impl nsIFind {
    #[inline]
    pub fn coerce<T: nsIFindCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFind {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFindCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFind) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFindVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean findBackwards; */
    pub get_findBackwards: unsafe extern "C" fn (this: *const nsIFind, aFindBackwards: *mut bool) -> nsresult,
    pub set_findBackwards: unsafe extern "C" fn (this: *const nsIFind, aFindBackwards: bool) -> nsresult,

    /* attribute boolean caseSensitive; */
    pub get_caseSensitive: unsafe extern "C" fn (this: *const nsIFind, aCaseSensitive: *mut bool) -> nsresult,
    pub set_caseSensitive: unsafe extern "C" fn (this: *const nsIFind, aCaseSensitive: bool) -> nsresult,

    /* attribute boolean entireWord; */
    pub get_entireWord: unsafe extern "C" fn (this: *const nsIFind, aEntireWord: *mut bool) -> nsresult,
    pub set_entireWord: unsafe extern "C" fn (this: *const nsIFind, aEntireWord: bool) -> nsresult,

    /* nsIDOMRange Find (in wstring aPatText, in nsIDOMRange aSearchRange, in nsIDOMRange aStartPoint, in nsIDOMRange aEndPoint); */
    pub Find: unsafe extern "C" fn (this: *const nsIFind, aPatText: *const libc::int16_t, aSearchRange: *const nsIDOMRange, aStartPoint: *const nsIDOMRange, aEndPoint: *const nsIDOMRange, _retval: *mut *const nsIDOMRange) -> nsresult,

}


impl nsIFind {
    /* attribute boolean findBackwards; */
    #[inline]
    pub unsafe fn get_findBackwards(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_findBackwards)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_findBackwards(&self, aFindBackwards: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_findBackwards)(self as *const _, aFindBackwards) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean caseSensitive; */
    #[inline]
    pub unsafe fn get_caseSensitive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_caseSensitive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_caseSensitive(&self, aCaseSensitive: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_caseSensitive)(self as *const _, aCaseSensitive) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean entireWord; */
    #[inline]
    pub unsafe fn get_entireWord(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_entireWord)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_entireWord(&self, aEntireWord: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_entireWord)(self as *const _, aEntireWord) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMRange Find (in wstring aPatText, in nsIDOMRange aSearchRange, in nsIDOMRange aStartPoint, in nsIDOMRange aEndPoint); */
    #[inline]
    pub unsafe fn Find(&self, aPatText: *const libc::int16_t, aSearchRange: Option<&nsIDOMRange>, aStartPoint: Option<&nsIDOMRange>, aEndPoint: Option<&nsIDOMRange>) -> Result<Option<RefPtr<nsIDOMRange>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).Find)(self as *const _, aPatText, aSearchRange.map_or(::std::ptr::null(), |x| x as *const _), aStartPoint.map_or(::std::ptr::null(), |x| x as *const _), aEndPoint.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


