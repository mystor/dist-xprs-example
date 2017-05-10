//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINavHistoryService.idl
//


pub mod nsINavHistoryResultNode_consts {
    pub const RESULT_TYPE_URI: i64 = 0;
    pub const RESULT_TYPE_QUERY: i64 = 5;
    pub const RESULT_TYPE_FOLDER: i64 = 6;
    pub const RESULT_TYPE_SEPARATOR: i64 = 7;
    pub const RESULT_TYPE_FOLDER_SHORTCUT: i64 = 9;
}


#[repr(C)]
pub struct nsINavHistoryResultNode {
    vtable: *const nsINavHistoryResultNodeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryResultNode {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x91d104bb, 0x17ef, 0x404b,
            [0x9f, 0x9a, 0xd9, 0xed, 0x8d, 0xe6, 0x82, 0x4c])
    }
}

unsafe impl RefCounted for nsINavHistoryResultNode {
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
pub trait nsINavHistoryResultNodeCoerce {
    fn coerce_from(v: &nsINavHistoryResultNode) -> &Self;
}

impl nsINavHistoryResultNodeCoerce for nsINavHistoryResultNode {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResultNode) -> &Self {
        v
    }
}

impl nsINavHistoryResultNode {
    #[inline]
    pub fn coerce<T: nsINavHistoryResultNodeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryResultNode {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINavHistoryResultNodeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResultNode) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryResultNodeVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsINavHistoryContainerResultNode parent; */
    pub get_parent: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aParent: *mut *const nsINavHistoryContainerResultNode) -> nsresult,

    /* readonly attribute nsINavHistoryResult parentResult; */
    pub get_parentResult: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aParentResult: *mut *const nsINavHistoryResult) -> nsresult,

    /* readonly attribute AUTF8String uri; */
    pub get_uri: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aUri: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned long type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute AUTF8String title; */
    pub get_title: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aTitle: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned long accessCount; */
    pub get_accessCount: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aAccessCount: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute PRTime time; */
    pub get_time: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aTime: *mut PRTime) -> nsresult,

    /* readonly attribute AUTF8String icon; */
    pub get_icon: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aIcon: *mut nsACString) -> nsresult,

    /* readonly attribute long indentLevel; */
    pub get_indentLevel: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aIndentLevel: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long bookmarkIndex; */
    pub get_bookmarkIndex: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aBookmarkIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long long itemId; */
    pub get_itemId: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aItemId: *mut libc::int64_t) -> nsresult,

    /* readonly attribute PRTime dateAdded; */
    pub get_dateAdded: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aDateAdded: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime lastModified; */
    pub get_lastModified: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aLastModified: *mut PRTime) -> nsresult,

    /* readonly attribute AString tags; */
    pub get_tags: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aTags: *mut nsAString) -> nsresult,

    /* readonly attribute ACString pageGuid; */
    pub get_pageGuid: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aPageGuid: *mut nsACString) -> nsresult,

    /* readonly attribute ACString bookmarkGuid; */
    pub get_bookmarkGuid: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aBookmarkGuid: *mut nsACString) -> nsresult,

    /* readonly attribute long long visitId; */
    pub get_visitId: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aVisitId: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long fromVisitId; */
    pub get_fromVisitId: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aFromVisitId: *mut libc::int64_t) -> nsresult,

    /* readonly attribute unsigned long visitType; */
    pub get_visitType: unsafe extern "C" fn (this: *const nsINavHistoryResultNode, aVisitType: *mut libc::uint32_t) -> nsresult,

}


impl nsINavHistoryResultNode {
    /* readonly attribute nsINavHistoryContainerResultNode parent; */
    #[inline]
    pub unsafe fn get_parent(&self, ) -> Result<Option<RefPtr<nsINavHistoryContainerResultNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsINavHistoryResult parentResult; */
    #[inline]
    pub unsafe fn get_parentResult(&self, ) -> Result<Option<RefPtr<nsINavHistoryResult>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentResult)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AUTF8String uri; */
    #[inline]
    pub unsafe fn get_uri(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_uri)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* readonly attribute AUTF8String title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_title)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long accessCount; */
    #[inline]
    pub unsafe fn get_accessCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_accessCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime time; */
    #[inline]
    pub unsafe fn get_time(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_time)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String icon; */
    #[inline]
    pub unsafe fn get_icon(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_icon)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long indentLevel; */
    #[inline]
    pub unsafe fn get_indentLevel(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_indentLevel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long bookmarkIndex; */
    #[inline]
    pub unsafe fn get_bookmarkIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_bookmarkIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long itemId; */
    #[inline]
    pub unsafe fn get_itemId(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_itemId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime dateAdded; */
    #[inline]
    pub unsafe fn get_dateAdded(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_dateAdded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime lastModified; */
    #[inline]
    pub unsafe fn get_lastModified(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString tags; */
    #[inline]
    pub unsafe fn get_tags(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_tags)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString pageGuid; */
    #[inline]
    pub unsafe fn get_pageGuid(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_pageGuid)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString bookmarkGuid; */
    #[inline]
    pub unsafe fn get_bookmarkGuid(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_bookmarkGuid)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long visitId; */
    #[inline]
    pub unsafe fn get_visitId(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_visitId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long fromVisitId; */
    #[inline]
    pub unsafe fn get_fromVisitId(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fromVisitId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long visitType; */
    #[inline]
    pub unsafe fn get_visitType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_visitType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsINavHistoryContainerResultNode_consts {
    pub const STATE_CLOSED: i64 = 0;
    pub const STATE_LOADING: i64 = 1;
    pub const STATE_OPENED: i64 = 2;
}


#[repr(C)]
pub struct nsINavHistoryContainerResultNode {
    vtable: *const nsINavHistoryContainerResultNodeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryContainerResultNode {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3e9cc95f, 0x0d93, 0x45f1,
            [0x89, 0x4f, 0x90, 0x8e, 0xeb, 0x98, 0x66, 0xd7])
    }
}

unsafe impl RefCounted for nsINavHistoryContainerResultNode {
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
pub trait nsINavHistoryContainerResultNodeCoerce {
    fn coerce_from(v: &nsINavHistoryContainerResultNode) -> &Self;
}

impl nsINavHistoryContainerResultNodeCoerce for nsINavHistoryContainerResultNode {
    #[inline]
    fn coerce_from(v: &nsINavHistoryContainerResultNode) -> &Self {
        v
    }
}

impl nsINavHistoryContainerResultNode {
    #[inline]
    pub fn coerce<T: nsINavHistoryContainerResultNodeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryContainerResultNode {
    type Target = nsINavHistoryResultNode;
    #[inline]
    fn deref(&self) -> &nsINavHistoryResultNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsINavHistoryResultNodeCoerce> nsINavHistoryContainerResultNodeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryContainerResultNode) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryContainerResultNodeVTable {
    pub __base: nsINavHistoryResultNodeVTable,

    /* attribute boolean containerOpen; */
    pub get_containerOpen: unsafe extern "C" fn (this: *const nsINavHistoryContainerResultNode, aContainerOpen: *mut bool) -> nsresult,
    pub set_containerOpen: unsafe extern "C" fn (this: *const nsINavHistoryContainerResultNode, aContainerOpen: bool) -> nsresult,

    /* readonly attribute unsigned short state; */
    pub get_state: unsafe extern "C" fn (this: *const nsINavHistoryContainerResultNode, aState: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute boolean hasChildren; */
    pub get_hasChildren: unsafe extern "C" fn (this: *const nsINavHistoryContainerResultNode, aHasChildren: *mut bool) -> nsresult,

    /* readonly attribute unsigned long childCount; */
    pub get_childCount: unsafe extern "C" fn (this: *const nsINavHistoryContainerResultNode, aChildCount: *mut libc::uint32_t) -> nsresult,

    /* nsINavHistoryResultNode getChild (in unsigned long aIndex); */
    pub getChild: unsafe extern "C" fn (this: *const nsINavHistoryContainerResultNode, aIndex: libc::uint32_t, _retval: *mut *const nsINavHistoryResultNode) -> nsresult,

    /* unsigned long getChildIndex (in nsINavHistoryResultNode aNode); */
    pub getChildIndex: unsafe extern "C" fn (this: *const nsINavHistoryContainerResultNode, aNode: *const nsINavHistoryResultNode, _retval: *mut libc::uint32_t) -> nsresult,

    /* nsINavHistoryResultNode findNodeByDetails (in AUTF8String aURIString, in PRTime aTime, in long long aItemId, in boolean aRecursive); */
    pub findNodeByDetails: unsafe extern "C" fn (this: *const nsINavHistoryContainerResultNode, aURIString: *const nsACString, aTime: PRTime, aItemId: libc::int64_t, aRecursive: bool, _retval: *mut *const nsINavHistoryResultNode) -> nsresult,

}


impl nsINavHistoryContainerResultNode {
    /* attribute boolean containerOpen; */
    #[inline]
    pub unsafe fn get_containerOpen(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_containerOpen)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_containerOpen(&self, aContainerOpen: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_containerOpen)(self as *const _, aContainerOpen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned short state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_state)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean hasChildren; */
    #[inline]
    pub unsafe fn get_hasChildren(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasChildren)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long childCount; */
    #[inline]
    pub unsafe fn get_childCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_childCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsINavHistoryResultNode getChild (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getChild(&self, aIndex: libc::uint32_t) -> Result<Option<RefPtr<nsINavHistoryResultNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChild)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long getChildIndex (in nsINavHistoryResultNode aNode); */
    #[inline]
    pub unsafe fn getChildIndex(&self, aNode: Option<&nsINavHistoryResultNode>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getChildIndex)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsINavHistoryResultNode findNodeByDetails (in AUTF8String aURIString, in PRTime aTime, in long long aItemId, in boolean aRecursive); */
    #[inline]
    pub unsafe fn findNodeByDetails(&self, aURIString: &[u8], aTime: PRTime, aItemId: libc::int64_t, aRecursive: bool) -> Result<Option<RefPtr<nsINavHistoryResultNode>>, nsresult> {
        let aURIString = nsCString::from(aURIString);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findNodeByDetails)(self as *const _, &*aURIString, aTime, aItemId, aRecursive, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsINavHistoryQueryResultNode {
    vtable: *const nsINavHistoryQueryResultNodeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryQueryResultNode {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x62817759, 0x4fee, 0x44a3,
            [0xb5, 0x8c, 0x3e, 0x2f, 0x5a, 0xfc, 0x9d, 0x0a])
    }
}

unsafe impl RefCounted for nsINavHistoryQueryResultNode {
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
pub trait nsINavHistoryQueryResultNodeCoerce {
    fn coerce_from(v: &nsINavHistoryQueryResultNode) -> &Self;
}

impl nsINavHistoryQueryResultNodeCoerce for nsINavHistoryQueryResultNode {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQueryResultNode) -> &Self {
        v
    }
}

impl nsINavHistoryQueryResultNode {
    #[inline]
    pub fn coerce<T: nsINavHistoryQueryResultNodeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryQueryResultNode {
    type Target = nsINavHistoryContainerResultNode;
    #[inline]
    fn deref(&self) -> &nsINavHistoryContainerResultNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsINavHistoryContainerResultNodeCoerce> nsINavHistoryQueryResultNodeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQueryResultNode) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryQueryResultNodeVTable {
    pub __base: nsINavHistoryContainerResultNodeVTable,

    /* void getQueries ([optional] out unsigned long queryCount, [array, size_is (queryCount), retval] out nsINavHistoryQuery queries); */
    /// Unable to call function as its signature contains a non-rust type
    pub getQueries: *const ::libc::c_void,

    /* readonly attribute nsINavHistoryQueryOptions queryOptions; */
    pub get_queryOptions: unsafe extern "C" fn (this: *const nsINavHistoryQueryResultNode, aQueryOptions: *mut *const nsINavHistoryQueryOptions) -> nsresult,

    /* readonly attribute long long folderItemId; */
    pub get_folderItemId: unsafe extern "C" fn (this: *const nsINavHistoryQueryResultNode, aFolderItemId: *mut libc::int64_t) -> nsresult,

    /* readonly attribute ACString targetFolderGuid; */
    pub get_targetFolderGuid: unsafe extern "C" fn (this: *const nsINavHistoryQueryResultNode, aTargetFolderGuid: *mut nsACString) -> nsresult,

}


impl nsINavHistoryQueryResultNode {
    /* void getQueries ([optional] out unsigned long queryCount, [array, size_is (queryCount), retval] out nsINavHistoryQuery queries); */


    /* readonly attribute nsINavHistoryQueryOptions queryOptions; */
    #[inline]
    pub unsafe fn get_queryOptions(&self, ) -> Result<Option<RefPtr<nsINavHistoryQueryOptions>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_queryOptions)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long long folderItemId; */
    #[inline]
    pub unsafe fn get_folderItemId(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_folderItemId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString targetFolderGuid; */
    #[inline]
    pub unsafe fn get_targetFolderGuid(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_targetFolderGuid)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsINavHistoryResultObserver {
    vtable: *const nsINavHistoryResultObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryResultObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf62d8b6b, 0x3c4e, 0x4a9f,
            [0xa8, 0x97, 0xdb, 0x60, 0x5d, 0x0b, 0x7a, 0x0f])
    }
}

unsafe impl RefCounted for nsINavHistoryResultObserver {
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
pub trait nsINavHistoryResultObserverCoerce {
    fn coerce_from(v: &nsINavHistoryResultObserver) -> &Self;
}

impl nsINavHistoryResultObserverCoerce for nsINavHistoryResultObserver {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResultObserver) -> &Self {
        v
    }
}

impl nsINavHistoryResultObserver {
    #[inline]
    pub fn coerce<T: nsINavHistoryResultObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryResultObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINavHistoryResultObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResultObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryResultObserverVTable {
    pub __base: nsISupportsVTable,

    /* void nodeInserted (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aNode, in unsigned long aNewIndex); */
    pub nodeInserted: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aParent: *const nsINavHistoryContainerResultNode, aNode: *const nsINavHistoryResultNode, aNewIndex: libc::uint32_t) -> nsresult,

    /* void nodeRemoved (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aItem, in unsigned long aOldIndex); */
    pub nodeRemoved: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aParent: *const nsINavHistoryContainerResultNode, aItem: *const nsINavHistoryResultNode, aOldIndex: libc::uint32_t) -> nsresult,

    /* void nodeMoved (in nsINavHistoryResultNode aNode, in nsINavHistoryContainerResultNode aOldParent, in unsigned long aOldIndex, in nsINavHistoryContainerResultNode aNewParent, in unsigned long aNewIndex); */
    pub nodeMoved: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aOldParent: *const nsINavHistoryContainerResultNode, aOldIndex: libc::uint32_t, aNewParent: *const nsINavHistoryContainerResultNode, aNewIndex: libc::uint32_t) -> nsresult,

    /* void nodeTitleChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewTitle); */
    pub nodeTitleChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aNewTitle: *const nsACString) -> nsresult,

    /* void nodeURIChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewURI); */
    pub nodeURIChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aNewURI: *const nsACString) -> nsresult,

    /* void nodeIconChanged (in nsINavHistoryResultNode aNode); */
    pub nodeIconChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode) -> nsresult,

    /* void nodeHistoryDetailsChanged (in nsINavHistoryResultNode aNode, in PRTime aNewVisitDate, in unsigned long aNewAccessCount); */
    pub nodeHistoryDetailsChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aNewVisitDate: PRTime, aNewAccessCount: libc::uint32_t) -> nsresult,

    /* void nodeTagsChanged (in nsINavHistoryResultNode aNode); */
    pub nodeTagsChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode) -> nsresult,

    /* void nodeKeywordChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewKeyword); */
    pub nodeKeywordChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aNewKeyword: *const nsACString) -> nsresult,

    /* void nodeAnnotationChanged (in nsINavHistoryResultNode aNode, in AUTF8String aAnnoName); */
    pub nodeAnnotationChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aAnnoName: *const nsACString) -> nsresult,

    /* void nodeDateAddedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
    pub nodeDateAddedChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aNewValue: PRTime) -> nsresult,

    /* void nodeLastModifiedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
    pub nodeLastModifiedChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aNewValue: PRTime) -> nsresult,

    /* void containerStateChanged (in nsINavHistoryContainerResultNode aContainerNode, in unsigned long aOldState, in unsigned long aNewState); */
    pub containerStateChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aContainerNode: *const nsINavHistoryContainerResultNode, aOldState: libc::uint32_t, aNewState: libc::uint32_t) -> nsresult,

    /* void invalidateContainer (in nsINavHistoryContainerResultNode aContainerNode); */
    pub invalidateContainer: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aContainerNode: *const nsINavHistoryContainerResultNode) -> nsresult,

    /* void sortingChanged (in unsigned short sortingMode); */
    pub sortingChanged: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, sortingMode: libc::uint16_t) -> nsresult,

    /* void batching (in boolean aToggleMode); */
    pub batching: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aToggleMode: bool) -> nsresult,

    /* attribute nsINavHistoryResult result; */
    pub get_result: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aResult: *mut *const nsINavHistoryResult) -> nsresult,
    pub set_result: unsafe extern "C" fn (this: *const nsINavHistoryResultObserver, aResult: *const nsINavHistoryResult) -> nsresult,

}


impl nsINavHistoryResultObserver {
    /* void nodeInserted (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aNode, in unsigned long aNewIndex); */
    #[inline]
    pub unsafe fn nodeInserted(&self, aParent: Option<&nsINavHistoryContainerResultNode>, aNode: Option<&nsINavHistoryResultNode>, aNewIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).nodeInserted)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aNode.map_or(::std::ptr::null(), |x| x as *const _), aNewIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeRemoved (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aItem, in unsigned long aOldIndex); */
    #[inline]
    pub unsafe fn nodeRemoved(&self, aParent: Option<&nsINavHistoryContainerResultNode>, aItem: Option<&nsINavHistoryResultNode>, aOldIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).nodeRemoved)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aItem.map_or(::std::ptr::null(), |x| x as *const _), aOldIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeMoved (in nsINavHistoryResultNode aNode, in nsINavHistoryContainerResultNode aOldParent, in unsigned long aOldIndex, in nsINavHistoryContainerResultNode aNewParent, in unsigned long aNewIndex); */
    #[inline]
    pub unsafe fn nodeMoved(&self, aNode: Option<&nsINavHistoryResultNode>, aOldParent: Option<&nsINavHistoryContainerResultNode>, aOldIndex: libc::uint32_t, aNewParent: Option<&nsINavHistoryContainerResultNode>, aNewIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).nodeMoved)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aOldParent.map_or(::std::ptr::null(), |x| x as *const _), aOldIndex, aNewParent.map_or(::std::ptr::null(), |x| x as *const _), aNewIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeTitleChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewTitle); */
    #[inline]
    pub unsafe fn nodeTitleChanged(&self, aNode: Option<&nsINavHistoryResultNode>, aNewTitle: &[u8]) -> Result<(), nsresult> {
        let aNewTitle = nsCString::from(aNewTitle);
        match ((*self.vtable).nodeTitleChanged)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &*aNewTitle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeURIChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewURI); */
    #[inline]
    pub unsafe fn nodeURIChanged(&self, aNode: Option<&nsINavHistoryResultNode>, aNewURI: &[u8]) -> Result<(), nsresult> {
        let aNewURI = nsCString::from(aNewURI);
        match ((*self.vtable).nodeURIChanged)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &*aNewURI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeIconChanged (in nsINavHistoryResultNode aNode); */
    #[inline]
    pub unsafe fn nodeIconChanged(&self, aNode: Option<&nsINavHistoryResultNode>) -> Result<(), nsresult> {

        match ((*self.vtable).nodeIconChanged)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeHistoryDetailsChanged (in nsINavHistoryResultNode aNode, in PRTime aNewVisitDate, in unsigned long aNewAccessCount); */
    #[inline]
    pub unsafe fn nodeHistoryDetailsChanged(&self, aNode: Option<&nsINavHistoryResultNode>, aNewVisitDate: PRTime, aNewAccessCount: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).nodeHistoryDetailsChanged)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aNewVisitDate, aNewAccessCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeTagsChanged (in nsINavHistoryResultNode aNode); */
    #[inline]
    pub unsafe fn nodeTagsChanged(&self, aNode: Option<&nsINavHistoryResultNode>) -> Result<(), nsresult> {

        match ((*self.vtable).nodeTagsChanged)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeKeywordChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewKeyword); */
    #[inline]
    pub unsafe fn nodeKeywordChanged(&self, aNode: Option<&nsINavHistoryResultNode>, aNewKeyword: &[u8]) -> Result<(), nsresult> {
        let aNewKeyword = nsCString::from(aNewKeyword);
        match ((*self.vtable).nodeKeywordChanged)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &*aNewKeyword) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeAnnotationChanged (in nsINavHistoryResultNode aNode, in AUTF8String aAnnoName); */
    #[inline]
    pub unsafe fn nodeAnnotationChanged(&self, aNode: Option<&nsINavHistoryResultNode>, aAnnoName: &[u8]) -> Result<(), nsresult> {
        let aAnnoName = nsCString::from(aAnnoName);
        match ((*self.vtable).nodeAnnotationChanged)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &*aAnnoName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeDateAddedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
    #[inline]
    pub unsafe fn nodeDateAddedChanged(&self, aNode: Option<&nsINavHistoryResultNode>, aNewValue: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).nodeDateAddedChanged)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aNewValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void nodeLastModifiedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
    #[inline]
    pub unsafe fn nodeLastModifiedChanged(&self, aNode: Option<&nsINavHistoryResultNode>, aNewValue: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).nodeLastModifiedChanged)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aNewValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void containerStateChanged (in nsINavHistoryContainerResultNode aContainerNode, in unsigned long aOldState, in unsigned long aNewState); */
    #[inline]
    pub unsafe fn containerStateChanged(&self, aContainerNode: Option<&nsINavHistoryContainerResultNode>, aOldState: libc::uint32_t, aNewState: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).containerStateChanged)(self as *const _, aContainerNode.map_or(::std::ptr::null(), |x| x as *const _), aOldState, aNewState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invalidateContainer (in nsINavHistoryContainerResultNode aContainerNode); */
    #[inline]
    pub unsafe fn invalidateContainer(&self, aContainerNode: Option<&nsINavHistoryContainerResultNode>) -> Result<(), nsresult> {

        match ((*self.vtable).invalidateContainer)(self as *const _, aContainerNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sortingChanged (in unsigned short sortingMode); */
    #[inline]
    pub unsafe fn sortingChanged(&self, sortingMode: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).sortingChanged)(self as *const _, sortingMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void batching (in boolean aToggleMode); */
    #[inline]
    pub unsafe fn batching(&self, aToggleMode: bool) -> Result<(), nsresult> {

        match ((*self.vtable).batching)(self as *const _, aToggleMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsINavHistoryResult result; */
    #[inline]
    pub unsafe fn get_result(&self, ) -> Result<Option<RefPtr<nsINavHistoryResult>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_result)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_result(&self, aResult: Option<&nsINavHistoryResult>) -> Result<(), nsresult> {

        match ((*self.vtable).set_result)(self as *const _, aResult.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsINavHistoryResultTreeViewer_consts {
    pub const INDEX_INVISIBLE: i64 = 4294967295;
}


#[repr(C)]
pub struct nsINavHistoryResultTreeViewer {
    vtable: *const nsINavHistoryResultTreeViewerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryResultTreeViewer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf8b518c0, 0x1faf, 0x11df,
            [0x8a, 0x39, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsINavHistoryResultTreeViewer {
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
pub trait nsINavHistoryResultTreeViewerCoerce {
    fn coerce_from(v: &nsINavHistoryResultTreeViewer) -> &Self;
}

impl nsINavHistoryResultTreeViewerCoerce for nsINavHistoryResultTreeViewer {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResultTreeViewer) -> &Self {
        v
    }
}

impl nsINavHistoryResultTreeViewer {
    #[inline]
    pub fn coerce<T: nsINavHistoryResultTreeViewerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryResultTreeViewer {
    type Target = nsINavHistoryResultObserver;
    #[inline]
    fn deref(&self) -> &nsINavHistoryResultObserver {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsINavHistoryResultObserverCoerce> nsINavHistoryResultTreeViewerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResultTreeViewer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryResultTreeViewerVTable {
    pub __base: nsINavHistoryResultObserverVTable,

    /* nsINavHistoryResultNode nodeForTreeIndex (in unsigned long aIndex); */
    pub nodeForTreeIndex: unsafe extern "C" fn (this: *const nsINavHistoryResultTreeViewer, aIndex: libc::uint32_t, _retval: *mut *const nsINavHistoryResultNode) -> nsresult,

    /* unsigned long treeIndexForNode (in nsINavHistoryResultNode aNode); */
    pub treeIndexForNode: unsafe extern "C" fn (this: *const nsINavHistoryResultTreeViewer, aNode: *const nsINavHistoryResultNode, _retval: *mut libc::uint32_t) -> nsresult,

}


impl nsINavHistoryResultTreeViewer {
    /* nsINavHistoryResultNode nodeForTreeIndex (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn nodeForTreeIndex(&self, aIndex: libc::uint32_t) -> Result<Option<RefPtr<nsINavHistoryResultNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).nodeForTreeIndex)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long treeIndexForNode (in nsINavHistoryResultNode aNode); */
    #[inline]
    pub unsafe fn treeIndexForNode(&self, aNode: Option<&nsINavHistoryResultNode>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).treeIndexForNode)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsINavHistoryResult {
    vtable: *const nsINavHistoryResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc2229ce3, 0x2159, 0x4001,
            [0x85, 0x9c, 0x70, 0x13, 0xc5, 0x2f, 0x76, 0x19])
    }
}

unsafe impl RefCounted for nsINavHistoryResult {
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
pub trait nsINavHistoryResultCoerce {
    fn coerce_from(v: &nsINavHistoryResult) -> &Self;
}

impl nsINavHistoryResultCoerce for nsINavHistoryResult {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResult) -> &Self {
        v
    }
}

impl nsINavHistoryResult {
    #[inline]
    pub fn coerce<T: nsINavHistoryResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINavHistoryResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryResultVTable {
    pub __base: nsISupportsVTable,

    /* attribute unsigned short sortingMode; */
    pub get_sortingMode: unsafe extern "C" fn (this: *const nsINavHistoryResult, aSortingMode: *mut libc::uint16_t) -> nsresult,
    pub set_sortingMode: unsafe extern "C" fn (this: *const nsINavHistoryResult, aSortingMode: libc::uint16_t) -> nsresult,

    /* attribute AUTF8String sortingAnnotation; */
    pub get_sortingAnnotation: unsafe extern "C" fn (this: *const nsINavHistoryResult, aSortingAnnotation: *mut nsACString) -> nsresult,
    pub set_sortingAnnotation: unsafe extern "C" fn (this: *const nsINavHistoryResult, aSortingAnnotation: *const nsACString) -> nsresult,

    /* attribute boolean suppressNotifications; */
    pub get_suppressNotifications: unsafe extern "C" fn (this: *const nsINavHistoryResult, aSuppressNotifications: *mut bool) -> nsresult,
    pub set_suppressNotifications: unsafe extern "C" fn (this: *const nsINavHistoryResult, aSuppressNotifications: bool) -> nsresult,

    /* void addObserver (in nsINavHistoryResultObserver aObserver, [optional] in boolean aOwnsWeak); */
    pub addObserver: unsafe extern "C" fn (this: *const nsINavHistoryResult, aObserver: *const nsINavHistoryResultObserver, aOwnsWeak: bool) -> nsresult,

    /* void removeObserver (in nsINavHistoryResultObserver aObserver); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsINavHistoryResult, aObserver: *const nsINavHistoryResultObserver) -> nsresult,

    /* readonly attribute nsINavHistoryContainerResultNode root; */
    pub get_root: unsafe extern "C" fn (this: *const nsINavHistoryResult, aRoot: *mut *const nsINavHistoryContainerResultNode) -> nsresult,

}


impl nsINavHistoryResult {
    /* attribute unsigned short sortingMode; */
    #[inline]
    pub unsafe fn get_sortingMode(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_sortingMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sortingMode(&self, aSortingMode: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_sortingMode)(self as *const _, aSortingMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String sortingAnnotation; */
    #[inline]
    pub unsafe fn get_sortingAnnotation(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_sortingAnnotation)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sortingAnnotation(&self, aSortingAnnotation: &[u8]) -> Result<(), nsresult> {
        let aSortingAnnotation = nsCString::from(aSortingAnnotation);
        match ((*self.vtable).set_sortingAnnotation)(self as *const _, &*aSortingAnnotation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean suppressNotifications; */
    #[inline]
    pub unsafe fn get_suppressNotifications(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_suppressNotifications)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_suppressNotifications(&self, aSuppressNotifications: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_suppressNotifications)(self as *const _, aSuppressNotifications) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addObserver (in nsINavHistoryResultObserver aObserver, [optional] in boolean aOwnsWeak); */
    #[inline]
    pub unsafe fn addObserver(&self, aObserver: Option<&nsINavHistoryResultObserver>, aOwnsWeak: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _), aOwnsWeak) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in nsINavHistoryResultObserver aObserver); */
    #[inline]
    pub unsafe fn removeObserver(&self, aObserver: Option<&nsINavHistoryResultObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsINavHistoryContainerResultNode root; */
    #[inline]
    pub unsafe fn get_root(&self, ) -> Result<Option<RefPtr<nsINavHistoryContainerResultNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_root)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


pub mod nsINavHistoryObserver_consts {
    pub const REASON_DELETED: i64 = 0;
    pub const REASON_EXPIRED: i64 = 1;
    pub const ATTRIBUTE_FAVICON: i64 = 3;
}


#[repr(C)]
pub struct nsINavHistoryObserver {
    vtable: *const nsINavHistoryObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0f0f45b0, 0x13a1, 0x44ae,
            [0xa0, 0xab, 0xc6, 0x04, 0x6e, 0xc6, 0xd4, 0xda])
    }
}

unsafe impl RefCounted for nsINavHistoryObserver {
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
pub trait nsINavHistoryObserverCoerce {
    fn coerce_from(v: &nsINavHistoryObserver) -> &Self;
}

impl nsINavHistoryObserverCoerce for nsINavHistoryObserver {
    #[inline]
    fn coerce_from(v: &nsINavHistoryObserver) -> &Self {
        v
    }
}

impl nsINavHistoryObserver {
    #[inline]
    pub fn coerce<T: nsINavHistoryObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINavHistoryObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onBeginUpdateBatch (); */
    pub onBeginUpdateBatch: unsafe extern "C" fn (this: *const nsINavHistoryObserver) -> nsresult,

    /* void onEndUpdateBatch (); */
    pub onEndUpdateBatch: unsafe extern "C" fn (this: *const nsINavHistoryObserver) -> nsresult,

    /* void onVisit (in nsIURI aURI, in long long aVisitId, in PRTime aTime, in long long aSessionId, in long long aReferrerVisitId, in unsigned long aTransitionType, in ACString aGuid, in boolean aHidden, in unsigned long aVisitCount, in unsigned long aTyped, in AString aLastKnownTitle); */
    pub onVisit: unsafe extern "C" fn (this: *const nsINavHistoryObserver, aURI: *const nsIURI, aVisitId: libc::int64_t, aTime: PRTime, aSessionId: libc::int64_t, aReferrerVisitId: libc::int64_t, aTransitionType: libc::uint32_t, aGuid: *const nsACString, aHidden: bool, aVisitCount: libc::uint32_t, aTyped: libc::uint32_t, aLastKnownTitle: *const nsAString) -> nsresult,

    /* void onTitleChanged (in nsIURI aURI, in AString aPageTitle, in ACString aGUID); */
    pub onTitleChanged: unsafe extern "C" fn (this: *const nsINavHistoryObserver, aURI: *const nsIURI, aPageTitle: *const nsAString, aGUID: *const nsACString) -> nsresult,

    /* void onFrecencyChanged (in nsIURI aURI, in long aNewFrecency, in ACString aGUID, in boolean aHidden, in PRTime aVisitDate); */
    pub onFrecencyChanged: unsafe extern "C" fn (this: *const nsINavHistoryObserver, aURI: *const nsIURI, aNewFrecency: libc::int32_t, aGUID: *const nsACString, aHidden: bool, aVisitDate: PRTime) -> nsresult,

    /* void onManyFrecenciesChanged (); */
    pub onManyFrecenciesChanged: unsafe extern "C" fn (this: *const nsINavHistoryObserver) -> nsresult,

    /* void onDeleteURI (in nsIURI aURI, in ACString aGUID, in unsigned short aReason); */
    pub onDeleteURI: unsafe extern "C" fn (this: *const nsINavHistoryObserver, aURI: *const nsIURI, aGUID: *const nsACString, aReason: libc::uint16_t) -> nsresult,

    /* void onClearHistory (); */
    pub onClearHistory: unsafe extern "C" fn (this: *const nsINavHistoryObserver) -> nsresult,

    /* void onPageChanged (in nsIURI aURI, in unsigned long aChangedAttribute, in AString aNewValue, in ACString aGUID); */
    pub onPageChanged: unsafe extern "C" fn (this: *const nsINavHistoryObserver, aURI: *const nsIURI, aChangedAttribute: libc::uint32_t, aNewValue: *const nsAString, aGUID: *const nsACString) -> nsresult,

    /* void onDeleteVisits (in nsIURI aURI, in PRTime aVisitTime, in ACString aGUID, in unsigned short aReason, in unsigned long aTransitionType); */
    pub onDeleteVisits: unsafe extern "C" fn (this: *const nsINavHistoryObserver, aURI: *const nsIURI, aVisitTime: PRTime, aGUID: *const nsACString, aReason: libc::uint16_t, aTransitionType: libc::uint32_t) -> nsresult,

}


impl nsINavHistoryObserver {
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

    /* void onVisit (in nsIURI aURI, in long long aVisitId, in PRTime aTime, in long long aSessionId, in long long aReferrerVisitId, in unsigned long aTransitionType, in ACString aGuid, in boolean aHidden, in unsigned long aVisitCount, in unsigned long aTyped, in AString aLastKnownTitle); */
    #[inline]
    pub unsafe fn onVisit(&self, aURI: Option<&nsIURI>, aVisitId: libc::int64_t, aTime: PRTime, aSessionId: libc::int64_t, aReferrerVisitId: libc::int64_t, aTransitionType: libc::uint32_t, aGuid: &[u8], aHidden: bool, aVisitCount: libc::uint32_t, aTyped: libc::uint32_t, aLastKnownTitle: &[u16]) -> Result<(), nsresult> {
        let aGuid = nsCString::from(aGuid);
        let aLastKnownTitle = nsString::from(aLastKnownTitle);
        match ((*self.vtable).onVisit)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aVisitId, aTime, aSessionId, aReferrerVisitId, aTransitionType, &*aGuid, aHidden, aVisitCount, aTyped, &*aLastKnownTitle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onTitleChanged (in nsIURI aURI, in AString aPageTitle, in ACString aGUID); */
    #[inline]
    pub unsafe fn onTitleChanged(&self, aURI: Option<&nsIURI>, aPageTitle: &[u16], aGUID: &[u8]) -> Result<(), nsresult> {
        let aPageTitle = nsString::from(aPageTitle);
        let aGUID = nsCString::from(aGUID);
        match ((*self.vtable).onTitleChanged)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aPageTitle, &*aGUID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onFrecencyChanged (in nsIURI aURI, in long aNewFrecency, in ACString aGUID, in boolean aHidden, in PRTime aVisitDate); */
    #[inline]
    pub unsafe fn onFrecencyChanged(&self, aURI: Option<&nsIURI>, aNewFrecency: libc::int32_t, aGUID: &[u8], aHidden: bool, aVisitDate: PRTime) -> Result<(), nsresult> {
        let aGUID = nsCString::from(aGUID);
        match ((*self.vtable).onFrecencyChanged)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aNewFrecency, &*aGUID, aHidden, aVisitDate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onManyFrecenciesChanged (); */
    #[inline]
    pub unsafe fn onManyFrecenciesChanged(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onManyFrecenciesChanged)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onDeleteURI (in nsIURI aURI, in ACString aGUID, in unsigned short aReason); */
    #[inline]
    pub unsafe fn onDeleteURI(&self, aURI: Option<&nsIURI>, aGUID: &[u8], aReason: libc::uint16_t) -> Result<(), nsresult> {
        let aGUID = nsCString::from(aGUID);
        match ((*self.vtable).onDeleteURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aGUID, aReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onClearHistory (); */
    #[inline]
    pub unsafe fn onClearHistory(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onClearHistory)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onPageChanged (in nsIURI aURI, in unsigned long aChangedAttribute, in AString aNewValue, in ACString aGUID); */
    #[inline]
    pub unsafe fn onPageChanged(&self, aURI: Option<&nsIURI>, aChangedAttribute: libc::uint32_t, aNewValue: &[u16], aGUID: &[u8]) -> Result<(), nsresult> {
        let aNewValue = nsString::from(aNewValue);
        let aGUID = nsCString::from(aGUID);
        match ((*self.vtable).onPageChanged)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aChangedAttribute, &*aNewValue, &*aGUID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onDeleteVisits (in nsIURI aURI, in PRTime aVisitTime, in ACString aGUID, in unsigned short aReason, in unsigned long aTransitionType); */
    #[inline]
    pub unsafe fn onDeleteVisits(&self, aURI: Option<&nsIURI>, aVisitTime: PRTime, aGUID: &[u8], aReason: libc::uint16_t, aTransitionType: libc::uint32_t) -> Result<(), nsresult> {
        let aGUID = nsCString::from(aGUID);
        match ((*self.vtable).onDeleteVisits)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aVisitTime, &*aGUID, aReason, aTransitionType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsINavHistoryQuery_consts {
    pub const TIME_RELATIVE_EPOCH: i64 = 0;
    pub const TIME_RELATIVE_TODAY: i64 = 1;
    pub const TIME_RELATIVE_NOW: i64 = 2;
}


#[repr(C)]
pub struct nsINavHistoryQuery {
    vtable: *const nsINavHistoryQueryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryQuery {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdc87ae79, 0x22f1, 0x4dcf,
            [0x97, 0x5b, 0x85, 0x2b, 0x01, 0xd2, 0x10, 0xcb])
    }
}

unsafe impl RefCounted for nsINavHistoryQuery {
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
pub trait nsINavHistoryQueryCoerce {
    fn coerce_from(v: &nsINavHistoryQuery) -> &Self;
}

impl nsINavHistoryQueryCoerce for nsINavHistoryQuery {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQuery) -> &Self {
        v
    }
}

impl nsINavHistoryQuery {
    #[inline]
    pub fn coerce<T: nsINavHistoryQueryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryQuery {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINavHistoryQueryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQuery) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryQueryVTable {
    pub __base: nsISupportsVTable,

    /* attribute PRTime beginTime; */
    pub get_beginTime: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aBeginTime: *mut PRTime) -> nsresult,
    pub set_beginTime: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aBeginTime: PRTime) -> nsresult,

    /* attribute unsigned long beginTimeReference; */
    pub get_beginTimeReference: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aBeginTimeReference: *mut libc::uint32_t) -> nsresult,
    pub set_beginTimeReference: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aBeginTimeReference: libc::uint32_t) -> nsresult,

    /* readonly attribute boolean hasBeginTime; */
    pub get_hasBeginTime: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aHasBeginTime: *mut bool) -> nsresult,

    /* readonly attribute PRTime absoluteBeginTime; */
    pub get_absoluteBeginTime: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aAbsoluteBeginTime: *mut PRTime) -> nsresult,

    /* attribute PRTime endTime; */
    pub get_endTime: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aEndTime: *mut PRTime) -> nsresult,
    pub set_endTime: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aEndTime: PRTime) -> nsresult,

    /* attribute unsigned long endTimeReference; */
    pub get_endTimeReference: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aEndTimeReference: *mut libc::uint32_t) -> nsresult,
    pub set_endTimeReference: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aEndTimeReference: libc::uint32_t) -> nsresult,

    /* readonly attribute boolean hasEndTime; */
    pub get_hasEndTime: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aHasEndTime: *mut bool) -> nsresult,

    /* readonly attribute PRTime absoluteEndTime; */
    pub get_absoluteEndTime: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aAbsoluteEndTime: *mut PRTime) -> nsresult,

    /* attribute AString searchTerms; */
    pub get_searchTerms: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aSearchTerms: *mut nsAString) -> nsresult,
    pub set_searchTerms: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aSearchTerms: *const nsAString) -> nsresult,

    /* readonly attribute boolean hasSearchTerms; */
    pub get_hasSearchTerms: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aHasSearchTerms: *mut bool) -> nsresult,

    /* attribute long minVisits; */
    pub get_minVisits: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aMinVisits: *mut libc::int32_t) -> nsresult,
    pub set_minVisits: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aMinVisits: libc::int32_t) -> nsresult,

    /* attribute long maxVisits; */
    pub get_maxVisits: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aMaxVisits: *mut libc::int32_t) -> nsresult,
    pub set_maxVisits: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aMaxVisits: libc::int32_t) -> nsresult,

    /* void setTransitions ([array, size_is (count), const] in unsigned long transitions, in unsigned long count); */
    /// Unable to call function as its signature contains a non-rust type
    pub setTransitions: *const ::libc::c_void,

    /* void getTransitions ([optional] out unsigned long count, [array, size_is (count), retval] out unsigned long transitions); */
    /// Unable to call function as its signature contains a non-rust type
    pub getTransitions: *const ::libc::c_void,

    /* readonly attribute unsigned long transitionCount; */
    pub get_transitionCount: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aTransitionCount: *mut libc::uint32_t) -> nsresult,

    /* attribute boolean onlyBookmarked; */
    pub get_onlyBookmarked: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aOnlyBookmarked: *mut bool) -> nsresult,
    pub set_onlyBookmarked: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aOnlyBookmarked: bool) -> nsresult,

    /* attribute boolean domainIsHost; */
    pub get_domainIsHost: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aDomainIsHost: *mut bool) -> nsresult,
    pub set_domainIsHost: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aDomainIsHost: bool) -> nsresult,

    /* attribute AUTF8String domain; */
    pub get_domain: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aDomain: *mut nsACString) -> nsresult,
    pub set_domain: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aDomain: *const nsACString) -> nsresult,

    /* readonly attribute boolean hasDomain; */
    pub get_hasDomain: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aHasDomain: *mut bool) -> nsresult,

    /* attribute nsIURI uri; */
    pub get_uri: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aUri: *mut *const nsIURI) -> nsresult,
    pub set_uri: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aUri: *const nsIURI) -> nsresult,

    /* readonly attribute boolean hasUri; */
    pub get_hasUri: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aHasUri: *mut bool) -> nsresult,

    /* attribute boolean annotationIsNot; */
    pub get_annotationIsNot: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aAnnotationIsNot: *mut bool) -> nsresult,
    pub set_annotationIsNot: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aAnnotationIsNot: bool) -> nsresult,

    /* attribute AUTF8String annotation; */
    pub get_annotation: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aAnnotation: *mut nsACString) -> nsresult,
    pub set_annotation: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aAnnotation: *const nsACString) -> nsresult,

    /* readonly attribute boolean hasAnnotation; */
    pub get_hasAnnotation: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aHasAnnotation: *mut bool) -> nsresult,

    /* attribute nsIVariant tags; */
    pub get_tags: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aTags: *mut *const nsIVariant) -> nsresult,
    pub set_tags: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aTags: *const nsIVariant) -> nsresult,

    /* attribute boolean tagsAreNot; */
    pub get_tagsAreNot: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aTagsAreNot: *mut bool) -> nsresult,
    pub set_tagsAreNot: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aTagsAreNot: bool) -> nsresult,

    /* void getFolders ([optional] out unsigned long count, [array, size_is (count), retval] out long long folders); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFolders: *const ::libc::c_void,

    /* readonly attribute unsigned long folderCount; */
    pub get_folderCount: unsafe extern "C" fn (this: *const nsINavHistoryQuery, aFolderCount: *mut libc::uint32_t) -> nsresult,

    /* void setFolders ([array, size_is (folderCount), const] in long long folders, in unsigned long folderCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub setFolders: *const ::libc::c_void,

    /* nsINavHistoryQuery clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsINavHistoryQuery, _retval: *mut *const nsINavHistoryQuery) -> nsresult,

}


impl nsINavHistoryQuery {
    /* attribute PRTime beginTime; */
    #[inline]
    pub unsafe fn get_beginTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_beginTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_beginTime(&self, aBeginTime: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).set_beginTime)(self as *const _, aBeginTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long beginTimeReference; */
    #[inline]
    pub unsafe fn get_beginTimeReference(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_beginTimeReference)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_beginTimeReference(&self, aBeginTimeReference: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_beginTimeReference)(self as *const _, aBeginTimeReference) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean hasBeginTime; */
    #[inline]
    pub unsafe fn get_hasBeginTime(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasBeginTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime absoluteBeginTime; */
    #[inline]
    pub unsafe fn get_absoluteBeginTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_absoluteBeginTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute PRTime endTime; */
    #[inline]
    pub unsafe fn get_endTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_endTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_endTime(&self, aEndTime: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).set_endTime)(self as *const _, aEndTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long endTimeReference; */
    #[inline]
    pub unsafe fn get_endTimeReference(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_endTimeReference)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_endTimeReference(&self, aEndTimeReference: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_endTimeReference)(self as *const _, aEndTimeReference) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean hasEndTime; */
    #[inline]
    pub unsafe fn get_hasEndTime(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasEndTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime absoluteEndTime; */
    #[inline]
    pub unsafe fn get_absoluteEndTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_absoluteEndTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AString searchTerms; */
    #[inline]
    pub unsafe fn get_searchTerms(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_searchTerms)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_searchTerms(&self, aSearchTerms: &[u16]) -> Result<(), nsresult> {
        let aSearchTerms = nsString::from(aSearchTerms);
        match ((*self.vtable).set_searchTerms)(self as *const _, &*aSearchTerms) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean hasSearchTerms; */
    #[inline]
    pub unsafe fn get_hasSearchTerms(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasSearchTerms)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute long minVisits; */
    #[inline]
    pub unsafe fn get_minVisits(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_minVisits)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_minVisits(&self, aMinVisits: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_minVisits)(self as *const _, aMinVisits) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long maxVisits; */
    #[inline]
    pub unsafe fn get_maxVisits(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_maxVisits)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_maxVisits(&self, aMaxVisits: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_maxVisits)(self as *const _, aMaxVisits) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setTransitions ([array, size_is (count), const] in unsigned long transitions, in unsigned long count); */


    /* void getTransitions ([optional] out unsigned long count, [array, size_is (count), retval] out unsigned long transitions); */


    /* readonly attribute unsigned long transitionCount; */
    #[inline]
    pub unsafe fn get_transitionCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_transitionCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean onlyBookmarked; */
    #[inline]
    pub unsafe fn get_onlyBookmarked(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_onlyBookmarked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_onlyBookmarked(&self, aOnlyBookmarked: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_onlyBookmarked)(self as *const _, aOnlyBookmarked) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean domainIsHost; */
    #[inline]
    pub unsafe fn get_domainIsHost(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_domainIsHost)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_domainIsHost(&self, aDomainIsHost: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_domainIsHost)(self as *const _, aDomainIsHost) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String domain; */
    #[inline]
    pub unsafe fn get_domain(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_domain)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_domain(&self, aDomain: &[u8]) -> Result<(), nsresult> {
        let aDomain = nsCString::from(aDomain);
        match ((*self.vtable).set_domain)(self as *const _, &*aDomain) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean hasDomain; */
    #[inline]
    pub unsafe fn get_hasDomain(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasDomain)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIURI uri; */
    #[inline]
    pub unsafe fn get_uri(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_uri)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_uri(&self, aUri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_uri)(self as *const _, aUri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean hasUri; */
    #[inline]
    pub unsafe fn get_hasUri(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasUri)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean annotationIsNot; */
    #[inline]
    pub unsafe fn get_annotationIsNot(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_annotationIsNot)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_annotationIsNot(&self, aAnnotationIsNot: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_annotationIsNot)(self as *const _, aAnnotationIsNot) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String annotation; */
    #[inline]
    pub unsafe fn get_annotation(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_annotation)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_annotation(&self, aAnnotation: &[u8]) -> Result<(), nsresult> {
        let aAnnotation = nsCString::from(aAnnotation);
        match ((*self.vtable).set_annotation)(self as *const _, &*aAnnotation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean hasAnnotation; */
    #[inline]
    pub unsafe fn get_hasAnnotation(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasAnnotation)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIVariant tags; */
    #[inline]
    pub unsafe fn get_tags(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_tags)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_tags(&self, aTags: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).set_tags)(self as *const _, aTags.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean tagsAreNot; */
    #[inline]
    pub unsafe fn get_tagsAreNot(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_tagsAreNot)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_tagsAreNot(&self, aTagsAreNot: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_tagsAreNot)(self as *const _, aTagsAreNot) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getFolders ([optional] out unsigned long count, [array, size_is (count), retval] out long long folders); */


    /* readonly attribute unsigned long folderCount; */
    #[inline]
    pub unsafe fn get_folderCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_folderCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setFolders ([array, size_is (folderCount), const] in long long folders, in unsigned long folderCount); */


    /* nsINavHistoryQuery clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsINavHistoryQuery>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


pub mod nsINavHistoryQueryOptions_consts {
    pub const SORT_BY_NONE: i64 = 0;
    pub const SORT_BY_TITLE_ASCENDING: i64 = 1;
    pub const SORT_BY_TITLE_DESCENDING: i64 = 2;
    pub const SORT_BY_DATE_ASCENDING: i64 = 3;
    pub const SORT_BY_DATE_DESCENDING: i64 = 4;
    pub const SORT_BY_URI_ASCENDING: i64 = 5;
    pub const SORT_BY_URI_DESCENDING: i64 = 6;
    pub const SORT_BY_VISITCOUNT_ASCENDING: i64 = 7;
    pub const SORT_BY_VISITCOUNT_DESCENDING: i64 = 8;
    pub const SORT_BY_KEYWORD_ASCENDING: i64 = 9;
    pub const SORT_BY_KEYWORD_DESCENDING: i64 = 10;
    pub const SORT_BY_DATEADDED_ASCENDING: i64 = 11;
    pub const SORT_BY_DATEADDED_DESCENDING: i64 = 12;
    pub const SORT_BY_LASTMODIFIED_ASCENDING: i64 = 13;
    pub const SORT_BY_LASTMODIFIED_DESCENDING: i64 = 14;
    pub const SORT_BY_TAGS_ASCENDING: i64 = 17;
    pub const SORT_BY_TAGS_DESCENDING: i64 = 18;
    pub const SORT_BY_ANNOTATION_ASCENDING: i64 = 19;
    pub const SORT_BY_ANNOTATION_DESCENDING: i64 = 20;
    pub const SORT_BY_FRECENCY_ASCENDING: i64 = 21;
    pub const SORT_BY_FRECENCY_DESCENDING: i64 = 22;
    pub const RESULTS_AS_URI: i64 = 0;
    pub const RESULTS_AS_VISIT: i64 = 1;
    pub const RESULTS_AS_FULL_VISIT: i64 = 2;
    pub const RESULTS_AS_DATE_QUERY: i64 = 3;
    pub const RESULTS_AS_SITE_QUERY: i64 = 4;
    pub const RESULTS_AS_DATE_SITE_QUERY: i64 = 5;
    pub const RESULTS_AS_TAG_QUERY: i64 = 6;
    pub const RESULTS_AS_TAG_CONTENTS: i64 = 7;
    pub const QUERY_TYPE_HISTORY: i64 = 0;
    pub const QUERY_TYPE_BOOKMARKS: i64 = 1;
    pub const QUERY_TYPE_UNIFIED: i64 = 2;
}


#[repr(C)]
pub struct nsINavHistoryQueryOptions {
    vtable: *const nsINavHistoryQueryOptionsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryQueryOptions {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8198dfa7, 0x8061, 0x4766,
            [0x95, 0xcb, 0xfa, 0x86, 0xb3, 0xc0, 0x0a, 0x47])
    }
}

unsafe impl RefCounted for nsINavHistoryQueryOptions {
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
pub trait nsINavHistoryQueryOptionsCoerce {
    fn coerce_from(v: &nsINavHistoryQueryOptions) -> &Self;
}

impl nsINavHistoryQueryOptionsCoerce for nsINavHistoryQueryOptions {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQueryOptions) -> &Self {
        v
    }
}

impl nsINavHistoryQueryOptions {
    #[inline]
    pub fn coerce<T: nsINavHistoryQueryOptionsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryQueryOptions {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINavHistoryQueryOptionsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQueryOptions) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryQueryOptionsVTable {
    pub __base: nsISupportsVTable,

    /* attribute unsigned short sortingMode; */
    pub get_sortingMode: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aSortingMode: *mut libc::uint16_t) -> nsresult,
    pub set_sortingMode: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aSortingMode: libc::uint16_t) -> nsresult,

    /* attribute AUTF8String sortingAnnotation; */
    pub get_sortingAnnotation: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aSortingAnnotation: *mut nsACString) -> nsresult,
    pub set_sortingAnnotation: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aSortingAnnotation: *const nsACString) -> nsresult,

    /* attribute unsigned short resultType; */
    pub get_resultType: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aResultType: *mut libc::uint16_t) -> nsresult,
    pub set_resultType: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aResultType: libc::uint16_t) -> nsresult,

    /* attribute boolean excludeItems; */
    pub get_excludeItems: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aExcludeItems: *mut bool) -> nsresult,
    pub set_excludeItems: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aExcludeItems: bool) -> nsresult,

    /* attribute boolean excludeQueries; */
    pub get_excludeQueries: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aExcludeQueries: *mut bool) -> nsresult,
    pub set_excludeQueries: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aExcludeQueries: bool) -> nsresult,

    /* attribute boolean excludeReadOnlyFolders; */
    pub get_excludeReadOnlyFolders: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aExcludeReadOnlyFolders: *mut bool) -> nsresult,
    pub set_excludeReadOnlyFolders: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aExcludeReadOnlyFolders: bool) -> nsresult,

    /* attribute boolean expandQueries; */
    pub get_expandQueries: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aExpandQueries: *mut bool) -> nsresult,
    pub set_expandQueries: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aExpandQueries: bool) -> nsresult,

    /* attribute boolean includeHidden; */
    pub get_includeHidden: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aIncludeHidden: *mut bool) -> nsresult,
    pub set_includeHidden: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aIncludeHidden: bool) -> nsresult,

    /* attribute unsigned long maxResults; */
    pub get_maxResults: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aMaxResults: *mut libc::uint32_t) -> nsresult,
    pub set_maxResults: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aMaxResults: libc::uint32_t) -> nsresult,

    /* attribute unsigned short queryType; */
    pub get_queryType: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aQueryType: *mut libc::uint16_t) -> nsresult,
    pub set_queryType: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aQueryType: libc::uint16_t) -> nsresult,

    /* attribute boolean asyncEnabled; */
    pub get_asyncEnabled: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aAsyncEnabled: *mut bool) -> nsresult,
    pub set_asyncEnabled: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, aAsyncEnabled: bool) -> nsresult,

    /* nsINavHistoryQueryOptions clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsINavHistoryQueryOptions, _retval: *mut *const nsINavHistoryQueryOptions) -> nsresult,

}


impl nsINavHistoryQueryOptions {
    /* attribute unsigned short sortingMode; */
    #[inline]
    pub unsafe fn get_sortingMode(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_sortingMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sortingMode(&self, aSortingMode: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_sortingMode)(self as *const _, aSortingMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String sortingAnnotation; */
    #[inline]
    pub unsafe fn get_sortingAnnotation(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_sortingAnnotation)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sortingAnnotation(&self, aSortingAnnotation: &[u8]) -> Result<(), nsresult> {
        let aSortingAnnotation = nsCString::from(aSortingAnnotation);
        match ((*self.vtable).set_sortingAnnotation)(self as *const _, &*aSortingAnnotation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned short resultType; */
    #[inline]
    pub unsafe fn get_resultType(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_resultType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_resultType(&self, aResultType: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_resultType)(self as *const _, aResultType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean excludeItems; */
    #[inline]
    pub unsafe fn get_excludeItems(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_excludeItems)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_excludeItems(&self, aExcludeItems: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_excludeItems)(self as *const _, aExcludeItems) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean excludeQueries; */
    #[inline]
    pub unsafe fn get_excludeQueries(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_excludeQueries)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_excludeQueries(&self, aExcludeQueries: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_excludeQueries)(self as *const _, aExcludeQueries) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean excludeReadOnlyFolders; */
    #[inline]
    pub unsafe fn get_excludeReadOnlyFolders(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_excludeReadOnlyFolders)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_excludeReadOnlyFolders(&self, aExcludeReadOnlyFolders: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_excludeReadOnlyFolders)(self as *const _, aExcludeReadOnlyFolders) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean expandQueries; */
    #[inline]
    pub unsafe fn get_expandQueries(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_expandQueries)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_expandQueries(&self, aExpandQueries: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_expandQueries)(self as *const _, aExpandQueries) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean includeHidden; */
    #[inline]
    pub unsafe fn get_includeHidden(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_includeHidden)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_includeHidden(&self, aIncludeHidden: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_includeHidden)(self as *const _, aIncludeHidden) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long maxResults; */
    #[inline]
    pub unsafe fn get_maxResults(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_maxResults)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_maxResults(&self, aMaxResults: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_maxResults)(self as *const _, aMaxResults) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned short queryType; */
    #[inline]
    pub unsafe fn get_queryType(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_queryType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_queryType(&self, aQueryType: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_queryType)(self as *const _, aQueryType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean asyncEnabled; */
    #[inline]
    pub unsafe fn get_asyncEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_asyncEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_asyncEnabled(&self, aAsyncEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_asyncEnabled)(self as *const _, aAsyncEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsINavHistoryQueryOptions clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsINavHistoryQueryOptions>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


pub mod nsINavHistoryService_consts {
    pub const TRANSITION_LINK: i64 = 1;
    pub const TRANSITION_TYPED: i64 = 2;
    pub const TRANSITION_BOOKMARK: i64 = 3;
    pub const TRANSITION_EMBED: i64 = 4;
    pub const TRANSITION_REDIRECT_PERMANENT: i64 = 5;
    pub const TRANSITION_REDIRECT_TEMPORARY: i64 = 6;
    pub const TRANSITION_DOWNLOAD: i64 = 7;
    pub const TRANSITION_FRAMED_LINK: i64 = 8;
    pub const TRANSITION_RELOAD: i64 = 9;
    pub const DATABASE_STATUS_OK: i64 = 0;
    pub const DATABASE_STATUS_CREATE: i64 = 1;
    pub const DATABASE_STATUS_CORRUPT: i64 = 2;
    pub const DATABASE_STATUS_UPGRADED: i64 = 3;
}


#[repr(C)]
pub struct nsINavHistoryService {
    vtable: *const nsINavHistoryServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8a1f527e, 0xc9d7, 0x4a51,
            [0xbf, 0x0c, 0xd8, 0x6f, 0x03, 0x79, 0xb7, 0x01])
    }
}

unsafe impl RefCounted for nsINavHistoryService {
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
pub trait nsINavHistoryServiceCoerce {
    fn coerce_from(v: &nsINavHistoryService) -> &Self;
}

impl nsINavHistoryServiceCoerce for nsINavHistoryService {
    #[inline]
    fn coerce_from(v: &nsINavHistoryService) -> &Self {
        v
    }
}

impl nsINavHistoryService {
    #[inline]
    pub fn coerce<T: nsINavHistoryServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINavHistoryServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short databaseStatus; */
    pub get_databaseStatus: unsafe extern "C" fn (this: *const nsINavHistoryService, aDatabaseStatus: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute boolean hasHistoryEntries; */
    pub get_hasHistoryEntries: unsafe extern "C" fn (this: *const nsINavHistoryService, aHasHistoryEntries: *mut bool) -> nsresult,

    /* AString getPageTitle (in nsIURI aURI); */
    pub getPageTitle: unsafe extern "C" fn (this: *const nsINavHistoryService, aURI: *const nsIURI, _retval: *mut nsAString) -> nsresult,

    /* void markPageAsFollowedBookmark (in nsIURI aURI); */
    pub markPageAsFollowedBookmark: unsafe extern "C" fn (this: *const nsINavHistoryService, aURI: *const nsIURI) -> nsresult,

    /* void markPageAsTyped (in nsIURI aURI); */
    pub markPageAsTyped: unsafe extern "C" fn (this: *const nsINavHistoryService, aURI: *const nsIURI) -> nsresult,

    /* void markPageAsFollowedLink (in nsIURI aURI); */
    pub markPageAsFollowedLink: unsafe extern "C" fn (this: *const nsINavHistoryService, aURI: *const nsIURI) -> nsresult,

    /* boolean canAddURI (in nsIURI aURI); */
    pub canAddURI: unsafe extern "C" fn (this: *const nsINavHistoryService, aURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* nsINavHistoryQuery getNewQuery (); */
    pub getNewQuery: unsafe extern "C" fn (this: *const nsINavHistoryService, _retval: *mut *const nsINavHistoryQuery) -> nsresult,

    /* nsINavHistoryQueryOptions getNewQueryOptions (); */
    pub getNewQueryOptions: unsafe extern "C" fn (this: *const nsINavHistoryService, _retval: *mut *const nsINavHistoryQueryOptions) -> nsresult,

    /* nsINavHistoryResult executeQuery (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions options); */
    pub executeQuery: unsafe extern "C" fn (this: *const nsINavHistoryService, aQuery: *const nsINavHistoryQuery, options: *const nsINavHistoryQueryOptions, _retval: *mut *const nsINavHistoryResult) -> nsresult,

    /* nsINavHistoryResult executeQueries ([array, size_is (aQueryCount)] in nsINavHistoryQuery aQueries, in unsigned long aQueryCount, in nsINavHistoryQueryOptions options); */
    /// Unable to call function as its signature contains a non-rust type
    pub executeQueries: *const ::libc::c_void,

    /* void queryStringToQueries (in AUTF8String aQueryString, [array, size_is (aResultCount)] out nsINavHistoryQuery aQueries, out unsigned long aResultCount, out nsINavHistoryQueryOptions options); */
    /// Unable to call function as its signature contains a non-rust type
    pub queryStringToQueries: *const ::libc::c_void,

    /* AUTF8String queriesToQueryString ([array, size_is (aQueryCount)] in nsINavHistoryQuery aQueries, in unsigned long aQueryCount, in nsINavHistoryQueryOptions options); */
    /// Unable to call function as its signature contains a non-rust type
    pub queriesToQueryString: *const ::libc::c_void,

    /* void addObserver (in nsINavHistoryObserver observer, [optional] in boolean ownsWeak); */
    pub addObserver: unsafe extern "C" fn (this: *const nsINavHistoryService, observer: *const nsINavHistoryObserver, ownsWeak: bool) -> nsresult,

    /* void removeObserver (in nsINavHistoryObserver observer); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsINavHistoryService, observer: *const nsINavHistoryObserver) -> nsresult,

    /* void getObservers ([optional] out unsigned long count, [array, size_is (count), retval] out nsINavHistoryObserver observers); */
    /// Unable to call function as its signature contains a non-rust type
    pub getObservers: *const ::libc::c_void,

    /* void runInBatchMode (in nsINavHistoryBatchCallback aCallback, in nsISupports aClosure); */
    pub runInBatchMode: unsafe extern "C" fn (this: *const nsINavHistoryService, aCallback: *const nsINavHistoryBatchCallback, aClosure: *const nsISupports) -> nsresult,

    /* readonly attribute boolean historyDisabled; */
    pub get_historyDisabled: unsafe extern "C" fn (this: *const nsINavHistoryService, aHistoryDisabled: *mut bool) -> nsresult,

    /* void clearEmbedVisits (); */
    pub clearEmbedVisits: unsafe extern "C" fn (this: *const nsINavHistoryService) -> nsresult,

    /* ACString makeGuid (); */
    pub makeGuid: unsafe extern "C" fn (this: *const nsINavHistoryService, _retval: *mut nsACString) -> nsresult,

}


impl nsINavHistoryService {
    /* readonly attribute unsigned short databaseStatus; */
    #[inline]
    pub unsafe fn get_databaseStatus(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_databaseStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean hasHistoryEntries; */
    #[inline]
    pub unsafe fn get_hasHistoryEntries(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasHistoryEntries)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getPageTitle (in nsIURI aURI); */
    #[inline]
    pub unsafe fn getPageTitle(&self, aURI: Option<&nsIURI>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getPageTitle)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void markPageAsFollowedBookmark (in nsIURI aURI); */
    #[inline]
    pub unsafe fn markPageAsFollowedBookmark(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).markPageAsFollowedBookmark)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void markPageAsTyped (in nsIURI aURI); */
    #[inline]
    pub unsafe fn markPageAsTyped(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).markPageAsTyped)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void markPageAsFollowedLink (in nsIURI aURI); */
    #[inline]
    pub unsafe fn markPageAsFollowedLink(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).markPageAsFollowedLink)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean canAddURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn canAddURI(&self, aURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canAddURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsINavHistoryQuery getNewQuery (); */
    #[inline]
    pub unsafe fn getNewQuery(&self, ) -> Result<Option<RefPtr<nsINavHistoryQuery>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNewQuery)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsINavHistoryQueryOptions getNewQueryOptions (); */
    #[inline]
    pub unsafe fn getNewQueryOptions(&self, ) -> Result<Option<RefPtr<nsINavHistoryQueryOptions>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNewQueryOptions)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsINavHistoryResult executeQuery (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions options); */
    #[inline]
    pub unsafe fn executeQuery(&self, aQuery: Option<&nsINavHistoryQuery>, options: Option<&nsINavHistoryQueryOptions>) -> Result<Option<RefPtr<nsINavHistoryResult>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).executeQuery)(self as *const _, aQuery.map_or(::std::ptr::null(), |x| x as *const _), options.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsINavHistoryResult executeQueries ([array, size_is (aQueryCount)] in nsINavHistoryQuery aQueries, in unsigned long aQueryCount, in nsINavHistoryQueryOptions options); */


    /* void queryStringToQueries (in AUTF8String aQueryString, [array, size_is (aResultCount)] out nsINavHistoryQuery aQueries, out unsigned long aResultCount, out nsINavHistoryQueryOptions options); */


    /* AUTF8String queriesToQueryString ([array, size_is (aQueryCount)] in nsINavHistoryQuery aQueries, in unsigned long aQueryCount, in nsINavHistoryQueryOptions options); */


    /* void addObserver (in nsINavHistoryObserver observer, [optional] in boolean ownsWeak); */
    #[inline]
    pub unsafe fn addObserver(&self, observer: Option<&nsINavHistoryObserver>, ownsWeak: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _), ownsWeak) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in nsINavHistoryObserver observer); */
    #[inline]
    pub unsafe fn removeObserver(&self, observer: Option<&nsINavHistoryObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getObservers ([optional] out unsigned long count, [array, size_is (count), retval] out nsINavHistoryObserver observers); */


    /* void runInBatchMode (in nsINavHistoryBatchCallback aCallback, in nsISupports aClosure); */
    #[inline]
    pub unsafe fn runInBatchMode(&self, aCallback: Option<&nsINavHistoryBatchCallback>, aClosure: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).runInBatchMode)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _), aClosure.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean historyDisabled; */
    #[inline]
    pub unsafe fn get_historyDisabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_historyDisabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void clearEmbedVisits (); */
    #[inline]
    pub unsafe fn clearEmbedVisits(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearEmbedVisits)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* ACString makeGuid (); */
    #[inline]
    pub unsafe fn makeGuid(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).makeGuid)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsINavHistoryBatchCallback {
    vtable: *const nsINavHistoryBatchCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINavHistoryBatchCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5a5a9154, 0x95ac, 0x4e3d,
            [0x90, 0xdf, 0x55, 0x88, 0x16, 0x29, 0x74, 0x07])
    }
}

unsafe impl RefCounted for nsINavHistoryBatchCallback {
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
pub trait nsINavHistoryBatchCallbackCoerce {
    fn coerce_from(v: &nsINavHistoryBatchCallback) -> &Self;
}

impl nsINavHistoryBatchCallbackCoerce for nsINavHistoryBatchCallback {
    #[inline]
    fn coerce_from(v: &nsINavHistoryBatchCallback) -> &Self {
        v
    }
}

impl nsINavHistoryBatchCallback {
    #[inline]
    pub fn coerce<T: nsINavHistoryBatchCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINavHistoryBatchCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINavHistoryBatchCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryBatchCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINavHistoryBatchCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void runBatched (in nsISupports aUserData); */
    pub runBatched: unsafe extern "C" fn (this: *const nsINavHistoryBatchCallback, aUserData: *const nsISupports) -> nsresult,

}


impl nsINavHistoryBatchCallback {
    /* void runBatched (in nsISupports aUserData); */
    #[inline]
    pub unsafe fn runBatched(&self, aUserData: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).runBatched)(self as *const _, aUserData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


