//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedContainer.idl
//


#[repr(C)]
pub struct nsIFeedContainer {
    vtable: *const nsIFeedContainerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedContainer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x577a1b4c, 0xb3d4, 0x4c76,
            [0x9c, 0xf8, 0x75, 0x3e, 0x66, 0x06, 0x11, 0x4f])
    }
}

unsafe impl RefCounted for nsIFeedContainer {
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
pub trait nsIFeedContainerCoerce {
    fn coerce_from(v: &nsIFeedContainer) -> &Self;
}

impl nsIFeedContainerCoerce for nsIFeedContainer {
    #[inline]
    fn coerce_from(v: &nsIFeedContainer) -> &Self {
        v
    }
}

impl nsIFeedContainer {
    #[inline]
    pub fn coerce<T: nsIFeedContainerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedContainer {
    type Target = nsIFeedElementBase;
    #[inline]
    fn deref(&self) -> &nsIFeedElementBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIFeedElementBaseCoerce> nsIFeedContainerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedContainer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedContainerVTable {
    pub __base: nsIFeedElementBaseVTable,

    /* attribute AString id; */
    pub get_id: unsafe extern "C" fn (this: *const nsIFeedContainer, aId: *mut nsAString) -> nsresult,
    pub set_id: unsafe extern "C" fn (this: *const nsIFeedContainer, aId: *const nsAString) -> nsresult,

    /* attribute nsIWritablePropertyBag2 fields; */
    pub get_fields: unsafe extern "C" fn (this: *const nsIFeedContainer, aFields: *mut *const nsIWritablePropertyBag2) -> nsresult,
    pub set_fields: unsafe extern "C" fn (this: *const nsIFeedContainer, aFields: *const nsIWritablePropertyBag2) -> nsresult,

    /* attribute nsIFeedTextConstruct title; */
    pub get_title: unsafe extern "C" fn (this: *const nsIFeedContainer, aTitle: *mut *const nsIFeedTextConstruct) -> nsresult,
    pub set_title: unsafe extern "C" fn (this: *const nsIFeedContainer, aTitle: *const nsIFeedTextConstruct) -> nsresult,

    /* attribute nsIURI link; */
    pub get_link: unsafe extern "C" fn (this: *const nsIFeedContainer, aLink: *mut *const nsIURI) -> nsresult,
    pub set_link: unsafe extern "C" fn (this: *const nsIFeedContainer, aLink: *const nsIURI) -> nsresult,

    /* attribute nsIArray links; */
    pub get_links: unsafe extern "C" fn (this: *const nsIFeedContainer, aLinks: *mut *const nsIArray) -> nsresult,
    pub set_links: unsafe extern "C" fn (this: *const nsIFeedContainer, aLinks: *const nsIArray) -> nsresult,

    /* attribute nsIArray categories; */
    pub get_categories: unsafe extern "C" fn (this: *const nsIFeedContainer, aCategories: *mut *const nsIArray) -> nsresult,
    pub set_categories: unsafe extern "C" fn (this: *const nsIFeedContainer, aCategories: *const nsIArray) -> nsresult,

    /* attribute nsIFeedTextConstruct rights; */
    pub get_rights: unsafe extern "C" fn (this: *const nsIFeedContainer, aRights: *mut *const nsIFeedTextConstruct) -> nsresult,
    pub set_rights: unsafe extern "C" fn (this: *const nsIFeedContainer, aRights: *const nsIFeedTextConstruct) -> nsresult,

    /* attribute nsIArray authors; */
    pub get_authors: unsafe extern "C" fn (this: *const nsIFeedContainer, aAuthors: *mut *const nsIArray) -> nsresult,
    pub set_authors: unsafe extern "C" fn (this: *const nsIFeedContainer, aAuthors: *const nsIArray) -> nsresult,

    /* attribute nsIArray contributors; */
    pub get_contributors: unsafe extern "C" fn (this: *const nsIFeedContainer, aContributors: *mut *const nsIArray) -> nsresult,
    pub set_contributors: unsafe extern "C" fn (this: *const nsIFeedContainer, aContributors: *const nsIArray) -> nsresult,

    /* attribute AString updated; */
    pub get_updated: unsafe extern "C" fn (this: *const nsIFeedContainer, aUpdated: *mut nsAString) -> nsresult,
    pub set_updated: unsafe extern "C" fn (this: *const nsIFeedContainer, aUpdated: *const nsAString) -> nsresult,

    /* void normalize (); */
    pub normalize: unsafe extern "C" fn (this: *const nsIFeedContainer) -> nsresult,

}


impl nsIFeedContainer {
    /* attribute AString id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_id)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_id(&self, aId: &[u16]) -> Result<(), nsresult> {
        let aId = nsString::from(aId);
        match ((*self.vtable).set_id)(self as *const _, &*aId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIWritablePropertyBag2 fields; */
    #[inline]
    pub unsafe fn get_fields(&self, ) -> Result<Option<RefPtr<nsIWritablePropertyBag2>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_fields)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_fields(&self, aFields: Option<&nsIWritablePropertyBag2>) -> Result<(), nsresult> {

        match ((*self.vtable).set_fields)(self as *const _, aFields.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIFeedTextConstruct title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<Option<RefPtr<nsIFeedTextConstruct>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_title)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_title(&self, aTitle: Option<&nsIFeedTextConstruct>) -> Result<(), nsresult> {

        match ((*self.vtable).set_title)(self as *const _, aTitle.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURI link; */
    #[inline]
    pub unsafe fn get_link(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_link)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_link(&self, aLink: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_link)(self as *const _, aLink.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIArray links; */
    #[inline]
    pub unsafe fn get_links(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_links)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_links(&self, aLinks: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_links)(self as *const _, aLinks.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIArray categories; */
    #[inline]
    pub unsafe fn get_categories(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_categories)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_categories(&self, aCategories: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_categories)(self as *const _, aCategories.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIFeedTextConstruct rights; */
    #[inline]
    pub unsafe fn get_rights(&self, ) -> Result<Option<RefPtr<nsIFeedTextConstruct>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rights)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_rights(&self, aRights: Option<&nsIFeedTextConstruct>) -> Result<(), nsresult> {

        match ((*self.vtable).set_rights)(self as *const _, aRights.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIArray authors; */
    #[inline]
    pub unsafe fn get_authors(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_authors)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_authors(&self, aAuthors: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_authors)(self as *const _, aAuthors.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIArray contributors; */
    #[inline]
    pub unsafe fn get_contributors(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contributors)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_contributors(&self, aContributors: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_contributors)(self as *const _, aContributors.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString updated; */
    #[inline]
    pub unsafe fn get_updated(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_updated)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_updated(&self, aUpdated: &[u16]) -> Result<(), nsresult> {
        let aUpdated = nsString::from(aUpdated);
        match ((*self.vtable).set_updated)(self as *const _, &*aUpdated) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void normalize (); */
    #[inline]
    pub unsafe fn normalize(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).normalize)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


