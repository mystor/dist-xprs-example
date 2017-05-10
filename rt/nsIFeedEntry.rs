//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedEntry.idl
//


#[repr(C)]
pub struct nsIFeedEntry {
    vtable: *const nsIFeedEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedEntry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x31bfd5b4, 0x8ff5, 0x4bfd,
            [0xa8, 0xcb, 0xb3, 0xdf, 0xbd, 0x4f, 0x0a, 0x5b])
    }
}

unsafe impl RefCounted for nsIFeedEntry {
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
pub trait nsIFeedEntryCoerce {
    fn coerce_from(v: &nsIFeedEntry) -> &Self;
}

impl nsIFeedEntryCoerce for nsIFeedEntry {
    #[inline]
    fn coerce_from(v: &nsIFeedEntry) -> &Self {
        v
    }
}

impl nsIFeedEntry {
    #[inline]
    pub fn coerce<T: nsIFeedEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedEntry {
    type Target = nsIFeedContainer;
    #[inline]
    fn deref(&self) -> &nsIFeedContainer {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIFeedContainerCoerce> nsIFeedEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedEntry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedEntryVTable {
    pub __base: nsIFeedContainerVTable,

    /* attribute nsIFeedTextConstruct summary; */
    pub get_summary: unsafe extern "C" fn (this: *const nsIFeedEntry, aSummary: *mut *const nsIFeedTextConstruct) -> nsresult,
    pub set_summary: unsafe extern "C" fn (this: *const nsIFeedEntry, aSummary: *const nsIFeedTextConstruct) -> nsresult,

    /* attribute AString published; */
    pub get_published: unsafe extern "C" fn (this: *const nsIFeedEntry, aPublished: *mut nsAString) -> nsresult,
    pub set_published: unsafe extern "C" fn (this: *const nsIFeedEntry, aPublished: *const nsAString) -> nsresult,

    /* attribute nsIFeedTextConstruct content; */
    pub get_content: unsafe extern "C" fn (this: *const nsIFeedEntry, aContent: *mut *const nsIFeedTextConstruct) -> nsresult,
    pub set_content: unsafe extern "C" fn (this: *const nsIFeedEntry, aContent: *const nsIFeedTextConstruct) -> nsresult,

    /* attribute nsIArray enclosures; */
    pub get_enclosures: unsafe extern "C" fn (this: *const nsIFeedEntry, aEnclosures: *mut *const nsIArray) -> nsresult,
    pub set_enclosures: unsafe extern "C" fn (this: *const nsIFeedEntry, aEnclosures: *const nsIArray) -> nsresult,

    /* attribute nsIArray mediaContent; */
    pub get_mediaContent: unsafe extern "C" fn (this: *const nsIFeedEntry, aMediaContent: *mut *const nsIArray) -> nsresult,
    pub set_mediaContent: unsafe extern "C" fn (this: *const nsIFeedEntry, aMediaContent: *const nsIArray) -> nsresult,

}


impl nsIFeedEntry {
    /* attribute nsIFeedTextConstruct summary; */
    #[inline]
    pub unsafe fn get_summary(&self, ) -> Result<Option<RefPtr<nsIFeedTextConstruct>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_summary)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_summary(&self, aSummary: Option<&nsIFeedTextConstruct>) -> Result<(), nsresult> {

        match ((*self.vtable).set_summary)(self as *const _, aSummary.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString published; */
    #[inline]
    pub unsafe fn get_published(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_published)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_published(&self, aPublished: &[u16]) -> Result<(), nsresult> {
        let aPublished = nsString::from(aPublished);
        match ((*self.vtable).set_published)(self as *const _, &*aPublished) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIFeedTextConstruct content; */
    #[inline]
    pub unsafe fn get_content(&self, ) -> Result<Option<RefPtr<nsIFeedTextConstruct>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_content)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_content(&self, aContent: Option<&nsIFeedTextConstruct>) -> Result<(), nsresult> {

        match ((*self.vtable).set_content)(self as *const _, aContent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIArray enclosures; */
    #[inline]
    pub unsafe fn get_enclosures(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_enclosures)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_enclosures(&self, aEnclosures: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_enclosures)(self as *const _, aEnclosures.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIArray mediaContent; */
    #[inline]
    pub unsafe fn get_mediaContent(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_mediaContent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_mediaContent(&self, aMediaContent: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_mediaContent)(self as *const _, aMediaContent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


