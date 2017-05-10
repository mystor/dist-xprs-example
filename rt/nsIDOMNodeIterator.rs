//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNodeIterator.idl
//


#[repr(C)]
pub struct nsIDOMNodeIterator {
    vtable: *const nsIDOMNodeIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMNodeIterator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa86bdac9, 0xff89, 0x4c94,
            [0x81, 0x60, 0x4f, 0xe8, 0x67, 0x33, 0xba, 0xb8])
    }
}

unsafe impl RefCounted for nsIDOMNodeIterator {
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
pub trait nsIDOMNodeIteratorCoerce {
    fn coerce_from(v: &nsIDOMNodeIterator) -> &Self;
}

impl nsIDOMNodeIteratorCoerce for nsIDOMNodeIterator {
    #[inline]
    fn coerce_from(v: &nsIDOMNodeIterator) -> &Self {
        v
    }
}

impl nsIDOMNodeIterator {
    #[inline]
    pub fn coerce<T: nsIDOMNodeIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMNodeIterator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMNodeIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMNodeIterator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMNodeIteratorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMNode root; */
    pub get_root: unsafe extern "C" fn (this: *const nsIDOMNodeIterator, aRoot: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute unsigned long whatToShow; */
    pub get_whatToShow: unsafe extern "C" fn (this: *const nsIDOMNodeIterator, aWhatToShow: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIDOMNodeFilter filter; */
    pub get_filter: unsafe extern "C" fn (this: *const nsIDOMNodeIterator, aFilter: *mut *const nsIDOMNodeFilter) -> nsresult,

    /* nsIDOMNode nextNode () raises (DOMException); */
    pub nextNode: unsafe extern "C" fn (this: *const nsIDOMNodeIterator, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode previousNode () raises (DOMException); */
    pub previousNode: unsafe extern "C" fn (this: *const nsIDOMNodeIterator, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* void detach (); */
    pub detach: unsafe extern "C" fn (this: *const nsIDOMNodeIterator) -> nsresult,

    /* readonly attribute nsIDOMNode referenceNode; */
    pub get_referenceNode: unsafe extern "C" fn (this: *const nsIDOMNodeIterator, aReferenceNode: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute boolean pointerBeforeReferenceNode; */
    pub get_pointerBeforeReferenceNode: unsafe extern "C" fn (this: *const nsIDOMNodeIterator, aPointerBeforeReferenceNode: *mut bool) -> nsresult,

}


impl nsIDOMNodeIterator {
    /* readonly attribute nsIDOMNode root; */
    #[inline]
    pub unsafe fn get_root(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_root)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long whatToShow; */
    #[inline]
    pub unsafe fn get_whatToShow(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_whatToShow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMNodeFilter filter; */
    #[inline]
    pub unsafe fn get_filter(&self, ) -> Result<Option<RefPtr<nsIDOMNodeFilter>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_filter)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode nextNode () raises (DOMException); */
    #[inline]
    pub unsafe fn nextNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).nextNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode previousNode () raises (DOMException); */
    #[inline]
    pub unsafe fn previousNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).previousNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void detach (); */
    #[inline]
    pub unsafe fn detach(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).detach)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMNode referenceNode; */
    #[inline]
    pub unsafe fn get_referenceNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_referenceNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean pointerBeforeReferenceNode; */
    #[inline]
    pub unsafe fn get_pointerBeforeReferenceNode(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_pointerBeforeReferenceNode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


