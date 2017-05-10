//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICategoryManager.idl
//


#[repr(C)]
pub struct nsICategoryManager {
    vtable: *const nsICategoryManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICategoryManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3275b2cd, 0xaf6d, 0x429a,
            [0x80, 0xd7, 0xf0, 0xc5, 0x12, 0x03, 0x42, 0xac])
    }
}

unsafe impl RefCounted for nsICategoryManager {
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
pub trait nsICategoryManagerCoerce {
    fn coerce_from(v: &nsICategoryManager) -> &Self;
}

impl nsICategoryManagerCoerce for nsICategoryManager {
    #[inline]
    fn coerce_from(v: &nsICategoryManager) -> &Self {
        v
    }
}

impl nsICategoryManager {
    #[inline]
    pub fn coerce<T: nsICategoryManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICategoryManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICategoryManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICategoryManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICategoryManagerVTable {
    pub __base: nsISupportsVTable,

    /* string getCategoryEntry (in string aCategory, in string aEntry); */
    pub getCategoryEntry: unsafe extern "C" fn (this: *const nsICategoryManager, aCategory: *const libc::c_char, aEntry: *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

    /* string addCategoryEntry (in string aCategory, in string aEntry, in string aValue, in boolean aPersist, in boolean aReplace); */
    pub addCategoryEntry: unsafe extern "C" fn (this: *const nsICategoryManager, aCategory: *const libc::c_char, aEntry: *const libc::c_char, aValue: *const libc::c_char, aPersist: bool, aReplace: bool, _retval: *mut *const libc::c_char) -> nsresult,

    /* void deleteCategoryEntry (in string aCategory, in string aEntry, in boolean aPersist); */
    pub deleteCategoryEntry: unsafe extern "C" fn (this: *const nsICategoryManager, aCategory: *const libc::c_char, aEntry: *const libc::c_char, aPersist: bool) -> nsresult,

    /* void deleteCategory (in string aCategory); */
    pub deleteCategory: unsafe extern "C" fn (this: *const nsICategoryManager, aCategory: *const libc::c_char) -> nsresult,

    /* nsISimpleEnumerator enumerateCategory (in string aCategory); */
    pub enumerateCategory: unsafe extern "C" fn (this: *const nsICategoryManager, aCategory: *const libc::c_char, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsISimpleEnumerator enumerateCategories (); */
    pub enumerateCategories: unsafe extern "C" fn (this: *const nsICategoryManager, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsICategoryManager {
    /* string getCategoryEntry (in string aCategory, in string aEntry); */
    #[inline]
    pub unsafe fn getCategoryEntry(&self, aCategory: *const libc::c_char, aEntry: *const libc::c_char) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getCategoryEntry)(self as *const _, aCategory, aEntry, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* string addCategoryEntry (in string aCategory, in string aEntry, in string aValue, in boolean aPersist, in boolean aReplace); */
    #[inline]
    pub unsafe fn addCategoryEntry(&self, aCategory: *const libc::c_char, aEntry: *const libc::c_char, aValue: *const libc::c_char, aPersist: bool, aReplace: bool) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).addCategoryEntry)(self as *const _, aCategory, aEntry, aValue, aPersist, aReplace, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void deleteCategoryEntry (in string aCategory, in string aEntry, in boolean aPersist); */
    #[inline]
    pub unsafe fn deleteCategoryEntry(&self, aCategory: *const libc::c_char, aEntry: *const libc::c_char, aPersist: bool) -> Result<(), nsresult> {

        match ((*self.vtable).deleteCategoryEntry)(self as *const _, aCategory, aEntry, aPersist) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteCategory (in string aCategory); */
    #[inline]
    pub unsafe fn deleteCategory(&self, aCategory: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).deleteCategory)(self as *const _, aCategory) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator enumerateCategory (in string aCategory); */
    #[inline]
    pub unsafe fn enumerateCategory(&self, aCategory: *const libc::c_char) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerateCategory)(self as *const _, aCategory, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator enumerateCategories (); */
    #[inline]
    pub unsafe fn enumerateCategories(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerateCategories)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


