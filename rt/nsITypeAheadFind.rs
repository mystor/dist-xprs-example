//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITypeAheadFind.idl
//


pub mod nsITypeAheadFind_consts {
    pub const FIND_FOUND: i64 = 0;
    pub const FIND_NOTFOUND: i64 = 1;
    pub const FIND_WRAPPED: i64 = 2;
    pub const FIND_PENDING: i64 = 3;
}


#[repr(C)]
pub struct nsITypeAheadFind {
    vtable: *const nsITypeAheadFindVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITypeAheadFind {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xae501e28, 0xc57f, 0x4692,
            [0xac, 0x74, 0x41, 0x0e, 0x1b, 0xed, 0x98, 0xb7])
    }
}

unsafe impl RefCounted for nsITypeAheadFind {
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
pub trait nsITypeAheadFindCoerce {
    fn coerce_from(v: &nsITypeAheadFind) -> &Self;
}

impl nsITypeAheadFindCoerce for nsITypeAheadFind {
    #[inline]
    fn coerce_from(v: &nsITypeAheadFind) -> &Self {
        v
    }
}

impl nsITypeAheadFind {
    #[inline]
    pub fn coerce<T: nsITypeAheadFindCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITypeAheadFind {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITypeAheadFindCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITypeAheadFind) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITypeAheadFindVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIDocShell aDocShell); */
    pub init: unsafe extern "C" fn (this: *const nsITypeAheadFind, aDocShell: *const nsIDocShell) -> nsresult,

    /* unsigned short find (in AString aSearchString, in boolean aLinksOnly); */
    pub find: unsafe extern "C" fn (this: *const nsITypeAheadFind, aSearchString: *const nsAString, aLinksOnly: bool, _retval: *mut libc::uint16_t) -> nsresult,

    /* unsigned short findAgain (in boolean findBackwards, in boolean aLinksOnly); */
    pub findAgain: unsafe extern "C" fn (this: *const nsITypeAheadFind, findBackwards: bool, aLinksOnly: bool, _retval: *mut libc::uint16_t) -> nsresult,

    /* nsIDOMRange getFoundRange (); */
    pub getFoundRange: unsafe extern "C" fn (this: *const nsITypeAheadFind, _retval: *mut *const nsIDOMRange) -> nsresult,

    /* void setDocShell (in nsIDocShell aDocShell); */
    pub setDocShell: unsafe extern "C" fn (this: *const nsITypeAheadFind, aDocShell: *const nsIDocShell) -> nsresult,

    /* void setSelectionModeAndRepaint (in short toggle); */
    pub setSelectionModeAndRepaint: unsafe extern "C" fn (this: *const nsITypeAheadFind, toggle: libc::int16_t) -> nsresult,

    /* void collapseSelection (); */
    pub collapseSelection: unsafe extern "C" fn (this: *const nsITypeAheadFind) -> nsresult,

    /* boolean isRangeVisible (in nsIDOMRange aRange, in boolean aMustBeInViewPort); */
    pub isRangeVisible: unsafe extern "C" fn (this: *const nsITypeAheadFind, aRange: *const nsIDOMRange, aMustBeInViewPort: bool, _retval: *mut bool) -> nsresult,

    /* readonly attribute AString searchString; */
    pub get_searchString: unsafe extern "C" fn (this: *const nsITypeAheadFind, aSearchString: *mut nsAString) -> nsresult,

    /* attribute boolean caseSensitive; */
    pub get_caseSensitive: unsafe extern "C" fn (this: *const nsITypeAheadFind, aCaseSensitive: *mut bool) -> nsresult,
    pub set_caseSensitive: unsafe extern "C" fn (this: *const nsITypeAheadFind, aCaseSensitive: bool) -> nsresult,

    /* attribute boolean entireWord; */
    pub get_entireWord: unsafe extern "C" fn (this: *const nsITypeAheadFind, aEntireWord: *mut bool) -> nsresult,
    pub set_entireWord: unsafe extern "C" fn (this: *const nsITypeAheadFind, aEntireWord: bool) -> nsresult,

    /* readonly attribute nsIDOMElement foundLink; */
    pub get_foundLink: unsafe extern "C" fn (this: *const nsITypeAheadFind, aFoundLink: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute nsIDOMElement foundEditable; */
    pub get_foundEditable: unsafe extern "C" fn (this: *const nsITypeAheadFind, aFoundEditable: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute mozIDOMWindow currentWindow; */
    pub get_currentWindow: unsafe extern "C" fn (this: *const nsITypeAheadFind, aCurrentWindow: *mut *const mozIDOMWindow) -> nsresult,

}


impl nsITypeAheadFind {
    /* void init (in nsIDocShell aDocShell); */
    #[inline]
    pub unsafe fn init(&self, aDocShell: Option<&nsIDocShell>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aDocShell.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned short find (in AString aSearchString, in boolean aLinksOnly); */
    #[inline]
    pub unsafe fn find(&self, aSearchString: &[u16], aLinksOnly: bool) -> Result<libc::uint16_t, nsresult> {
        let aSearchString = nsString::from(aSearchString);
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).find)(self as *const _, &*aSearchString, aLinksOnly, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned short findAgain (in boolean findBackwards, in boolean aLinksOnly); */
    #[inline]
    pub unsafe fn findAgain(&self, findBackwards: bool, aLinksOnly: bool) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).findAgain)(self as *const _, findBackwards, aLinksOnly, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMRange getFoundRange (); */
    #[inline]
    pub unsafe fn getFoundRange(&self, ) -> Result<Option<RefPtr<nsIDOMRange>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFoundRange)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setDocShell (in nsIDocShell aDocShell); */
    #[inline]
    pub unsafe fn setDocShell(&self, aDocShell: Option<&nsIDocShell>) -> Result<(), nsresult> {

        match ((*self.vtable).setDocShell)(self as *const _, aDocShell.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setSelectionModeAndRepaint (in short toggle); */
    #[inline]
    pub unsafe fn setSelectionModeAndRepaint(&self, toggle: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setSelectionModeAndRepaint)(self as *const _, toggle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void collapseSelection (); */
    #[inline]
    pub unsafe fn collapseSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).collapseSelection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isRangeVisible (in nsIDOMRange aRange, in boolean aMustBeInViewPort); */
    #[inline]
    pub unsafe fn isRangeVisible(&self, aRange: Option<&nsIDOMRange>, aMustBeInViewPort: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isRangeVisible)(self as *const _, aRange.map_or(::std::ptr::null(), |x| x as *const _), aMustBeInViewPort, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString searchString; */
    #[inline]
    pub unsafe fn get_searchString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_searchString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* readonly attribute nsIDOMElement foundLink; */
    #[inline]
    pub unsafe fn get_foundLink(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_foundLink)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMElement foundEditable; */
    #[inline]
    pub unsafe fn get_foundEditable(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_foundEditable)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute mozIDOMWindow currentWindow; */
    #[inline]
    pub unsafe fn get_currentWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


