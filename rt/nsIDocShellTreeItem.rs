//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocShellTreeItem.idl
//


pub mod nsIDocShellTreeItem_consts {
    pub const typeChrome: i64 = 0;
    pub const typeContent: i64 = 1;
    pub const typeContentWrapper: i64 = 2;
    pub const typeChromeWrapper: i64 = 3;
    pub const typeAll: i64 = 2147483647;
}


#[repr(C)]
pub struct nsIDocShellTreeItem {
    vtable: *const nsIDocShellTreeItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDocShellTreeItem {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9b7c586f, 0x9214, 0x480c,
            [0xa2, 0xc4, 0x49, 0xb5, 0x26, 0xff, 0xf1, 0xa6])
    }
}

unsafe impl RefCounted for nsIDocShellTreeItem {
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
pub trait nsIDocShellTreeItemCoerce {
    fn coerce_from(v: &nsIDocShellTreeItem) -> &Self;
}

impl nsIDocShellTreeItemCoerce for nsIDocShellTreeItem {
    #[inline]
    fn coerce_from(v: &nsIDocShellTreeItem) -> &Self {
        v
    }
}

impl nsIDocShellTreeItem {
    #[inline]
    pub fn coerce<T: nsIDocShellTreeItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDocShellTreeItem {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDocShellTreeItemCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocShellTreeItem) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDocShellTreeItemVTable {
    pub __base: nsISupportsVTable,

    /* attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aName: *const nsAString) -> nsresult,

    /* boolean nameEquals (in AString name); */
    pub nameEquals: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, name: *const nsAString, _retval: *mut bool) -> nsresult,

    /* attribute long itemType; */
    pub get_itemType: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aItemType: *mut libc::int32_t) -> nsresult,
    pub set_itemType: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aItemType: libc::int32_t) -> nsresult,

    /* [noscript,nostdcall,notxpcom] long ItemType (); */
    pub ItemType: unsafe extern "C" fn (this: *const nsIDocShellTreeItem) -> libc::int32_t,

    /* readonly attribute nsIDocShellTreeItem parent; */
    pub get_parent: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aParent: *mut *const nsIDocShellTreeItem) -> nsresult,

    /* readonly attribute nsIDocShellTreeItem sameTypeParent; */
    pub get_sameTypeParent: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aSameTypeParent: *mut *const nsIDocShellTreeItem) -> nsresult,

    /* readonly attribute nsIDocShellTreeItem rootTreeItem; */
    pub get_rootTreeItem: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aRootTreeItem: *mut *const nsIDocShellTreeItem) -> nsresult,

    /* readonly attribute nsIDocShellTreeItem sameTypeRootTreeItem; */
    pub get_sameTypeRootTreeItem: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aSameTypeRootTreeItem: *mut *const nsIDocShellTreeItem) -> nsresult,

    /* nsIDocShellTreeItem findItemWithName (in AString name, in nsIDocShellTreeItem aRequestor, in nsIDocShellTreeItem aOriginalRequestor, in bool aSkipTabGroup); */
    pub findItemWithName: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, name: *const nsAString, aRequestor: *const nsIDocShellTreeItem, aOriginalRequestor: *const nsIDocShellTreeItem, aSkipTabGroup: bool, _retval: *mut *const nsIDocShellTreeItem) -> nsresult,

    /* readonly attribute nsIDocShellTreeOwner treeOwner; */
    pub get_treeOwner: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aTreeOwner: *mut *const nsIDocShellTreeOwner) -> nsresult,

    /* [noscript] void setTreeOwner (in nsIDocShellTreeOwner treeOwner); */
    pub setTreeOwner: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, treeOwner: *const nsIDocShellTreeOwner) -> nsresult,

    /* readonly attribute long childCount; */
    pub get_childCount: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aChildCount: *mut libc::int32_t) -> nsresult,

    /* void addChild (in nsIDocShellTreeItem child); */
    pub addChild: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, child: *const nsIDocShellTreeItem) -> nsresult,

    /* void removeChild (in nsIDocShellTreeItem child); */
    pub removeChild: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, child: *const nsIDocShellTreeItem) -> nsresult,

    /* nsIDocShellTreeItem getChildAt (in long index); */
    pub getChildAt: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, index: libc::int32_t, _retval: *mut *const nsIDocShellTreeItem) -> nsresult,

    /* nsIDocShellTreeItem findChildWithName (in AString aName, in boolean aRecurse, in boolean aSameType, in nsIDocShellTreeItem aRequestor, in nsIDocShellTreeItem aOriginalRequestor); */
    pub findChildWithName: unsafe extern "C" fn (this: *const nsIDocShellTreeItem, aName: *const nsAString, aRecurse: bool, aSameType: bool, aRequestor: *const nsIDocShellTreeItem, aOriginalRequestor: *const nsIDocShellTreeItem, _retval: *mut *const nsIDocShellTreeItem) -> nsresult,

    /* [noscript,nostdcall,notxpcom] nsIDocument getDocument (); */
    pub getDocument: unsafe extern "C" fn (this: *const nsIDocShellTreeItem) -> *const nsIDocument,

    /* [noscript,nostdcall,notxpcom] nsPIDOMWindowOuter getWindow (); */
    pub getWindow: unsafe extern "C" fn (this: *const nsIDocShellTreeItem) -> *const nsPIDOMWindowOuter,

}


impl nsIDocShellTreeItem {
    /* attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean nameEquals (in AString name); */
    #[inline]
    pub unsafe fn nameEquals(&self, name: &[u16]) -> Result<bool, nsresult> {
        let name = nsString::from(name);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).nameEquals)(self as *const _, &*name, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute long itemType; */
    #[inline]
    pub unsafe fn get_itemType(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_itemType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_itemType(&self, aItemType: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_itemType)(self as *const _, aItemType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,nostdcall,notxpcom] long ItemType (); */
    #[inline]
    pub unsafe fn ItemType(&self, ) -> libc::int32_t {

        let _retval = ((*self.vtable).ItemType)(self as *const _, );
        _retval
    }

    /* readonly attribute nsIDocShellTreeItem parent; */
    #[inline]
    pub unsafe fn get_parent(&self, ) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDocShellTreeItem sameTypeParent; */
    #[inline]
    pub unsafe fn get_sameTypeParent(&self, ) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sameTypeParent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDocShellTreeItem rootTreeItem; */
    #[inline]
    pub unsafe fn get_rootTreeItem(&self, ) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rootTreeItem)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDocShellTreeItem sameTypeRootTreeItem; */
    #[inline]
    pub unsafe fn get_sameTypeRootTreeItem(&self, ) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sameTypeRootTreeItem)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDocShellTreeItem findItemWithName (in AString name, in nsIDocShellTreeItem aRequestor, in nsIDocShellTreeItem aOriginalRequestor, in bool aSkipTabGroup); */
    #[inline]
    pub unsafe fn findItemWithName(&self, name: &[u16], aRequestor: Option<&nsIDocShellTreeItem>, aOriginalRequestor: Option<&nsIDocShellTreeItem>, aSkipTabGroup: bool) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findItemWithName)(self as *const _, &*name, aRequestor.map_or(::std::ptr::null(), |x| x as *const _), aOriginalRequestor.map_or(::std::ptr::null(), |x| x as *const _), aSkipTabGroup, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDocShellTreeOwner treeOwner; */
    #[inline]
    pub unsafe fn get_treeOwner(&self, ) -> Result<Option<RefPtr<nsIDocShellTreeOwner>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_treeOwner)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void setTreeOwner (in nsIDocShellTreeOwner treeOwner); */
    #[inline]
    pub unsafe fn setTreeOwner(&self, treeOwner: Option<&nsIDocShellTreeOwner>) -> Result<(), nsresult> {

        match ((*self.vtable).setTreeOwner)(self as *const _, treeOwner.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long childCount; */
    #[inline]
    pub unsafe fn get_childCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_childCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addChild (in nsIDocShellTreeItem child); */
    #[inline]
    pub unsafe fn addChild(&self, child: Option<&nsIDocShellTreeItem>) -> Result<(), nsresult> {

        match ((*self.vtable).addChild)(self as *const _, child.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeChild (in nsIDocShellTreeItem child); */
    #[inline]
    pub unsafe fn removeChild(&self, child: Option<&nsIDocShellTreeItem>) -> Result<(), nsresult> {

        match ((*self.vtable).removeChild)(self as *const _, child.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDocShellTreeItem getChildAt (in long index); */
    #[inline]
    pub unsafe fn getChildAt(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChildAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDocShellTreeItem findChildWithName (in AString aName, in boolean aRecurse, in boolean aSameType, in nsIDocShellTreeItem aRequestor, in nsIDocShellTreeItem aOriginalRequestor); */
    #[inline]
    pub unsafe fn findChildWithName(&self, aName: &[u16], aRecurse: bool, aSameType: bool, aRequestor: Option<&nsIDocShellTreeItem>, aOriginalRequestor: Option<&nsIDocShellTreeItem>) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findChildWithName)(self as *const _, &*aName, aRecurse, aSameType, aRequestor.map_or(::std::ptr::null(), |x| x as *const _), aOriginalRequestor.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript,nostdcall,notxpcom] nsIDocument getDocument (); */
    #[inline]
    pub unsafe fn getDocument(&self, ) -> Option<RefPtr<nsIDocument>> {

        let _retval = ((*self.vtable).getDocument)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

    /* [noscript,nostdcall,notxpcom] nsPIDOMWindowOuter getWindow (); */
    #[inline]
    pub unsafe fn getWindow(&self, ) -> Option<RefPtr<nsPIDOMWindowOuter>> {

        let _retval = ((*self.vtable).getWindow)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

}


