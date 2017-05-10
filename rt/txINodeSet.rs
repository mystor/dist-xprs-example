//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/txINodeSet.idl
//


#[repr(C)]
pub struct txINodeSet {
    vtable: *const txINodeSetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for txINodeSet {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x15d424c0, 0x6b47, 0x11d9,
            [0x97, 0x91, 0x00, 0x0a, 0x95, 0xdc, 0x23, 0x4c])
    }
}

unsafe impl RefCounted for txINodeSet {
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
pub trait txINodeSetCoerce {
    fn coerce_from(v: &txINodeSet) -> &Self;
}

impl txINodeSetCoerce for txINodeSet {
    #[inline]
    fn coerce_from(v: &txINodeSet) -> &Self {
        v
    }
}

impl txINodeSet {
    #[inline]
    pub fn coerce<T: txINodeSetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for txINodeSet {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> txINodeSetCoerce for T {
    #[inline]
    fn coerce_from(v: &txINodeSet) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct txINodeSetVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMNode item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const txINodeSet, index: libc::uint32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* double itemAsNumber (in unsigned long index); */
    pub itemAsNumber: unsafe extern "C" fn (this: *const txINodeSet, index: libc::uint32_t, _retval: *mut libc::c_double) -> nsresult,

    /* DOMString itemAsString (in unsigned long index); */
    pub itemAsString: unsafe extern "C" fn (this: *const txINodeSet, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const txINodeSet, aLength: *mut libc::uint32_t) -> nsresult,

    /* void add (in nsIDOMNode node); */
    pub add: unsafe extern "C" fn (this: *const txINodeSet, node: *const nsIDOMNode) -> nsresult,

}


impl txINodeSet {
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

    /* double itemAsNumber (in unsigned long index); */
    #[inline]
    pub unsafe fn itemAsNumber(&self, index: libc::uint32_t) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).itemAsNumber)(self as *const _, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* DOMString itemAsString (in unsigned long index); */
    #[inline]
    pub unsafe fn itemAsString(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).itemAsString)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* void add (in nsIDOMNode node); */
    #[inline]
    pub unsafe fn add(&self, node: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).add)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


