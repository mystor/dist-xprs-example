//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocShellTreeOwner.idl
//


#[repr(C)]
pub struct nsIDocShellTreeOwner {
    vtable: *const nsIDocShellTreeOwnerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDocShellTreeOwner {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0e3dc4b1, 0x4cea, 0x4a37,
            [0xaf, 0x71, 0x79, 0xf0, 0xaf, 0xd0, 0x75, 0x74])
    }
}

unsafe impl RefCounted for nsIDocShellTreeOwner {
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
pub trait nsIDocShellTreeOwnerCoerce {
    fn coerce_from(v: &nsIDocShellTreeOwner) -> &Self;
}

impl nsIDocShellTreeOwnerCoerce for nsIDocShellTreeOwner {
    #[inline]
    fn coerce_from(v: &nsIDocShellTreeOwner) -> &Self {
        v
    }
}

impl nsIDocShellTreeOwner {
    #[inline]
    pub fn coerce<T: nsIDocShellTreeOwnerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDocShellTreeOwner {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDocShellTreeOwnerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocShellTreeOwner) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDocShellTreeOwnerVTable {
    pub __base: nsISupportsVTable,

    /* void contentShellAdded (in nsIDocShellTreeItem aContentShell, in boolean aPrimary); */
    pub contentShellAdded: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, aContentShell: *const nsIDocShellTreeItem, aPrimary: bool) -> nsresult,

    /* void contentShellRemoved (in nsIDocShellTreeItem aContentShell); */
    pub contentShellRemoved: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, aContentShell: *const nsIDocShellTreeItem) -> nsresult,

    /* readonly attribute nsIDocShellTreeItem primaryContentShell; */
    pub get_primaryContentShell: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, aPrimaryContentShell: *mut *const nsIDocShellTreeItem) -> nsresult,

    /* void tabParentAdded (in nsITabParent aTab, in boolean aPrimary); */
    pub tabParentAdded: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, aTab: *const nsITabParent, aPrimary: bool) -> nsresult,

    /* void tabParentRemoved (in nsITabParent aTab); */
    pub tabParentRemoved: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, aTab: *const nsITabParent) -> nsresult,

    /* readonly attribute nsITabParent primaryTabParent; */
    pub get_primaryTabParent: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, aPrimaryTabParent: *mut *const nsITabParent) -> nsresult,

    /* void sizeShellTo (in nsIDocShellTreeItem shell, in long cx, in long cy); */
    pub sizeShellTo: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, shell: *const nsIDocShellTreeItem, cx: libc::int32_t, cy: libc::int32_t) -> nsresult,

    /* void getPrimaryContentSize (out long width, out long height); */
    pub getPrimaryContentSize: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* void setPrimaryContentSize (in long width, in long height); */
    pub setPrimaryContentSize: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, width: libc::int32_t, height: libc::int32_t) -> nsresult,

    /* void getRootShellSize (out long width, out long height); */
    pub getRootShellSize: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* void setRootShellSize (in long width, in long height); */
    pub setRootShellSize: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, width: libc::int32_t, height: libc::int32_t) -> nsresult,

    /* void setPersistence (in boolean aPersistPosition, in boolean aPersistSize, in boolean aPersistSizeMode); */
    pub setPersistence: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, aPersistPosition: bool, aPersistSize: bool, aPersistSizeMode: bool) -> nsresult,

    /* void getPersistence (out boolean aPersistPosition, out boolean aPersistSize, out boolean aPersistSizeMode); */
    pub getPersistence: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, aPersistPosition: *mut bool, aPersistSize: *mut bool, aPersistSizeMode: *mut bool) -> nsresult,

    /* readonly attribute unsigned long tabCount; */
    pub get_tabCount: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, aTabCount: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute bool hasPrimaryContent; */
    pub get_hasPrimaryContent: unsafe extern "C" fn (this: *const nsIDocShellTreeOwner, aHasPrimaryContent: *mut bool) -> nsresult,

}


impl nsIDocShellTreeOwner {
    /* void contentShellAdded (in nsIDocShellTreeItem aContentShell, in boolean aPrimary); */
    #[inline]
    pub unsafe fn contentShellAdded(&self, aContentShell: Option<&nsIDocShellTreeItem>, aPrimary: bool) -> Result<(), nsresult> {

        match ((*self.vtable).contentShellAdded)(self as *const _, aContentShell.map_or(::std::ptr::null(), |x| x as *const _), aPrimary) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void contentShellRemoved (in nsIDocShellTreeItem aContentShell); */
    #[inline]
    pub unsafe fn contentShellRemoved(&self, aContentShell: Option<&nsIDocShellTreeItem>) -> Result<(), nsresult> {

        match ((*self.vtable).contentShellRemoved)(self as *const _, aContentShell.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDocShellTreeItem primaryContentShell; */
    #[inline]
    pub unsafe fn get_primaryContentShell(&self, ) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_primaryContentShell)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void tabParentAdded (in nsITabParent aTab, in boolean aPrimary); */
    #[inline]
    pub unsafe fn tabParentAdded(&self, aTab: Option<&nsITabParent>, aPrimary: bool) -> Result<(), nsresult> {

        match ((*self.vtable).tabParentAdded)(self as *const _, aTab.map_or(::std::ptr::null(), |x| x as *const _), aPrimary) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void tabParentRemoved (in nsITabParent aTab); */
    #[inline]
    pub unsafe fn tabParentRemoved(&self, aTab: Option<&nsITabParent>) -> Result<(), nsresult> {

        match ((*self.vtable).tabParentRemoved)(self as *const _, aTab.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsITabParent primaryTabParent; */
    #[inline]
    pub unsafe fn get_primaryTabParent(&self, ) -> Result<Option<RefPtr<nsITabParent>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_primaryTabParent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void sizeShellTo (in nsIDocShellTreeItem shell, in long cx, in long cy); */
    #[inline]
    pub unsafe fn sizeShellTo(&self, shell: Option<&nsIDocShellTreeItem>, cx: libc::int32_t, cy: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).sizeShellTo)(self as *const _, shell.map_or(::std::ptr::null(), |x| x as *const _), cx, cy) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getPrimaryContentSize (out long width, out long height); */
    #[inline]
    pub unsafe fn getPrimaryContentSize(&self, ) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getPrimaryContentSize)(self as *const _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((width, height))
    }

    /* void setPrimaryContentSize (in long width, in long height); */
    #[inline]
    pub unsafe fn setPrimaryContentSize(&self, width: libc::int32_t, height: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setPrimaryContentSize)(self as *const _, width, height) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getRootShellSize (out long width, out long height); */
    #[inline]
    pub unsafe fn getRootShellSize(&self, ) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRootShellSize)(self as *const _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((width, height))
    }

    /* void setRootShellSize (in long width, in long height); */
    #[inline]
    pub unsafe fn setRootShellSize(&self, width: libc::int32_t, height: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setRootShellSize)(self as *const _, width, height) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPersistence (in boolean aPersistPosition, in boolean aPersistSize, in boolean aPersistSizeMode); */
    #[inline]
    pub unsafe fn setPersistence(&self, aPersistPosition: bool, aPersistSize: bool, aPersistSizeMode: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setPersistence)(self as *const _, aPersistPosition, aPersistSize, aPersistSizeMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getPersistence (out boolean aPersistPosition, out boolean aPersistSize, out boolean aPersistSizeMode); */
    #[inline]
    pub unsafe fn getPersistence(&self, ) -> Result<(bool, bool, bool), nsresult> {
        let mut aPersistPosition: bool = ::std::mem::zeroed();
        let mut aPersistSize: bool = ::std::mem::zeroed();
        let mut aPersistSizeMode: bool = ::std::mem::zeroed();
        match ((*self.vtable).getPersistence)(self as *const _, &mut aPersistPosition as *mut _, &mut aPersistSize as *mut _, &mut aPersistSizeMode as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aPersistPosition, aPersistSize, aPersistSizeMode))
    }

    /* readonly attribute unsigned long tabCount; */
    #[inline]
    pub unsafe fn get_tabCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tabCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool hasPrimaryContent; */
    #[inline]
    pub unsafe fn get_hasPrimaryContent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasPrimaryContent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


