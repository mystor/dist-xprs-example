//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICertTree.idl
//


#[repr(C)]
pub struct nsICertTreeItem {
    vtable: *const nsICertTreeItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICertTreeItem {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd0180863, 0x606e, 0x49e6,
            [0x83, 0x24, 0xcf, 0x45, 0xed, 0x4d, 0xd8, 0x91])
    }
}

unsafe impl RefCounted for nsICertTreeItem {
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
pub trait nsICertTreeItemCoerce {
    fn coerce_from(v: &nsICertTreeItem) -> &Self;
}

impl nsICertTreeItemCoerce for nsICertTreeItem {
    #[inline]
    fn coerce_from(v: &nsICertTreeItem) -> &Self {
        v
    }
}

impl nsICertTreeItem {
    #[inline]
    pub fn coerce<T: nsICertTreeItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICertTreeItem {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICertTreeItemCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertTreeItem) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICertTreeItemVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIX509Cert cert; */
    pub get_cert: unsafe extern "C" fn (this: *const nsICertTreeItem, aCert: *mut *const nsIX509Cert) -> nsresult,

    /* readonly attribute AString hostPort; */
    pub get_hostPort: unsafe extern "C" fn (this: *const nsICertTreeItem, aHostPort: *mut nsAString) -> nsresult,

}


impl nsICertTreeItem {
    /* readonly attribute nsIX509Cert cert; */
    #[inline]
    pub unsafe fn get_cert(&self, ) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_cert)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString hostPort; */
    #[inline]
    pub unsafe fn get_hostPort(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_hostPort)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsICertTree {
    vtable: *const nsICertTreeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICertTree {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x55d5ad6b, 0x5572, 0x47fe,
            [0x94, 0x1c, 0xf0, 0x1f, 0xe7, 0x23, 0x65, 0x9e])
    }
}

unsafe impl RefCounted for nsICertTree {
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
pub trait nsICertTreeCoerce {
    fn coerce_from(v: &nsICertTree) -> &Self;
}

impl nsICertTreeCoerce for nsICertTree {
    #[inline]
    fn coerce_from(v: &nsICertTree) -> &Self {
        v
    }
}

impl nsICertTree {
    #[inline]
    pub fn coerce<T: nsICertTreeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICertTree {
    type Target = nsITreeView;
    #[inline]
    fn deref(&self) -> &nsITreeView {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsITreeViewCoerce> nsICertTreeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertTree) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICertTreeVTable {
    pub __base: nsITreeViewVTable,

    /* void loadCerts (in unsigned long type); */
    pub loadCerts: unsafe extern "C" fn (this: *const nsICertTree, type_: libc::uint32_t) -> nsresult,

    /* void loadCertsFromCache (in nsIX509CertList cache, in unsigned long type); */
    pub loadCertsFromCache: unsafe extern "C" fn (this: *const nsICertTree, cache: *const nsIX509CertList, type_: libc::uint32_t) -> nsresult,

    /* nsIX509Cert getCert (in unsigned long index); */
    pub getCert: unsafe extern "C" fn (this: *const nsICertTree, index: libc::uint32_t, _retval: *mut *const nsIX509Cert) -> nsresult,

    /* nsICertTreeItem getTreeItem (in unsigned long index); */
    pub getTreeItem: unsafe extern "C" fn (this: *const nsICertTree, index: libc::uint32_t, _retval: *mut *const nsICertTreeItem) -> nsresult,

    /* void deleteEntryObject (in unsigned long index); */
    pub deleteEntryObject: unsafe extern "C" fn (this: *const nsICertTree, index: libc::uint32_t) -> nsresult,

}


impl nsICertTree {
    /* void loadCerts (in unsigned long type); */
    #[inline]
    pub unsafe fn loadCerts(&self, type_: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).loadCerts)(self as *const _, type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void loadCertsFromCache (in nsIX509CertList cache, in unsigned long type); */
    #[inline]
    pub unsafe fn loadCertsFromCache(&self, cache: Option<&nsIX509CertList>, type_: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).loadCertsFromCache)(self as *const _, cache.map_or(::std::ptr::null(), |x| x as *const _), type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIX509Cert getCert (in unsigned long index); */
    #[inline]
    pub unsafe fn getCert(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCert)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsICertTreeItem getTreeItem (in unsigned long index); */
    #[inline]
    pub unsafe fn getTreeItem(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsICertTreeItem>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getTreeItem)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void deleteEntryObject (in unsigned long index); */
    #[inline]
    pub unsafe fn deleteEntryObject(&self, index: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).deleteEntryObject)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


