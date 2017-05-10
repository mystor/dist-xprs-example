//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGlobalHistory2.idl
//


#[repr(C)]
pub struct nsIGlobalHistory2 {
    vtable: *const nsIGlobalHistory2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGlobalHistory2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcf777d42, 0x1270, 0x4b34,
            [0xbe, 0x7b, 0x29, 0x31, 0xc9, 0x3f, 0xed, 0xa5])
    }
}

unsafe impl RefCounted for nsIGlobalHistory2 {
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
pub trait nsIGlobalHistory2Coerce {
    fn coerce_from(v: &nsIGlobalHistory2) -> &Self;
}

impl nsIGlobalHistory2Coerce for nsIGlobalHistory2 {
    #[inline]
    fn coerce_from(v: &nsIGlobalHistory2) -> &Self {
        v
    }
}

impl nsIGlobalHistory2 {
    #[inline]
    pub fn coerce<T: nsIGlobalHistory2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGlobalHistory2 {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGlobalHistory2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIGlobalHistory2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGlobalHistory2VTable {
    pub __base: nsISupportsVTable,

    /* void addURI (in nsIURI aURI, in boolean aRedirect, in boolean aToplevel, in nsIURI aReferrer); */
    pub addURI: unsafe extern "C" fn (this: *const nsIGlobalHistory2, aURI: *const nsIURI, aRedirect: bool, aToplevel: bool, aReferrer: *const nsIURI) -> nsresult,

    /* boolean isVisited (in nsIURI aURI); */
    pub isVisited: unsafe extern "C" fn (this: *const nsIGlobalHistory2, aURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* void setPageTitle (in nsIURI aURI, in AString aTitle); */
    pub setPageTitle: unsafe extern "C" fn (this: *const nsIGlobalHistory2, aURI: *const nsIURI, aTitle: *const nsAString) -> nsresult,

}


impl nsIGlobalHistory2 {
    /* void addURI (in nsIURI aURI, in boolean aRedirect, in boolean aToplevel, in nsIURI aReferrer); */
    #[inline]
    pub unsafe fn addURI(&self, aURI: Option<&nsIURI>, aRedirect: bool, aToplevel: bool, aReferrer: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).addURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aRedirect, aToplevel, aReferrer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isVisited (in nsIURI aURI); */
    #[inline]
    pub unsafe fn isVisited(&self, aURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isVisited)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setPageTitle (in nsIURI aURI, in AString aTitle); */
    #[inline]
    pub unsafe fn setPageTitle(&self, aURI: Option<&nsIURI>, aTitle: &[u16]) -> Result<(), nsresult> {
        let aTitle = nsString::from(aTitle);
        match ((*self.vtable).setPageTitle)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aTitle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


