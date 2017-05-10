//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeed.idl
//


pub mod nsIFeed_consts {
    pub const TYPE_FEED: i64 = 0;
    pub const TYPE_AUDIO: i64 = 1;
    pub const TYPE_IMAGE: i64 = 2;
    pub const TYPE_VIDEO: i64 = 4;
}


#[repr(C)]
pub struct nsIFeed {
    vtable: *const nsIFeedVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeed {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3b8aae33, 0x80e2, 0x4efa,
            [0x99, 0xc8, 0xa6, 0xc5, 0xb9, 0x9f, 0x76, 0xea])
    }
}

unsafe impl RefCounted for nsIFeed {
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
pub trait nsIFeedCoerce {
    fn coerce_from(v: &nsIFeed) -> &Self;
}

impl nsIFeedCoerce for nsIFeed {
    #[inline]
    fn coerce_from(v: &nsIFeed) -> &Self {
        v
    }
}

impl nsIFeed {
    #[inline]
    pub fn coerce<T: nsIFeedCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeed {
    type Target = nsIFeedContainer;
    #[inline]
    fn deref(&self) -> &nsIFeedContainer {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIFeedContainerCoerce> nsIFeedCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeed) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedVTable {
    pub __base: nsIFeedContainerVTable,

    /* attribute nsIFeedTextConstruct subtitle; */
    pub get_subtitle: unsafe extern "C" fn (this: *const nsIFeed, aSubtitle: *mut *const nsIFeedTextConstruct) -> nsresult,
    pub set_subtitle: unsafe extern "C" fn (this: *const nsIFeed, aSubtitle: *const nsIFeedTextConstruct) -> nsresult,

    /* readonly attribute unsigned long type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIFeed, aType: *mut libc::uint32_t) -> nsresult,

    /* attribute long enclosureCount; */
    pub get_enclosureCount: unsafe extern "C" fn (this: *const nsIFeed, aEnclosureCount: *mut libc::int32_t) -> nsresult,
    pub set_enclosureCount: unsafe extern "C" fn (this: *const nsIFeed, aEnclosureCount: libc::int32_t) -> nsresult,

    /* attribute nsIArray items; */
    pub get_items: unsafe extern "C" fn (this: *const nsIFeed, aItems: *mut *const nsIArray) -> nsresult,
    pub set_items: unsafe extern "C" fn (this: *const nsIFeed, aItems: *const nsIArray) -> nsresult,

    /* attribute nsIWritablePropertyBag2 cloud; */
    pub get_cloud: unsafe extern "C" fn (this: *const nsIFeed, aCloud: *mut *const nsIWritablePropertyBag2) -> nsresult,
    pub set_cloud: unsafe extern "C" fn (this: *const nsIFeed, aCloud: *const nsIWritablePropertyBag2) -> nsresult,

    /* attribute nsIFeedGenerator generator; */
    pub get_generator: unsafe extern "C" fn (this: *const nsIFeed, aGenerator: *mut *const nsIFeedGenerator) -> nsresult,
    pub set_generator: unsafe extern "C" fn (this: *const nsIFeed, aGenerator: *const nsIFeedGenerator) -> nsresult,

    /* attribute nsIWritablePropertyBag2 image; */
    pub get_image: unsafe extern "C" fn (this: *const nsIFeed, aImage: *mut *const nsIWritablePropertyBag2) -> nsresult,
    pub set_image: unsafe extern "C" fn (this: *const nsIFeed, aImage: *const nsIWritablePropertyBag2) -> nsresult,

    /* attribute nsIWritablePropertyBag2 textInput; */
    pub get_textInput: unsafe extern "C" fn (this: *const nsIFeed, aTextInput: *mut *const nsIWritablePropertyBag2) -> nsresult,
    pub set_textInput: unsafe extern "C" fn (this: *const nsIFeed, aTextInput: *const nsIWritablePropertyBag2) -> nsresult,

    /* attribute nsIArray skipDays; */
    pub get_skipDays: unsafe extern "C" fn (this: *const nsIFeed, aSkipDays: *mut *const nsIArray) -> nsresult,
    pub set_skipDays: unsafe extern "C" fn (this: *const nsIFeed, aSkipDays: *const nsIArray) -> nsresult,

    /* attribute nsIArray skipHours; */
    pub get_skipHours: unsafe extern "C" fn (this: *const nsIFeed, aSkipHours: *mut *const nsIArray) -> nsresult,
    pub set_skipHours: unsafe extern "C" fn (this: *const nsIFeed, aSkipHours: *const nsIArray) -> nsresult,

}


impl nsIFeed {
    /* attribute nsIFeedTextConstruct subtitle; */
    #[inline]
    pub unsafe fn get_subtitle(&self, ) -> Result<Option<RefPtr<nsIFeedTextConstruct>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_subtitle)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_subtitle(&self, aSubtitle: Option<&nsIFeedTextConstruct>) -> Result<(), nsresult> {

        match ((*self.vtable).set_subtitle)(self as *const _, aSubtitle.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute long enclosureCount; */
    #[inline]
    pub unsafe fn get_enclosureCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_enclosureCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_enclosureCount(&self, aEnclosureCount: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_enclosureCount)(self as *const _, aEnclosureCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIArray items; */
    #[inline]
    pub unsafe fn get_items(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_items)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_items(&self, aItems: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_items)(self as *const _, aItems.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIWritablePropertyBag2 cloud; */
    #[inline]
    pub unsafe fn get_cloud(&self, ) -> Result<Option<RefPtr<nsIWritablePropertyBag2>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_cloud)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_cloud(&self, aCloud: Option<&nsIWritablePropertyBag2>) -> Result<(), nsresult> {

        match ((*self.vtable).set_cloud)(self as *const _, aCloud.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIFeedGenerator generator; */
    #[inline]
    pub unsafe fn get_generator(&self, ) -> Result<Option<RefPtr<nsIFeedGenerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_generator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_generator(&self, aGenerator: Option<&nsIFeedGenerator>) -> Result<(), nsresult> {

        match ((*self.vtable).set_generator)(self as *const _, aGenerator.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIWritablePropertyBag2 image; */
    #[inline]
    pub unsafe fn get_image(&self, ) -> Result<Option<RefPtr<nsIWritablePropertyBag2>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_image)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_image(&self, aImage: Option<&nsIWritablePropertyBag2>) -> Result<(), nsresult> {

        match ((*self.vtable).set_image)(self as *const _, aImage.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIWritablePropertyBag2 textInput; */
    #[inline]
    pub unsafe fn get_textInput(&self, ) -> Result<Option<RefPtr<nsIWritablePropertyBag2>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_textInput)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_textInput(&self, aTextInput: Option<&nsIWritablePropertyBag2>) -> Result<(), nsresult> {

        match ((*self.vtable).set_textInput)(self as *const _, aTextInput.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIArray skipDays; */
    #[inline]
    pub unsafe fn get_skipDays(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_skipDays)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_skipDays(&self, aSkipDays: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_skipDays)(self as *const _, aSkipDays.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIArray skipHours; */
    #[inline]
    pub unsafe fn get_skipHours(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_skipHours)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_skipHours(&self, aSkipHours: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_skipHours)(self as *const _, aSkipHours.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


