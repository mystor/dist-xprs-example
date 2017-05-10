//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINavBookmarksService.idl
//


#[repr(C)]
pub struct nsINavBookmarkObserver {
    vtable: *const nsINavBookmarkObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavBookmarkObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc06b4e7d, 0x15b1, 0x4d4f,
            [0xbd, 0xf7, 0x14, 0x7d, 0x2b, 0xe9, 0x08, 0x4a])
    }
}

unsafe impl RefCounted for nsINavBookmarkObserver {
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
pub trait nsINavBookmarkObserverCoerce {
    fn coerce_from(v: &nsINavBookmarkObserver) -> &Self;
}

impl nsINavBookmarkObserverCoerce for nsINavBookmarkObserver {
    #[inline]
    fn coerce_from(v: &nsINavBookmarkObserver) -> &Self {
        v
    }
}

impl nsINavBookmarkObserver {
    #[inline]
    pub fn coerce<T: nsINavBookmarkObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavBookmarkObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINavBookmarkObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavBookmarkObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavBookmarkObserverVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean skipTags; */
    pub get_skipTags: unsafe extern "C" fn (this: *const nsINavBookmarkObserver, aSkipTags: *mut bool) -> nsresult,

    /* readonly attribute boolean skipDescendantsOnItemRemoval; */
    pub get_skipDescendantsOnItemRemoval: unsafe extern "C" fn (this: *const nsINavBookmarkObserver, aSkipDescendantsOnItemRemoval: *mut bool) -> nsresult,

    /* void onBeginUpdateBatch (); */
    pub onBeginUpdateBatch: unsafe extern "C" fn (this: *const nsINavBookmarkObserver) -> nsresult,

    /* void onEndUpdateBatch (); */
    pub onEndUpdateBatch: unsafe extern "C" fn (this: *const nsINavBookmarkObserver) -> nsresult,

    /* void onItemAdded (in long long aItemId, in long long aParentId, in long aIndex, in unsigned short aItemType, in nsIURI aURI, in AUTF8String aTitle, in PRTime aDateAdded, in ACString aGuid, in ACString aParentGuid, in unsigned short aSource); */
    pub onItemAdded: unsafe extern "C" fn (this: *const nsINavBookmarkObserver, aItemId: libc::int64_t, aParentId: libc::int64_t, aIndex: libc::int32_t, aItemType: libc::uint16_t, aURI: *const nsIURI, aTitle: *const nsACString, aDateAdded: PRTime, aGuid: *const nsACString, aParentGuid: *const nsACString, aSource: libc::uint16_t) -> nsresult,

    /* void onItemRemoved (in long long aItemId, in long long aParentId, in long aIndex, in unsigned short aItemType, in nsIURI aURI, in ACString aGuid, in ACString aParentGuid, in unsigned short aSource); */
    pub onItemRemoved: unsafe extern "C" fn (this: *const nsINavBookmarkObserver, aItemId: libc::int64_t, aParentId: libc::int64_t, aIndex: libc::int32_t, aItemType: libc::uint16_t, aURI: *const nsIURI, aGuid: *const nsACString, aParentGuid: *const nsACString, aSource: libc::uint16_t) -> nsresult,

    /* void onItemChanged (in long long aItemId, in ACString aProperty, in boolean aIsAnnotationProperty, in AUTF8String aNewValue, in PRTime aLastModified, in unsigned short aItemType, in long long aParentId, in ACString aGuid, in ACString aParentGuid, in AUTF8String aOldValue, in unsigned short aSource); */
    pub onItemChanged: unsafe extern "C" fn (this: *const nsINavBookmarkObserver, aItemId: libc::int64_t, aProperty: *const nsACString, aIsAnnotationProperty: bool, aNewValue: *const nsACString, aLastModified: PRTime, aItemType: libc::uint16_t, aParentId: libc::int64_t, aGuid: *const nsACString, aParentGuid: *const nsACString, aOldValue: *const nsACString, aSource: libc::uint16_t) -> nsresult,

    /* void onItemVisited (in long long aItemId, in long long aVisitId, in PRTime aTime, in unsigned long aTransitionType, in nsIURI aURI, in long long aParentId, in ACString aGuid, in ACString aParentGuid); */
    pub onItemVisited: unsafe extern "C" fn (this: *const nsINavBookmarkObserver, aItemId: libc::int64_t, aVisitId: libc::int64_t, aTime: PRTime, aTransitionType: libc::uint32_t, aURI: *const nsIURI, aParentId: libc::int64_t, aGuid: *const nsACString, aParentGuid: *const nsACString) -> nsresult,

    /* void onItemMoved (in long long aItemId, in long long aOldParentId, in long aOldIndex, in long long aNewParentId, in long aNewIndex, in unsigned short aItemType, in ACString aGuid, in ACString aOldParentGuid, in ACString aNewParentGuid, in unsigned short aSource); */
    pub onItemMoved: unsafe extern "C" fn (this: *const nsINavBookmarkObserver, aItemId: libc::int64_t, aOldParentId: libc::int64_t, aOldIndex: libc::int32_t, aNewParentId: libc::int64_t, aNewIndex: libc::int32_t, aItemType: libc::uint16_t, aGuid: *const nsACString, aOldParentGuid: *const nsACString, aNewParentGuid: *const nsACString, aSource: libc::uint16_t) -> nsresult,

}


impl nsINavBookmarkObserver {
    /* readonly attribute boolean skipTags; */
    #[inline]
    pub unsafe fn get_skipTags(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_skipTags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean skipDescendantsOnItemRemoval; */
    #[inline]
    pub unsafe fn get_skipDescendantsOnItemRemoval(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_skipDescendantsOnItemRemoval)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void onBeginUpdateBatch (); */
    #[inline]
    pub unsafe fn onBeginUpdateBatch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onBeginUpdateBatch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onEndUpdateBatch (); */
    #[inline]
    pub unsafe fn onEndUpdateBatch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onEndUpdateBatch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onItemAdded (in long long aItemId, in long long aParentId, in long aIndex, in unsigned short aItemType, in nsIURI aURI, in AUTF8String aTitle, in PRTime aDateAdded, in ACString aGuid, in ACString aParentGuid, in unsigned short aSource); */
    #[inline]
    pub unsafe fn onItemAdded(&self, aItemId: libc::int64_t, aParentId: libc::int64_t, aIndex: libc::int32_t, aItemType: libc::uint16_t, aURI: Option<&nsIURI>, aTitle: &[u8], aDateAdded: PRTime, aGuid: &[u8], aParentGuid: &[u8], aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aTitle = nsCString::from(aTitle);
        let aGuid = nsCString::from(aGuid);
        let aParentGuid = nsCString::from(aParentGuid);
        match ((*self.vtable).onItemAdded)(self as *const _, aItemId, aParentId, aIndex, aItemType, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aTitle, aDateAdded, &*aGuid, &*aParentGuid, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onItemRemoved (in long long aItemId, in long long aParentId, in long aIndex, in unsigned short aItemType, in nsIURI aURI, in ACString aGuid, in ACString aParentGuid, in unsigned short aSource); */
    #[inline]
    pub unsafe fn onItemRemoved(&self, aItemId: libc::int64_t, aParentId: libc::int64_t, aIndex: libc::int32_t, aItemType: libc::uint16_t, aURI: Option<&nsIURI>, aGuid: &[u8], aParentGuid: &[u8], aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aGuid = nsCString::from(aGuid);
        let aParentGuid = nsCString::from(aParentGuid);
        match ((*self.vtable).onItemRemoved)(self as *const _, aItemId, aParentId, aIndex, aItemType, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aGuid, &*aParentGuid, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onItemChanged (in long long aItemId, in ACString aProperty, in boolean aIsAnnotationProperty, in AUTF8String aNewValue, in PRTime aLastModified, in unsigned short aItemType, in long long aParentId, in ACString aGuid, in ACString aParentGuid, in AUTF8String aOldValue, in unsigned short aSource); */
    #[inline]
    pub unsafe fn onItemChanged(&self, aItemId: libc::int64_t, aProperty: &[u8], aIsAnnotationProperty: bool, aNewValue: &[u8], aLastModified: PRTime, aItemType: libc::uint16_t, aParentId: libc::int64_t, aGuid: &[u8], aParentGuid: &[u8], aOldValue: &[u8], aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aProperty = nsCString::from(aProperty);
        let aNewValue = nsCString::from(aNewValue);
        let aGuid = nsCString::from(aGuid);
        let aParentGuid = nsCString::from(aParentGuid);
        let aOldValue = nsCString::from(aOldValue);
        match ((*self.vtable).onItemChanged)(self as *const _, aItemId, &*aProperty, aIsAnnotationProperty, &*aNewValue, aLastModified, aItemType, aParentId, &*aGuid, &*aParentGuid, &*aOldValue, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onItemVisited (in long long aItemId, in long long aVisitId, in PRTime aTime, in unsigned long aTransitionType, in nsIURI aURI, in long long aParentId, in ACString aGuid, in ACString aParentGuid); */
    #[inline]
    pub unsafe fn onItemVisited(&self, aItemId: libc::int64_t, aVisitId: libc::int64_t, aTime: PRTime, aTransitionType: libc::uint32_t, aURI: Option<&nsIURI>, aParentId: libc::int64_t, aGuid: &[u8], aParentGuid: &[u8]) -> Result<(), nsresult> {
        let aGuid = nsCString::from(aGuid);
        let aParentGuid = nsCString::from(aParentGuid);
        match ((*self.vtable).onItemVisited)(self as *const _, aItemId, aVisitId, aTime, aTransitionType, aURI.map_or(::std::ptr::null(), |x| x as *const _), aParentId, &*aGuid, &*aParentGuid) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onItemMoved (in long long aItemId, in long long aOldParentId, in long aOldIndex, in long long aNewParentId, in long aNewIndex, in unsigned short aItemType, in ACString aGuid, in ACString aOldParentGuid, in ACString aNewParentGuid, in unsigned short aSource); */
    #[inline]
    pub unsafe fn onItemMoved(&self, aItemId: libc::int64_t, aOldParentId: libc::int64_t, aOldIndex: libc::int32_t, aNewParentId: libc::int64_t, aNewIndex: libc::int32_t, aItemType: libc::uint16_t, aGuid: &[u8], aOldParentGuid: &[u8], aNewParentGuid: &[u8], aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aGuid = nsCString::from(aGuid);
        let aOldParentGuid = nsCString::from(aOldParentGuid);
        let aNewParentGuid = nsCString::from(aNewParentGuid);
        match ((*self.vtable).onItemMoved)(self as *const _, aItemId, aOldParentId, aOldIndex, aNewParentId, aNewIndex, aItemType, &*aGuid, &*aOldParentGuid, &*aNewParentGuid, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsINavBookmarksService_consts {
    pub const DEFAULT_INDEX: i64 = -1;
    pub const TYPE_BOOKMARK: i64 = 1;
    pub const TYPE_FOLDER: i64 = 2;
    pub const TYPE_SEPARATOR: i64 = 3;
    pub const TYPE_DYNAMIC_CONTAINER: i64 = 4;
    pub const SOURCE_DEFAULT: i64 = 0;
    pub const SOURCE_SYNC: i64 = 1;
    pub const SOURCE_IMPORT: i64 = 2;
    pub const SOURCE_IMPORT_REPLACE: i64 = 3;
    pub const SOURCE_SYNC_REPARENT_REMOVED_FOLDER_CHILDREN: i64 = 4;
    pub const SYNC_STATUS_UNKNOWN: i64 = 0;
    pub const SYNC_STATUS_NEW: i64 = 1;
    pub const SYNC_STATUS_NORMAL: i64 = 2;
}


#[repr(C)]
pub struct nsINavBookmarksService {
    vtable: *const nsINavBookmarksServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavBookmarksService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x24533891, 0xafa6, 0x4663,
            [0xb7, 0x2d, 0x31, 0x43, 0xd0, 0x3f, 0x1b, 0x04])
    }
}

unsafe impl RefCounted for nsINavBookmarksService {
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
pub trait nsINavBookmarksServiceCoerce {
    fn coerce_from(v: &nsINavBookmarksService) -> &Self;
}

impl nsINavBookmarksServiceCoerce for nsINavBookmarksService {
    #[inline]
    fn coerce_from(v: &nsINavBookmarksService) -> &Self {
        v
    }
}

impl nsINavBookmarksService {
    #[inline]
    pub fn coerce<T: nsINavBookmarksServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavBookmarksService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINavBookmarksServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavBookmarksService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavBookmarksServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long long placesRoot; */
    pub get_placesRoot: unsafe extern "C" fn (this: *const nsINavBookmarksService, aPlacesRoot: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long bookmarksMenuFolder; */
    pub get_bookmarksMenuFolder: unsafe extern "C" fn (this: *const nsINavBookmarksService, aBookmarksMenuFolder: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long tagsFolder; */
    pub get_tagsFolder: unsafe extern "C" fn (this: *const nsINavBookmarksService, aTagsFolder: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long unfiledBookmarksFolder; */
    pub get_unfiledBookmarksFolder: unsafe extern "C" fn (this: *const nsINavBookmarksService, aUnfiledBookmarksFolder: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long toolbarFolder; */
    pub get_toolbarFolder: unsafe extern "C" fn (this: *const nsINavBookmarksService, aToolbarFolder: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long mobileFolder; */
    pub get_mobileFolder: unsafe extern "C" fn (this: *const nsINavBookmarksService, aMobileFolder: *mut libc::int64_t) -> nsresult,

    /* long long insertBookmark (in long long aParentId, in nsIURI aURI, in long aIndex, in AUTF8String aTitle, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
    pub insertBookmark: unsafe extern "C" fn (this: *const nsINavBookmarksService, aParentId: libc::int64_t, aURI: *const nsIURI, aIndex: libc::int32_t, aTitle: *const nsACString, aGuid: *const nsACString, aSource: libc::uint16_t, _retval: *mut libc::int64_t) -> nsresult,

    /* void removeItem (in long long aItemId, [optional] in unsigned short aSource); */
    pub removeItem: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, aSource: libc::uint16_t) -> nsresult,

    /* long long createFolder (in long long aParentFolder, in AUTF8String name, in long index, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
    pub createFolder: unsafe extern "C" fn (this: *const nsINavBookmarksService, aParentFolder: libc::int64_t, name: *const nsACString, index: libc::int32_t, aGuid: *const nsACString, aSource: libc::uint16_t, _retval: *mut libc::int64_t) -> nsresult,

    /* nsITransaction getRemoveFolderTransaction (in long long aItemId, [optional] in unsigned short aSource); */
    pub getRemoveFolderTransaction: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, aSource: libc::uint16_t, _retval: *mut *const nsITransaction) -> nsresult,

    /* void removeFolderChildren (in long long aItemId, [optional] in unsigned short aSource); */
    pub removeFolderChildren: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, aSource: libc::uint16_t) -> nsresult,

    /* void moveItem (in long long aItemId, in long long aNewParentId, in long aIndex, [optional] in unsigned short aSource); */
    pub moveItem: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, aNewParentId: libc::int64_t, aIndex: libc::int32_t, aSource: libc::uint16_t) -> nsresult,

    /* long long insertSeparator (in long long aParentId, in long aIndex, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
    pub insertSeparator: unsafe extern "C" fn (this: *const nsINavBookmarksService, aParentId: libc::int64_t, aIndex: libc::int32_t, aGuid: *const nsACString, aSource: libc::uint16_t, _retval: *mut libc::int64_t) -> nsresult,

    /* long long getIdForItemAt (in long long aParentId, in long aIndex); */
    pub getIdForItemAt: unsafe extern "C" fn (this: *const nsINavBookmarksService, aParentId: libc::int64_t, aIndex: libc::int32_t, _retval: *mut libc::int64_t) -> nsresult,

    /* void setItemTitle (in long long aItemId, in AUTF8String aTitle, [optional] in unsigned short aSource); */
    pub setItemTitle: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, aTitle: *const nsACString, aSource: libc::uint16_t) -> nsresult,

    /* AUTF8String getItemTitle (in long long aItemId); */
    pub getItemTitle: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, _retval: *mut nsACString) -> nsresult,

    /* void setItemDateAdded (in long long aItemId, in PRTime aDateAdded, [optional] in unsigned short aSource); */
    pub setItemDateAdded: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, aDateAdded: PRTime, aSource: libc::uint16_t) -> nsresult,

    /* PRTime getItemDateAdded (in long long aItemId); */
    pub getItemDateAdded: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, _retval: *mut PRTime) -> nsresult,

    /* void setItemLastModified (in long long aItemId, in PRTime aLastModified, [optional] in unsigned short aSource); */
    pub setItemLastModified: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, aLastModified: PRTime, aSource: libc::uint16_t) -> nsresult,

    /* PRTime getItemLastModified (in long long aItemId); */
    pub getItemLastModified: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, _retval: *mut PRTime) -> nsresult,

    /* nsIURI getBookmarkURI (in long long aItemId); */
    pub getBookmarkURI: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, _retval: *mut *const nsIURI) -> nsresult,

    /* long getItemIndex (in long long aItemId); */
    pub getItemIndex: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, _retval: *mut libc::int32_t) -> nsresult,

    /* void setItemIndex (in long long aItemId, in long aNewIndex, [optional] in unsigned short aSource); */
    pub setItemIndex: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, aNewIndex: libc::int32_t, aSource: libc::uint16_t) -> nsresult,

    /* unsigned short getItemType (in long long aItemId); */
    pub getItemType: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, _retval: *mut libc::uint16_t) -> nsresult,

    /* boolean isBookmarked (in nsIURI aURI); */
    pub isBookmarked: unsafe extern "C" fn (this: *const nsINavBookmarksService, aURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* nsIURI getBookmarkedURIFor (in nsIURI aURI); */
    pub getBookmarkedURIFor: unsafe extern "C" fn (this: *const nsINavBookmarksService, aURI: *const nsIURI, _retval: *mut *const nsIURI) -> nsresult,

    /* void changeBookmarkURI (in long long aItemId, in nsIURI aNewURI, [optional] in unsigned short aSource); */
    pub changeBookmarkURI: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, aNewURI: *const nsIURI, aSource: libc::uint16_t) -> nsresult,

    /* long long getFolderIdForItem (in long long aItemId); */
    pub getFolderIdForItem: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, _retval: *mut libc::int64_t) -> nsresult,

    /* void getBookmarkIdsForURI (in nsIURI aURI, [optional] out unsigned long count, [array, size_is (count), retval] out long long bookmarks); */
    /// Unable to call function as its signature contains a non-rust type
    pub getBookmarkIdsForURI: *const ::libc::c_void,

    /* void setKeywordForBookmark (in long long aItemId, in AString aKeyword, [optional] in unsigned short aSource); */
    pub setKeywordForBookmark: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, aKeyword: *const nsAString, aSource: libc::uint16_t) -> nsresult,

    /* AString getKeywordForBookmark (in long long aItemId); */
    pub getKeywordForBookmark: unsafe extern "C" fn (this: *const nsINavBookmarksService, aItemId: libc::int64_t, _retval: *mut nsAString) -> nsresult,

    /* void addObserver (in nsINavBookmarkObserver observer, [optional] in boolean ownsWeak); */
    pub addObserver: unsafe extern "C" fn (this: *const nsINavBookmarksService, observer: *const nsINavBookmarkObserver, ownsWeak: bool) -> nsresult,

    /* void removeObserver (in nsINavBookmarkObserver observer); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsINavBookmarksService, observer: *const nsINavBookmarkObserver) -> nsresult,

    /* void getObservers ([optional] out unsigned long count, [array, size_is (count), retval] out nsINavBookmarkObserver observers); */
    /// Unable to call function as its signature contains a non-rust type
    pub getObservers: *const ::libc::c_void,

    /* void runInBatchMode (in nsINavHistoryBatchCallback aCallback, in nsISupports aUserData); */
    pub runInBatchMode: unsafe extern "C" fn (this: *const nsINavBookmarksService, aCallback: *const nsINavHistoryBatchCallback, aUserData: *const nsISupports) -> nsresult,

}


impl nsINavBookmarksService {
    /* readonly attribute long long placesRoot; */
    #[inline]
    pub unsafe fn get_placesRoot(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_placesRoot)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long bookmarksMenuFolder; */
    #[inline]
    pub unsafe fn get_bookmarksMenuFolder(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_bookmarksMenuFolder)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long tagsFolder; */
    #[inline]
    pub unsafe fn get_tagsFolder(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tagsFolder)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long unfiledBookmarksFolder; */
    #[inline]
    pub unsafe fn get_unfiledBookmarksFolder(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_unfiledBookmarksFolder)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long toolbarFolder; */
    #[inline]
    pub unsafe fn get_toolbarFolder(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_toolbarFolder)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long mobileFolder; */
    #[inline]
    pub unsafe fn get_mobileFolder(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_mobileFolder)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long long insertBookmark (in long long aParentId, in nsIURI aURI, in long aIndex, in AUTF8String aTitle, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn insertBookmark(&self, aParentId: libc::int64_t, aURI: Option<&nsIURI>, aIndex: libc::int32_t, aTitle: &[u8], aGuid: &[u8], aSource: libc::uint16_t) -> Result<libc::int64_t, nsresult> {
        let aTitle = nsCString::from(aTitle);
        let aGuid = nsCString::from(aGuid);
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).insertBookmark)(self as *const _, aParentId, aURI.map_or(::std::ptr::null(), |x| x as *const _), aIndex, &*aTitle, &*aGuid, aSource, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void removeItem (in long long aItemId, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn removeItem(&self, aItemId: libc::int64_t, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeItem)(self as *const _, aItemId, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long long createFolder (in long long aParentFolder, in AUTF8String name, in long index, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn createFolder(&self, aParentFolder: libc::int64_t, name: &[u8], index: libc::int32_t, aGuid: &[u8], aSource: libc::uint16_t) -> Result<libc::int64_t, nsresult> {
        let name = nsCString::from(name);
        let aGuid = nsCString::from(aGuid);
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).createFolder)(self as *const _, aParentFolder, &*name, index, &*aGuid, aSource, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsITransaction getRemoveFolderTransaction (in long long aItemId, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn getRemoveFolderTransaction(&self, aItemId: libc::int64_t, aSource: libc::uint16_t) -> Result<Option<RefPtr<nsITransaction>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRemoveFolderTransaction)(self as *const _, aItemId, aSource, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void removeFolderChildren (in long long aItemId, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn removeFolderChildren(&self, aItemId: libc::int64_t, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeFolderChildren)(self as *const _, aItemId, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void moveItem (in long long aItemId, in long long aNewParentId, in long aIndex, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn moveItem(&self, aItemId: libc::int64_t, aNewParentId: libc::int64_t, aIndex: libc::int32_t, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).moveItem)(self as *const _, aItemId, aNewParentId, aIndex, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long long insertSeparator (in long long aParentId, in long aIndex, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn insertSeparator(&self, aParentId: libc::int64_t, aIndex: libc::int32_t, aGuid: &[u8], aSource: libc::uint16_t) -> Result<libc::int64_t, nsresult> {
        let aGuid = nsCString::from(aGuid);
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).insertSeparator)(self as *const _, aParentId, aIndex, &*aGuid, aSource, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long long getIdForItemAt (in long long aParentId, in long aIndex); */
    #[inline]
    pub unsafe fn getIdForItemAt(&self, aParentId: libc::int64_t, aIndex: libc::int32_t) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).getIdForItemAt)(self as *const _, aParentId, aIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setItemTitle (in long long aItemId, in AUTF8String aTitle, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn setItemTitle(&self, aItemId: libc::int64_t, aTitle: &[u8], aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aTitle = nsCString::from(aTitle);
        match ((*self.vtable).setItemTitle)(self as *const _, aItemId, &*aTitle, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AUTF8String getItemTitle (in long long aItemId); */
    #[inline]
    pub unsafe fn getItemTitle(&self, aItemId: libc::int64_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getItemTitle)(self as *const _, aItemId, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setItemDateAdded (in long long aItemId, in PRTime aDateAdded, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn setItemDateAdded(&self, aItemId: libc::int64_t, aDateAdded: PRTime, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setItemDateAdded)(self as *const _, aItemId, aDateAdded, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* PRTime getItemDateAdded (in long long aItemId); */
    #[inline]
    pub unsafe fn getItemDateAdded(&self, aItemId: libc::int64_t) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).getItemDateAdded)(self as *const _, aItemId, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setItemLastModified (in long long aItemId, in PRTime aLastModified, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn setItemLastModified(&self, aItemId: libc::int64_t, aLastModified: PRTime, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setItemLastModified)(self as *const _, aItemId, aLastModified, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* PRTime getItemLastModified (in long long aItemId); */
    #[inline]
    pub unsafe fn getItemLastModified(&self, aItemId: libc::int64_t) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).getItemLastModified)(self as *const _, aItemId, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIURI getBookmarkURI (in long long aItemId); */
    #[inline]
    pub unsafe fn getBookmarkURI(&self, aItemId: libc::int64_t) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getBookmarkURI)(self as *const _, aItemId, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* long getItemIndex (in long long aItemId); */
    #[inline]
    pub unsafe fn getItemIndex(&self, aItemId: libc::int64_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getItemIndex)(self as *const _, aItemId, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setItemIndex (in long long aItemId, in long aNewIndex, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn setItemIndex(&self, aItemId: libc::int64_t, aNewIndex: libc::int32_t, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setItemIndex)(self as *const _, aItemId, aNewIndex, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned short getItemType (in long long aItemId); */
    #[inline]
    pub unsafe fn getItemType(&self, aItemId: libc::int64_t) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).getItemType)(self as *const _, aItemId, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isBookmarked (in nsIURI aURI); */
    #[inline]
    pub unsafe fn isBookmarked(&self, aURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isBookmarked)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIURI getBookmarkedURIFor (in nsIURI aURI); */
    #[inline]
    pub unsafe fn getBookmarkedURIFor(&self, aURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getBookmarkedURIFor)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void changeBookmarkURI (in long long aItemId, in nsIURI aNewURI, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn changeBookmarkURI(&self, aItemId: libc::int64_t, aNewURI: Option<&nsIURI>, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).changeBookmarkURI)(self as *const _, aItemId, aNewURI.map_or(::std::ptr::null(), |x| x as *const _), aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long long getFolderIdForItem (in long long aItemId); */
    #[inline]
    pub unsafe fn getFolderIdForItem(&self, aItemId: libc::int64_t) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).getFolderIdForItem)(self as *const _, aItemId, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getBookmarkIdsForURI (in nsIURI aURI, [optional] out unsigned long count, [array, size_is (count), retval] out long long bookmarks); */


    /* void setKeywordForBookmark (in long long aItemId, in AString aKeyword, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn setKeywordForBookmark(&self, aItemId: libc::int64_t, aKeyword: &[u16], aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aKeyword = nsString::from(aKeyword);
        match ((*self.vtable).setKeywordForBookmark)(self as *const _, aItemId, &*aKeyword, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getKeywordForBookmark (in long long aItemId); */
    #[inline]
    pub unsafe fn getKeywordForBookmark(&self, aItemId: libc::int64_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getKeywordForBookmark)(self as *const _, aItemId, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addObserver (in nsINavBookmarkObserver observer, [optional] in boolean ownsWeak); */
    #[inline]
    pub unsafe fn addObserver(&self, observer: Option<&nsINavBookmarkObserver>, ownsWeak: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _), ownsWeak) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in nsINavBookmarkObserver observer); */
    #[inline]
    pub unsafe fn removeObserver(&self, observer: Option<&nsINavBookmarkObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getObservers ([optional] out unsigned long count, [array, size_is (count), retval] out nsINavBookmarkObserver observers); */


    /* void runInBatchMode (in nsINavHistoryBatchCallback aCallback, in nsISupports aUserData); */
    #[inline]
    pub unsafe fn runInBatchMode(&self, aCallback: Option<&nsINavHistoryBatchCallback>, aUserData: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).runInBatchMode)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _), aUserData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


