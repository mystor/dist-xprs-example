//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/inIDeepTreeWalker.idl
//


#[repr(C)]
pub struct inIDeepTreeWalker {
    vtable: *const inIDeepTreeWalkerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for inIDeepTreeWalker {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6657e8eb, 0xb646, 0x48e7,
            [0x99, 0x3e, 0xcf, 0xa6, 0xe9, 0x64, 0x15, 0xb4])
    }
}

unsafe impl RefCounted for inIDeepTreeWalker {
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
pub trait inIDeepTreeWalkerCoerce {
    fn coerce_from(v: &inIDeepTreeWalker) -> &Self;
}

impl inIDeepTreeWalkerCoerce for inIDeepTreeWalker {
    #[inline]
    fn coerce_from(v: &inIDeepTreeWalker) -> &Self {
        v
    }
}

impl inIDeepTreeWalker {
    #[inline]
    pub fn coerce<T: inIDeepTreeWalkerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for inIDeepTreeWalker {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> inIDeepTreeWalkerCoerce for T {
    #[inline]
    fn coerce_from(v: &inIDeepTreeWalker) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct inIDeepTreeWalkerVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean showAnonymousContent; */
    pub get_showAnonymousContent: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aShowAnonymousContent: *mut bool) -> nsresult,
    pub set_showAnonymousContent: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aShowAnonymousContent: bool) -> nsresult,

    /* attribute boolean showSubDocuments; */
    pub get_showSubDocuments: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aShowSubDocuments: *mut bool) -> nsresult,
    pub set_showSubDocuments: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aShowSubDocuments: bool) -> nsresult,

    /* attribute boolean showDocumentsAsNodes; */
    pub get_showDocumentsAsNodes: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aShowDocumentsAsNodes: *mut bool) -> nsresult,
    pub set_showDocumentsAsNodes: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aShowDocumentsAsNodes: bool) -> nsresult,

    /* void init (in nsIDOMNode aRoot, in unsigned long aWhatToShow); */
    pub init: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aRoot: *const nsIDOMNode, aWhatToShow: libc::uint32_t) -> nsresult,

    /* readonly attribute nsIDOMNode root; */
    pub get_root: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aRoot: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute unsigned long whatToShow; */
    pub get_whatToShow: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aWhatToShow: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIDOMNodeFilter filter; */
    pub get_filter: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aFilter: *mut *const nsIDOMNodeFilter) -> nsresult,

    /* attribute nsIDOMNode currentNode; */
    pub get_currentNode: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aCurrentNode: *mut *const nsIDOMNode) -> nsresult,
    pub set_currentNode: unsafe extern "C" fn (this: *const inIDeepTreeWalker, aCurrentNode: *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode parentNode (); */
    pub parentNode: unsafe extern "C" fn (this: *const inIDeepTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode firstChild (); */
    pub firstChild: unsafe extern "C" fn (this: *const inIDeepTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode lastChild (); */
    pub lastChild: unsafe extern "C" fn (this: *const inIDeepTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode previousSibling (); */
    pub previousSibling: unsafe extern "C" fn (this: *const inIDeepTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode nextSibling (); */
    pub nextSibling: unsafe extern "C" fn (this: *const inIDeepTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode previousNode (); */
    pub previousNode: unsafe extern "C" fn (this: *const inIDeepTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode nextNode (); */
    pub nextNode: unsafe extern "C" fn (this: *const inIDeepTreeWalker, _retval: *mut *const nsIDOMNode) -> nsresult,

}


impl inIDeepTreeWalker {
    /* attribute boolean showAnonymousContent; */
    #[inline]
    pub unsafe fn get_showAnonymousContent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_showAnonymousContent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_showAnonymousContent(&self, aShowAnonymousContent: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_showAnonymousContent)(self as *const _, aShowAnonymousContent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean showSubDocuments; */
    #[inline]
    pub unsafe fn get_showSubDocuments(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_showSubDocuments)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_showSubDocuments(&self, aShowSubDocuments: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_showSubDocuments)(self as *const _, aShowSubDocuments) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean showDocumentsAsNodes; */
    #[inline]
    pub unsafe fn get_showDocumentsAsNodes(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_showDocumentsAsNodes)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_showDocumentsAsNodes(&self, aShowDocumentsAsNodes: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_showDocumentsAsNodes)(self as *const _, aShowDocumentsAsNodes) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void init (in nsIDOMNode aRoot, in unsigned long aWhatToShow); */
    #[inline]
    pub unsafe fn init(&self, aRoot: Option<&nsIDOMNode>, aWhatToShow: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aRoot.map_or(::std::ptr::null(), |x| x as *const _), aWhatToShow) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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


