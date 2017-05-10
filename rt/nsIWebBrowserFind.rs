//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserFind.idl
//


#[repr(C)]
pub struct nsIWebBrowserFind {
    vtable: *const nsIWebBrowserFindVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserFind {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe4920136, 0xb3e0, 0x49e0,
            [0xb1, 0xcd, 0x6c, 0x78, 0x3d, 0x25, 0x91, 0xa8])
    }
}

unsafe impl RefCounted for nsIWebBrowserFind {
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
pub trait nsIWebBrowserFindCoerce {
    fn coerce_from(v: &nsIWebBrowserFind) -> &Self;
}

impl nsIWebBrowserFindCoerce for nsIWebBrowserFind {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserFind) -> &Self {
        v
    }
}

impl nsIWebBrowserFind {
    #[inline]
    pub fn coerce<T: nsIWebBrowserFindCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserFind {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserFindCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserFind) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserFindVTable {
    pub __base: nsISupportsVTable,

    /* boolean findNext (); */
    pub findNext: unsafe extern "C" fn (this: *const nsIWebBrowserFind, _retval: *mut bool) -> nsresult,

    /* attribute wstring searchString; */
    pub get_searchString: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aSearchString: *mut *const libc::int16_t) -> nsresult,
    pub set_searchString: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aSearchString: *const libc::int16_t) -> nsresult,

    /* attribute boolean findBackwards; */
    pub get_findBackwards: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aFindBackwards: *mut bool) -> nsresult,
    pub set_findBackwards: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aFindBackwards: bool) -> nsresult,

    /* attribute boolean wrapFind; */
    pub get_wrapFind: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aWrapFind: *mut bool) -> nsresult,
    pub set_wrapFind: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aWrapFind: bool) -> nsresult,

    /* attribute boolean entireWord; */
    pub get_entireWord: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aEntireWord: *mut bool) -> nsresult,
    pub set_entireWord: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aEntireWord: bool) -> nsresult,

    /* attribute boolean matchCase; */
    pub get_matchCase: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aMatchCase: *mut bool) -> nsresult,
    pub set_matchCase: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aMatchCase: bool) -> nsresult,

    /* attribute boolean searchFrames; */
    pub get_searchFrames: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aSearchFrames: *mut bool) -> nsresult,
    pub set_searchFrames: unsafe extern "C" fn (this: *const nsIWebBrowserFind, aSearchFrames: bool) -> nsresult,

}


impl nsIWebBrowserFind {
    /* boolean findNext (); */
    #[inline]
    pub unsafe fn findNext(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).findNext)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute wstring searchString; */
    #[inline]
    pub unsafe fn get_searchString(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_searchString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_searchString(&self, aSearchString: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_searchString)(self as *const _, aSearchString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* attribute boolean wrapFind; */
    #[inline]
    pub unsafe fn get_wrapFind(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_wrapFind)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_wrapFind(&self, aWrapFind: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_wrapFind)(self as *const _, aWrapFind) {
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

    /* attribute boolean matchCase; */
    #[inline]
    pub unsafe fn get_matchCase(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_matchCase)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_matchCase(&self, aMatchCase: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_matchCase)(self as *const _, aMatchCase) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean searchFrames; */
    #[inline]
    pub unsafe fn get_searchFrames(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_searchFrames)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_searchFrames(&self, aSearchFrames: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_searchFrames)(self as *const _, aSearchFrames) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIWebBrowserFindInFrames {
    vtable: *const nsIWebBrowserFindInFramesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserFindInFrames {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe0f5d182, 0x34bc, 0x11d5,
            [0xbe, 0x5b, 0xb7, 0x60, 0x67, 0x6c, 0x6e, 0xbc])
    }
}

unsafe impl RefCounted for nsIWebBrowserFindInFrames {
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
pub trait nsIWebBrowserFindInFramesCoerce {
    fn coerce_from(v: &nsIWebBrowserFindInFrames) -> &Self;
}

impl nsIWebBrowserFindInFramesCoerce for nsIWebBrowserFindInFrames {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserFindInFrames) -> &Self {
        v
    }
}

impl nsIWebBrowserFindInFrames {
    #[inline]
    pub fn coerce<T: nsIWebBrowserFindInFramesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserFindInFrames {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserFindInFramesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserFindInFrames) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserFindInFramesVTable {
    pub __base: nsISupportsVTable,

    /* attribute mozIDOMWindowProxy currentSearchFrame; */
    pub get_currentSearchFrame: unsafe extern "C" fn (this: *const nsIWebBrowserFindInFrames, aCurrentSearchFrame: *mut *const mozIDOMWindowProxy) -> nsresult,
    pub set_currentSearchFrame: unsafe extern "C" fn (this: *const nsIWebBrowserFindInFrames, aCurrentSearchFrame: *const mozIDOMWindowProxy) -> nsresult,

    /* attribute mozIDOMWindowProxy rootSearchFrame; */
    pub get_rootSearchFrame: unsafe extern "C" fn (this: *const nsIWebBrowserFindInFrames, aRootSearchFrame: *mut *const mozIDOMWindowProxy) -> nsresult,
    pub set_rootSearchFrame: unsafe extern "C" fn (this: *const nsIWebBrowserFindInFrames, aRootSearchFrame: *const mozIDOMWindowProxy) -> nsresult,

    /* attribute boolean searchSubframes; */
    pub get_searchSubframes: unsafe extern "C" fn (this: *const nsIWebBrowserFindInFrames, aSearchSubframes: *mut bool) -> nsresult,
    pub set_searchSubframes: unsafe extern "C" fn (this: *const nsIWebBrowserFindInFrames, aSearchSubframes: bool) -> nsresult,

    /* attribute boolean searchParentFrames; */
    pub get_searchParentFrames: unsafe extern "C" fn (this: *const nsIWebBrowserFindInFrames, aSearchParentFrames: *mut bool) -> nsresult,
    pub set_searchParentFrames: unsafe extern "C" fn (this: *const nsIWebBrowserFindInFrames, aSearchParentFrames: bool) -> nsresult,

}


impl nsIWebBrowserFindInFrames {
    /* attribute mozIDOMWindowProxy currentSearchFrame; */
    #[inline]
    pub unsafe fn get_currentSearchFrame(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentSearchFrame)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_currentSearchFrame(&self, aCurrentSearchFrame: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).set_currentSearchFrame)(self as *const _, aCurrentSearchFrame.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute mozIDOMWindowProxy rootSearchFrame; */
    #[inline]
    pub unsafe fn get_rootSearchFrame(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rootSearchFrame)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_rootSearchFrame(&self, aRootSearchFrame: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).set_rootSearchFrame)(self as *const _, aRootSearchFrame.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean searchSubframes; */
    #[inline]
    pub unsafe fn get_searchSubframes(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_searchSubframes)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_searchSubframes(&self, aSearchSubframes: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_searchSubframes)(self as *const _, aSearchSubframes) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean searchParentFrames; */
    #[inline]
    pub unsafe fn get_searchParentFrames(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_searchParentFrames)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_searchParentFrames(&self, aSearchParentFrames: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_searchParentFrames)(self as *const _, aSearchParentFrames) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


