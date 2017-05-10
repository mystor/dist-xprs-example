//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLOptionsCollection.idl
//


#[repr(C)]
pub struct nsIDOMHTMLOptionsCollection {
    vtable: *const nsIDOMHTMLOptionsCollectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLOptionsCollection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4173cc53, 0x30f6, 0x4d12,
            [0xa7, 0x70, 0x98, 0x1b, 0xa5, 0x31, 0x64, 0xe2])
    }
}

unsafe impl RefCounted for nsIDOMHTMLOptionsCollection {
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
pub trait nsIDOMHTMLOptionsCollectionCoerce {
    fn coerce_from(v: &nsIDOMHTMLOptionsCollection) -> &Self;
}

impl nsIDOMHTMLOptionsCollectionCoerce for nsIDOMHTMLOptionsCollection {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLOptionsCollection) -> &Self {
        v
    }
}

impl nsIDOMHTMLOptionsCollection {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLOptionsCollectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLOptionsCollection {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLOptionsCollectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLOptionsCollection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLOptionsCollectionVTable {
    pub __base: nsISupportsVTable,

    /* attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionsCollection, aLength: *mut libc::uint32_t) -> nsresult,
    pub set_length: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionsCollection, aLength: libc::uint32_t) -> nsresult,

    /* nsIDOMNode item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionsCollection, index: libc::uint32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode namedItem (in DOMString name); */
    pub namedItem: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionsCollection, name: *const nsAString, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* attribute long selectedIndex; */
    pub get_selectedIndex: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionsCollection, aSelectedIndex: *mut libc::int32_t) -> nsresult,
    pub set_selectedIndex: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionsCollection, aSelectedIndex: libc::int32_t) -> nsresult,

    /* [noscript] readonly attribute nsIDOMHTMLSelectElement select; */
    pub get_select: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionsCollection, aSelect: *mut *const nsIDOMHTMLSelectElement) -> nsresult,

    /* void add (in nsIDOMHTMLOptionElement option, [optional] in nsIVariant before); */
    pub add: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionsCollection, option: *const nsIDOMHTMLOptionElement, before: *const nsIVariant) -> nsresult,

    /* void remove (in long index); */
    pub remove: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionsCollection, index: libc::int32_t) -> nsresult,

}


impl nsIDOMHTMLOptionsCollection {
    /* attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_length(&self, aLength: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_length)(self as *const _, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMNode item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).item)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode namedItem (in DOMString name); */
    #[inline]
    pub unsafe fn namedItem(&self, name: &[u16]) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).namedItem)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute long selectedIndex; */
    #[inline]
    pub unsafe fn get_selectedIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectedIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selectedIndex(&self, aSelectedIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_selectedIndex)(self as *const _, aSelectedIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute nsIDOMHTMLSelectElement select; */
    #[inline]
    pub unsafe fn get_select(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLSelectElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_select)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void add (in nsIDOMHTMLOptionElement option, [optional] in nsIVariant before); */
    #[inline]
    pub unsafe fn add(&self, option: Option<&nsIDOMHTMLOptionElement>, before: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).add)(self as *const _, option.map_or(::std::ptr::null(), |x| x as *const _), before.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void remove (in long index); */
    #[inline]
    pub unsafe fn remove(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).remove)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


