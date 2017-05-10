//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULStore.idl
//


#[repr(C)]
pub struct nsIXULStore {
    vtable: *const nsIXULStoreVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULStore {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x987c4b35, 0xc426, 0x4dd7,
            [0xad, 0x49, 0x3c, 0x9f, 0xa4, 0xc6, 0x5d, 0x20])
    }
}

unsafe impl RefCounted for nsIXULStore {
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
pub trait nsIXULStoreCoerce {
    fn coerce_from(v: &nsIXULStore) -> &Self;
}

impl nsIXULStoreCoerce for nsIXULStore {
    #[inline]
    fn coerce_from(v: &nsIXULStore) -> &Self {
        v
    }
}

impl nsIXULStore {
    #[inline]
    pub fn coerce<T: nsIXULStoreCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULStore {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULStoreCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULStore) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULStoreVTable {
    pub __base: nsISupportsVTable,

    /* void setValue (in AString doc, in AString id, in AString attr, in AString value); */
    pub setValue: unsafe extern "C" fn (this: *const nsIXULStore, doc: *const nsAString, id: *const nsAString, attr: *const nsAString, value: *const nsAString) -> nsresult,

    /* bool hasValue (in AString doc, in AString id, in AString attr); */
    pub hasValue: unsafe extern "C" fn (this: *const nsIXULStore, doc: *const nsAString, id: *const nsAString, attr: *const nsAString, _retval: *mut bool) -> nsresult,

    /* AString getValue (in AString doc, in AString id, in AString attr); */
    pub getValue: unsafe extern "C" fn (this: *const nsIXULStore, doc: *const nsAString, id: *const nsAString, attr: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* void removeValue (in AString doc, in AString id, in AString attr); */
    pub removeValue: unsafe extern "C" fn (this: *const nsIXULStore, doc: *const nsAString, id: *const nsAString, attr: *const nsAString) -> nsresult,

    /* nsIStringEnumerator getIDsEnumerator (in AString doc); */
    pub getIDsEnumerator: unsafe extern "C" fn (this: *const nsIXULStore, doc: *const nsAString, _retval: *mut *const nsIStringEnumerator) -> nsresult,

    /* nsIStringEnumerator getAttributeEnumerator (in AString doc, in AString id); */
    pub getAttributeEnumerator: unsafe extern "C" fn (this: *const nsIXULStore, doc: *const nsAString, id: *const nsAString, _retval: *mut *const nsIStringEnumerator) -> nsresult,

}


impl nsIXULStore {
    /* void setValue (in AString doc, in AString id, in AString attr, in AString value); */
    #[inline]
    pub unsafe fn setValue(&self, doc: &[u16], id: &[u16], attr: &[u16], value: &[u16]) -> Result<(), nsresult> {
        let doc = nsString::from(doc);
        let id = nsString::from(id);
        let attr = nsString::from(attr);
        let value = nsString::from(value);
        match ((*self.vtable).setValue)(self as *const _, &*doc, &*id, &*attr, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool hasValue (in AString doc, in AString id, in AString attr); */
    #[inline]
    pub unsafe fn hasValue(&self, doc: &[u16], id: &[u16], attr: &[u16]) -> Result<bool, nsresult> {
        let doc = nsString::from(doc);
        let id = nsString::from(id);
        let attr = nsString::from(attr);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasValue)(self as *const _, &*doc, &*id, &*attr, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getValue (in AString doc, in AString id, in AString attr); */
    #[inline]
    pub unsafe fn getValue(&self, doc: &[u16], id: &[u16], attr: &[u16]) -> Result<nsString, nsresult> {
        let doc = nsString::from(doc);
        let id = nsString::from(id);
        let attr = nsString::from(attr);
        let mut _retval = nsString::new();
        match ((*self.vtable).getValue)(self as *const _, &*doc, &*id, &*attr, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void removeValue (in AString doc, in AString id, in AString attr); */
    #[inline]
    pub unsafe fn removeValue(&self, doc: &[u16], id: &[u16], attr: &[u16]) -> Result<(), nsresult> {
        let doc = nsString::from(doc);
        let id = nsString::from(id);
        let attr = nsString::from(attr);
        match ((*self.vtable).removeValue)(self as *const _, &*doc, &*id, &*attr) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIStringEnumerator getIDsEnumerator (in AString doc); */
    #[inline]
    pub unsafe fn getIDsEnumerator(&self, doc: &[u16]) -> Result<Option<RefPtr<nsIStringEnumerator>>, nsresult> {
        let doc = nsString::from(doc);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getIDsEnumerator)(self as *const _, &*doc, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIStringEnumerator getAttributeEnumerator (in AString doc, in AString id); */
    #[inline]
    pub unsafe fn getAttributeEnumerator(&self, doc: &[u16], id: &[u16]) -> Result<Option<RefPtr<nsIStringEnumerator>>, nsresult> {
        let doc = nsString::from(doc);
        let id = nsString::from(id);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAttributeEnumerator)(self as *const _, &*doc, &*id, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


