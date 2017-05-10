//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMMozNamedAttrMap.idl
//


#[repr(C)]
pub struct nsIDOMMozNamedAttrMap {
    vtable: *const nsIDOMMozNamedAttrMapVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMMozNamedAttrMap {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcb5564cd, 0x26ec, 0x418f,
            [0xa6, 0xd6, 0x1d, 0x57, 0xcd, 0x2c, 0x97, 0x1c])
    }
}

unsafe impl RefCounted for nsIDOMMozNamedAttrMap {
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
pub trait nsIDOMMozNamedAttrMapCoerce {
    fn coerce_from(v: &nsIDOMMozNamedAttrMap) -> &Self;
}

impl nsIDOMMozNamedAttrMapCoerce for nsIDOMMozNamedAttrMap {
    #[inline]
    fn coerce_from(v: &nsIDOMMozNamedAttrMap) -> &Self {
        v
    }
}

impl nsIDOMMozNamedAttrMap {
    #[inline]
    pub fn coerce<T: nsIDOMMozNamedAttrMapCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMMozNamedAttrMap {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMMozNamedAttrMapCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMMozNamedAttrMap) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMMozNamedAttrMapVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMAttr getNamedItem (in DOMString name); */
    pub getNamedItem: unsafe extern "C" fn (this: *const nsIDOMMozNamedAttrMap, name: *const nsAString, _retval: *mut *const nsIDOMAttr) -> nsresult,

    /* nsIDOMAttr setNamedItem (in nsIDOMAttr arg) raises (DOMException); */
    pub setNamedItem: unsafe extern "C" fn (this: *const nsIDOMMozNamedAttrMap, arg: *const nsIDOMAttr, _retval: *mut *const nsIDOMAttr) -> nsresult,

    /* nsIDOMAttr removeNamedItem (in DOMString name) raises (DOMException); */
    pub removeNamedItem: unsafe extern "C" fn (this: *const nsIDOMMozNamedAttrMap, name: *const nsAString, _retval: *mut *const nsIDOMAttr) -> nsresult,

    /* nsIDOMAttr item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMMozNamedAttrMap, index: libc::uint32_t, _retval: *mut *const nsIDOMAttr) -> nsresult,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMMozNamedAttrMap, aLength: *mut libc::uint32_t) -> nsresult,

    /* nsIDOMAttr getNamedItemNS (in DOMString namespaceURI, in DOMString localName); */
    pub getNamedItemNS: unsafe extern "C" fn (this: *const nsIDOMMozNamedAttrMap, namespaceURI: *const nsAString, localName: *const nsAString, _retval: *mut *const nsIDOMAttr) -> nsresult,

    /* nsIDOMAttr setNamedItemNS (in nsIDOMAttr arg) raises (DOMException); */
    pub setNamedItemNS: unsafe extern "C" fn (this: *const nsIDOMMozNamedAttrMap, arg: *const nsIDOMAttr, _retval: *mut *const nsIDOMAttr) -> nsresult,

    /* nsIDOMAttr removeNamedItemNS (in DOMString namespaceURI, in DOMString localName) raises (DOMException); */
    pub removeNamedItemNS: unsafe extern "C" fn (this: *const nsIDOMMozNamedAttrMap, namespaceURI: *const nsAString, localName: *const nsAString, _retval: *mut *const nsIDOMAttr) -> nsresult,

}


impl nsIDOMMozNamedAttrMap {
    /* nsIDOMAttr getNamedItem (in DOMString name); */
    #[inline]
    pub unsafe fn getNamedItem(&self, name: &[u16]) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNamedItem)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMAttr setNamedItem (in nsIDOMAttr arg) raises (DOMException); */
    #[inline]
    pub unsafe fn setNamedItem(&self, arg: Option<&nsIDOMAttr>) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).setNamedItem)(self as *const _, arg.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMAttr removeNamedItem (in DOMString name) raises (DOMException); */
    #[inline]
    pub unsafe fn removeNamedItem(&self, name: &[u16]) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).removeNamedItem)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMAttr item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).item)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMAttr getNamedItemNS (in DOMString namespaceURI, in DOMString localName); */
    #[inline]
    pub unsafe fn getNamedItemNS(&self, namespaceURI: &[u16], localName: &[u16]) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let localName = nsString::from(localName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNamedItemNS)(self as *const _, &*namespaceURI, &*localName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMAttr setNamedItemNS (in nsIDOMAttr arg) raises (DOMException); */
    #[inline]
    pub unsafe fn setNamedItemNS(&self, arg: Option<&nsIDOMAttr>) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).setNamedItemNS)(self as *const _, arg.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMAttr removeNamedItemNS (in DOMString namespaceURI, in DOMString localName) raises (DOMException); */
    #[inline]
    pub unsafe fn removeNamedItemNS(&self, namespaceURI: &[u16], localName: &[u16]) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let localName = nsString::from(localName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).removeNamedItemNS)(self as *const _, &*namespaceURI, &*localName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


