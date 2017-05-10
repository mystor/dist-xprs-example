//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITaggingService.idl
//


pub mod nsITaggingService_consts {
    pub const MAX_TAG_LENGTH: i64 = 100;
}


#[repr(C)]
pub struct nsITaggingService {
    vtable: *const nsITaggingServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITaggingService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9759bd0e, 0x78e2, 0x4421,
            [0x9e, 0xd1, 0xc6, 0x76, 0xe1, 0xaf, 0x35, 0x13])
    }
}

unsafe impl RefCounted for nsITaggingService {
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
pub trait nsITaggingServiceCoerce {
    fn coerce_from(v: &nsITaggingService) -> &Self;
}

impl nsITaggingServiceCoerce for nsITaggingService {
    #[inline]
    fn coerce_from(v: &nsITaggingService) -> &Self {
        v
    }
}

impl nsITaggingService {
    #[inline]
    pub fn coerce<T: nsITaggingServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITaggingService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITaggingServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITaggingService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITaggingServiceVTable {
    pub __base: nsISupportsVTable,

    /* void tagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource); */
    pub tagURI: unsafe extern "C" fn (this: *const nsITaggingService, aURI: *const nsIURI, aTags: *const nsIVariant, aSource: libc::uint16_t) -> nsresult,

    /* void untagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource); */
    pub untagURI: unsafe extern "C" fn (this: *const nsITaggingService, aURI: *const nsIURI, aTags: *const nsIVariant, aSource: libc::uint16_t) -> nsresult,

    /* nsIVariant getURIsForTag (in AString aTag); */
    pub getURIsForTag: unsafe extern "C" fn (this: *const nsITaggingService, aTag: *const nsAString, _retval: *mut *const nsIVariant) -> nsresult,

    /* void getTagsForURI (in nsIURI aURI, [optional] out unsigned long length, [array, size_is (length), retval] out wstring aTags); */
    /// Unable to call function as its signature contains a non-rust type
    pub getTagsForURI: *const ::libc::c_void,

    /* readonly attribute nsIVariant allTags; */
    pub get_allTags: unsafe extern "C" fn (this: *const nsITaggingService, aAllTags: *mut *const nsIVariant) -> nsresult,

    /* readonly attribute boolean hasTags; */
    pub get_hasTags: unsafe extern "C" fn (this: *const nsITaggingService, aHasTags: *mut bool) -> nsresult,

}


impl nsITaggingService {
    /* void tagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn tagURI(&self, aURI: Option<&nsIURI>, aTags: Option<&nsIVariant>, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).tagURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aTags.map_or(::std::ptr::null(), |x| x as *const _), aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void untagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn untagURI(&self, aURI: Option<&nsIURI>, aTags: Option<&nsIVariant>, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).untagURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aTags.map_or(::std::ptr::null(), |x| x as *const _), aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIVariant getURIsForTag (in AString aTag); */
    #[inline]
    pub unsafe fn getURIsForTag(&self, aTag: &[u16]) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let aTag = nsString::from(aTag);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getURIsForTag)(self as *const _, &*aTag, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getTagsForURI (in nsIURI aURI, [optional] out unsigned long length, [array, size_is (length), retval] out wstring aTags); */


    /* readonly attribute nsIVariant allTags; */
    #[inline]
    pub unsafe fn get_allTags(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_allTags)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean hasTags; */
    #[inline]
    pub unsafe fn get_hasTags(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasTags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


