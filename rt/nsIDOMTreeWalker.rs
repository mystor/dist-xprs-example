//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMTreeWalker.idl
//


#[repr(C)]
pub struct nsIDOMTreeWalker {
    vtable: *const nsIDOMTreeWalkerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMTreeWalker {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc4ffa159, 0x237c, 0x4dde,
            [0xb0, 0xba, 0x20, 0xb9, 0xf9, 0x27, 0x0c, 0xf6])
    }
}

unsafe impl RefCounted for nsIDOMTreeWalker {
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
pub trait nsIDOMTreeWalkerCoerce {
    fn coerce_from(v: &nsIDOMTreeWalker) -> &Self;
}

impl nsIDOMTreeWalkerCoerce for nsIDOMTreeWalker {
    #[inline]
    fn coerce_from(v: &nsIDOMTreeWalker) -> &Self {
        v
    }
}

impl nsIDOMTreeWalker {
    #[inline]
    pub fn coerce<T: nsIDOMTreeWalkerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMTreeWalker {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMTreeWalkerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMTreeWalker) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMTreeWalkerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMNode root; */
    pub get_root: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, aRoot: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute unsigned long whatToShow; */
    pub get_whatToShow: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, aWhatToShow: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIDOMNodeFilter filter; */
    pub get_filter: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, aFilter: *mut *const nsIDOMNodeFilter) -> nsresult,

    /* attribute nsIDOMNode currentNode; */
    pub get_currentNode: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, aCurrentNode: *mut *const nsIDOMNode) -> nsresult,
    pub set_currentNode: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, aCurrentNode: *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode parentNode (); */
    pub parentNode: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode firstChild (); */
    pub firstChild: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode lastChild (); */
    pub lastChild: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode previousSibling (); */
    pub previousSibling: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode nextSibling (); */
    pub nextSibling: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode previousNode (); */
    pub previousNode: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode nextNode (); */
    pub nextNode: unsafe extern "C" fn (this: *const nsIDOMTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

}


impl nsIDOMTreeWalker {
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

    /* attribute nsIDOMNode currentNode; */
    #[inline]
    pub unsafe fn get_currentNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_currentNode(&self, aCurrentNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).set_currentNode)(self as *const _, aCurrentNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMNode parentNode (); */
    #[inline]
    pub unsafe fn parentNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).parentNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode firstChild (); */
    #[inline]
    pub unsafe fn firstChild(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).firstChild)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode lastChild (); */
    #[inline]
    pub unsafe fn lastChild(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).lastChild)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode previousSibling (); */
    #[inline]
    pub unsafe fn previousSibling(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).previousSibling)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode nextSibling (); */
    #[inline]
    pub unsafe fn nextSibling(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).nextSibling)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode previousNode (); */
    #[inline]
    pub unsafe fn previousNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).previousNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode nextNode (); */
    #[inline]
    pub unsafe fn nextNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).nextNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


