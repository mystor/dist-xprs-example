//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/inIDOMView.idl
//


#[repr(C)]
pub struct inIDOMView {
    vtable: *const inIDOMViewVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for inIDOMView {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfbb67442, 0x27a3, 0x483c,
            [0x8e, 0xb2, 0x29, 0xc3, 0xee, 0xd7, 0x51, 0x4c])
    }
}

unsafe impl RefCounted for inIDOMView {
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
pub trait inIDOMViewCoerce {
    fn coerce_from(v: &inIDOMView) -> &Self;
}

impl inIDOMViewCoerce for inIDOMView {
    #[inline]
    fn coerce_from(v: &inIDOMView) -> &Self {
        v
    }
}

impl inIDOMView {
    #[inline]
    pub fn coerce<T: inIDOMViewCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for inIDOMView {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> inIDOMViewCoerce for T {
    #[inline]
    fn coerce_from(v: &inIDOMView) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct inIDOMViewVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIDOMNode rootNode; */
    pub get_rootNode: unsafe extern "C" fn (this: *const inIDOMView, aRootNode: *mut *const nsIDOMNode) -> nsresult,
    pub set_rootNode: unsafe extern "C" fn (this: *const inIDOMView, aRootNode: *const nsIDOMNode) -> nsresult,

    /* attribute boolean showAnonymousContent; */
    pub get_showAnonymousContent: unsafe extern "C" fn (this: *const inIDOMView, aShowAnonymousContent: *mut bool) -> nsresult,
    pub set_showAnonymousContent: unsafe extern "C" fn (this: *const inIDOMView, aShowAnonymousContent: bool) -> nsresult,

    /* attribute boolean showSubDocuments; */
    pub get_showSubDocuments: unsafe extern "C" fn (this: *const inIDOMView, aShowSubDocuments: *mut bool) -> nsresult,
    pub set_showSubDocuments: unsafe extern "C" fn (this: *const inIDOMView, aShowSubDocuments: bool) -> nsresult,

    /* attribute boolean showWhitespaceNodes; */
    pub get_showWhitespaceNodes: unsafe extern "C" fn (this: *const inIDOMView, aShowWhitespaceNodes: *mut bool) -> nsresult,
    pub set_showWhitespaceNodes: unsafe extern "C" fn (this: *const inIDOMView, aShowWhitespaceNodes: bool) -> nsresult,

    /* attribute boolean showAccessibleNodes; */
    pub get_showAccessibleNodes: unsafe extern "C" fn (this: *const inIDOMView, aShowAccessibleNodes: *mut bool) -> nsresult,
    pub set_showAccessibleNodes: unsafe extern "C" fn (this: *const inIDOMView, aShowAccessibleNodes: bool) -> nsresult,

    /* attribute unsigned long whatToShow; */
    pub get_whatToShow: unsafe extern "C" fn (this: *const inIDOMView, aWhatToShow: *mut libc::uint32_t) -> nsresult,
    pub set_whatToShow: unsafe extern "C" fn (this: *const inIDOMView, aWhatToShow: libc::uint32_t) -> nsresult,

    /* nsIDOMNode getNodeFromRowIndex (in long rowIndex); */
    pub getNodeFromRowIndex: unsafe extern "C" fn (this: *const inIDOMView, rowIndex: libc::int32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* long getRowIndexFromNode (in nsIDOMNode node); */
    pub getRowIndexFromNode: unsafe extern "C" fn (this: *const inIDOMView, node: *const nsIDOMNode, _retval: *mut libc::int32_t) -> nsresult,

    /* void rebuild (); */
    pub rebuild: unsafe extern "C" fn (this: *const inIDOMView) -> nsresult,

}


impl inIDOMView {
    /* attribute nsIDOMNode rootNode; */
    #[inline]
    pub unsafe fn get_rootNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rootNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_rootNode(&self, aRootNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).set_rootNode)(self as *const _, aRootNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* attribute boolean showWhitespaceNodes; */
    #[inline]
    pub unsafe fn get_showWhitespaceNodes(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_showWhitespaceNodes)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_showWhitespaceNodes(&self, aShowWhitespaceNodes: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_showWhitespaceNodes)(self as *const _, aShowWhitespaceNodes) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean showAccessibleNodes; */
    #[inline]
    pub unsafe fn get_showAccessibleNodes(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_showAccessibleNodes)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_showAccessibleNodes(&self, aShowAccessibleNodes: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_showAccessibleNodes)(self as *const _, aShowAccessibleNodes) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long whatToShow; */
    #[inline]
    pub unsafe fn get_whatToShow(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_whatToShow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_whatToShow(&self, aWhatToShow: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_whatToShow)(self as *const _, aWhatToShow) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMNode getNodeFromRowIndex (in long rowIndex); */
    #[inline]
    pub unsafe fn getNodeFromRowIndex(&self, rowIndex: libc::int32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNodeFromRowIndex)(self as *const _, rowIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* long getRowIndexFromNode (in nsIDOMNode node); */
    #[inline]
    pub unsafe fn getRowIndexFromNode(&self, node: Option<&nsIDOMNode>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRowIndexFromNode)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void rebuild (); */
    #[inline]
    pub unsafe fn rebuild(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).rebuild)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


