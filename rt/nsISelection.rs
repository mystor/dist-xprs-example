//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISelection.idl
//


#[repr(C)]
pub struct nsISelection {
    vtable: *const nsISelectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISelection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe0a4d4b3, 0xf34e, 0x44bd,
            [0xb1, 0xf2, 0x4e, 0x3b, 0xde, 0x9b, 0x69, 0x15])
    }
}

unsafe impl RefCounted for nsISelection {
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
pub trait nsISelectionCoerce {
    fn coerce_from(v: &nsISelection) -> &Self;
}

impl nsISelectionCoerce for nsISelection {
    #[inline]
    fn coerce_from(v: &nsISelection) -> &Self {
        v
    }
}

impl nsISelection {
    #[inline]
    pub fn coerce<T: nsISelectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISelection {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISelectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISelection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISelectionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMNode anchorNode; */
    pub get_anchorNode: unsafe extern "C" fn (this: *const nsISelection, aAnchorNode: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute long anchorOffset; */
    pub get_anchorOffset: unsafe extern "C" fn (this: *const nsISelection, aAnchorOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsIDOMNode focusNode; */
    pub get_focusNode: unsafe extern "C" fn (this: *const nsISelection, aFocusNode: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute long focusOffset; */
    pub get_focusOffset: unsafe extern "C" fn (this: *const nsISelection, aFocusOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean isCollapsed; */
    pub get_isCollapsed: unsafe extern "C" fn (this: *const nsISelection, aIsCollapsed: *mut bool) -> nsresult,

    /* [noscript,nostdcall,notxpcom] boolean collapsed (); */
    pub collapsed: unsafe extern "C" fn (this: *const nsISelection) -> bool,

    /* readonly attribute long rangeCount; */
    pub get_rangeCount: unsafe extern "C" fn (this: *const nsISelection, aRangeCount: *mut libc::int32_t) -> nsresult,

    /* nsIDOMRange getRangeAt (in long index); */
    pub getRangeAt: unsafe extern "C" fn (this: *const nsISelection, index: libc::int32_t, _retval: *mut *const nsIDOMRange) -> nsresult,

    /* void collapse (in nsIDOMNode parentNode, in long offset); */
    pub collapse: unsafe extern "C" fn (this: *const nsISelection, parentNode: *const nsIDOMNode, offset: libc::int32_t) -> nsresult,

    /* [noscript] void collapseNative (in nsINode parentNode, in long offset); */
    pub collapseNative: unsafe extern "C" fn (this: *const nsISelection, parentNode: *const nsINode, offset: libc::int32_t) -> nsresult,

    /* void extend (in nsIDOMNode parentNode, in long offset); */
    pub extend: unsafe extern "C" fn (this: *const nsISelection, parentNode: *const nsIDOMNode, offset: libc::int32_t) -> nsresult,

    /* [noscript] void extendNative (in nsINode parentNode, in long offset); */
    pub extendNative: unsafe extern "C" fn (this: *const nsISelection, parentNode: *const nsINode, offset: libc::int32_t) -> nsresult,

    /* void collapseToStart (); */
    pub collapseToStart: unsafe extern "C" fn (this: *const nsISelection) -> nsresult,

    /* void collapseToEnd (); */
    pub collapseToEnd: unsafe extern "C" fn (this: *const nsISelection) -> nsresult,

    /* boolean containsNode (in nsIDOMNode node, in boolean partlyContained); */
    pub containsNode: unsafe extern "C" fn (this: *const nsISelection, node: *const nsIDOMNode, partlyContained: bool, _retval: *mut bool) -> nsresult,

    /* void selectAllChildren (in nsIDOMNode parentNode); */
    pub selectAllChildren: unsafe extern "C" fn (this: *const nsISelection, parentNode: *const nsIDOMNode) -> nsresult,

    /* void addRange (in nsIDOMRange range); */
    pub addRange: unsafe extern "C" fn (this: *const nsISelection, range: *const nsIDOMRange) -> nsresult,

    /* void removeRange (in nsIDOMRange range); */
    pub removeRange: unsafe extern "C" fn (this: *const nsISelection, range: *const nsIDOMRange) -> nsresult,

    /* void removeAllRanges (); */
    pub removeAllRanges: unsafe extern "C" fn (this: *const nsISelection) -> nsresult,

    /* void deleteFromDocument (); */
    pub deleteFromDocument: unsafe extern "C" fn (this: *const nsISelection) -> nsresult,

    /* DOMString toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISelection, _retval: *mut nsAString) -> nsresult,

    /* void modify (in DOMString alter, in DOMString direction, in DOMString granularity); */
    pub modify: unsafe extern "C" fn (this: *const nsISelection, alter: *const nsAString, direction: *const nsAString, granularity: *const nsAString) -> nsresult,

}


impl nsISelection {
    /* readonly attribute nsIDOMNode anchorNode; */
    #[inline]
    pub unsafe fn get_anchorNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_anchorNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long anchorOffset; */
    #[inline]
    pub unsafe fn get_anchorOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_anchorOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMNode focusNode; */
    #[inline]
    pub unsafe fn get_focusNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_focusNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long focusOffset; */
    #[inline]
    pub unsafe fn get_focusOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_focusOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isCollapsed; */
    #[inline]
    pub unsafe fn get_isCollapsed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isCollapsed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,nostdcall,notxpcom] boolean collapsed (); */
    #[inline]
    pub unsafe fn collapsed(&self, ) -> bool {

        let _retval = ((*self.vtable).collapsed)(self as *const _, );
        _retval
    }

    /* readonly attribute long rangeCount; */
    #[inline]
    pub unsafe fn get_rangeCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_rangeCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMRange getRangeAt (in long index); */
    #[inline]
    pub unsafe fn getRangeAt(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIDOMRange>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRangeAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void collapse (in nsIDOMNode parentNode, in long offset); */
    #[inline]
    pub unsafe fn collapse(&self, parentNode: Option<&nsIDOMNode>, offset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).collapse)(self as *const _, parentNode.map_or(::std::ptr::null(), |x| x as *const _), offset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void collapseNative (in nsINode parentNode, in long offset); */
    #[inline]
    pub unsafe fn collapseNative(&self, parentNode: Option<&nsINode>, offset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).collapseNative)(self as *const _, parentNode.map_or(::std::ptr::null(), |x| x as *const _), offset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void extend (in nsIDOMNode parentNode, in long offset); */
    #[inline]
    pub unsafe fn extend(&self, parentNode: Option<&nsIDOMNode>, offset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).extend)(self as *const _, parentNode.map_or(::std::ptr::null(), |x| x as *const _), offset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void extendNative (in nsINode parentNode, in long offset); */
    #[inline]
    pub unsafe fn extendNative(&self, parentNode: Option<&nsINode>, offset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).extendNative)(self as *const _, parentNode.map_or(::std::ptr::null(), |x| x as *const _), offset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void collapseToStart (); */
    #[inline]
    pub unsafe fn collapseToStart(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).collapseToStart)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void collapseToEnd (); */
    #[inline]
    pub unsafe fn collapseToEnd(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).collapseToEnd)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean containsNode (in nsIDOMNode node, in boolean partlyContained); */
    #[inline]
    pub unsafe fn containsNode(&self, node: Option<&nsIDOMNode>, partlyContained: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).containsNode)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _), partlyContained, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void selectAllChildren (in nsIDOMNode parentNode); */
    #[inline]
    pub unsafe fn selectAllChildren(&self, parentNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).selectAllChildren)(self as *const _, parentNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addRange (in nsIDOMRange range); */
    #[inline]
    pub unsafe fn addRange(&self, range: Option<&nsIDOMRange>) -> Result<(), nsresult> {

        match ((*self.vtable).addRange)(self as *const _, range.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeRange (in nsIDOMRange range); */
    #[inline]
    pub unsafe fn removeRange(&self, range: Option<&nsIDOMRange>) -> Result<(), nsresult> {

        match ((*self.vtable).removeRange)(self as *const _, range.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAllRanges (); */
    #[inline]
    pub unsafe fn removeAllRanges(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).removeAllRanges)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteFromDocument (); */
    #[inline]
    pub unsafe fn deleteFromDocument(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).deleteFromDocument)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMString toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).toString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void modify (in DOMString alter, in DOMString direction, in DOMString granularity); */
    #[inline]
    pub unsafe fn modify(&self, alter: &[u16], direction: &[u16], granularity: &[u16]) -> Result<(), nsresult> {
        let alter = nsString::from(alter);
        let direction = nsString::from(direction);
        let granularity = nsString::from(granularity);
        match ((*self.vtable).modify)(self as *const _, &*alter, &*direction, &*granularity) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


