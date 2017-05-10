//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMRange.idl
//


pub mod nsIDOMRange_consts {
    pub const START_TO_START: i64 = 0;
    pub const START_TO_END: i64 = 1;
    pub const END_TO_END: i64 = 2;
    pub const END_TO_START: i64 = 3;
}


#[repr(C)]
pub struct nsIDOMRange {
    vtable: *const nsIDOMRangeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMRange {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1f94055c, 0x42e7, 0x4a30,
            [0x96, 0xa1, 0x6a, 0x80, 0x4f, 0x1c, 0x2d, 0x1e])
    }
}

unsafe impl RefCounted for nsIDOMRange {
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
pub trait nsIDOMRangeCoerce {
    fn coerce_from(v: &nsIDOMRange) -> &Self;
}

impl nsIDOMRangeCoerce for nsIDOMRange {
    #[inline]
    fn coerce_from(v: &nsIDOMRange) -> &Self {
        v
    }
}

impl nsIDOMRange {
    #[inline]
    pub fn coerce<T: nsIDOMRangeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMRange {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMRangeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMRange) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMRangeVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMNode startContainer; */
    pub get_startContainer: unsafe extern "C" fn (this: *const nsIDOMRange, aStartContainer: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute long startOffset; */
    pub get_startOffset: unsafe extern "C" fn (this: *const nsIDOMRange, aStartOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsIDOMNode endContainer; */
    pub get_endContainer: unsafe extern "C" fn (this: *const nsIDOMRange, aEndContainer: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute long endOffset; */
    pub get_endOffset: unsafe extern "C" fn (this: *const nsIDOMRange, aEndOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean collapsed; */
    pub get_collapsed: unsafe extern "C" fn (this: *const nsIDOMRange, aCollapsed: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMNode commonAncestorContainer; */
    pub get_commonAncestorContainer: unsafe extern "C" fn (this: *const nsIDOMRange, aCommonAncestorContainer: *mut *const nsIDOMNode) -> nsresult,

    /* void setStart (in nsIDOMNode refNode, in long offset); */
    pub setStart: unsafe extern "C" fn (this: *const nsIDOMRange, refNode: *const nsIDOMNode, offset: libc::int32_t) -> nsresult,

    /* void setEnd (in nsIDOMNode refNode, in long offset); */
    pub setEnd: unsafe extern "C" fn (this: *const nsIDOMRange, refNode: *const nsIDOMNode, offset: libc::int32_t) -> nsresult,

    /* void setStartBefore (in nsIDOMNode refNode); */
    pub setStartBefore: unsafe extern "C" fn (this: *const nsIDOMRange, refNode: *const nsIDOMNode) -> nsresult,

    /* void setStartAfter (in nsIDOMNode refNode); */
    pub setStartAfter: unsafe extern "C" fn (this: *const nsIDOMRange, refNode: *const nsIDOMNode) -> nsresult,

    /* void setEndBefore (in nsIDOMNode refNode); */
    pub setEndBefore: unsafe extern "C" fn (this: *const nsIDOMRange, refNode: *const nsIDOMNode) -> nsresult,

    /* void setEndAfter (in nsIDOMNode refNode); */
    pub setEndAfter: unsafe extern "C" fn (this: *const nsIDOMRange, refNode: *const nsIDOMNode) -> nsresult,

    /* void collapse (in boolean toStart); */
    pub collapse: unsafe extern "C" fn (this: *const nsIDOMRange, toStart: bool) -> nsresult,

    /* void selectNode (in nsIDOMNode refNode); */
    pub selectNode: unsafe extern "C" fn (this: *const nsIDOMRange, refNode: *const nsIDOMNode) -> nsresult,

    /* void selectNodeContents (in nsIDOMNode refNode); */
    pub selectNodeContents: unsafe extern "C" fn (this: *const nsIDOMRange, refNode: *const nsIDOMNode) -> nsresult,

    /* short compareBoundaryPoints (in unsigned short how, in nsIDOMRange sourceRange); */
    pub compareBoundaryPoints: unsafe extern "C" fn (this: *const nsIDOMRange, how: libc::uint16_t, sourceRange: *const nsIDOMRange, _retval: *mut libc::int16_t) -> nsresult,

    /* void deleteContents (); */
    pub deleteContents: unsafe extern "C" fn (this: *const nsIDOMRange) -> nsresult,

    /* nsIDOMDocumentFragment extractContents (); */
    pub extractContents: unsafe extern "C" fn (this: *const nsIDOMRange, _retval: *mut *const nsIDOMDocumentFragment) -> nsresult,

    /* nsIDOMDocumentFragment cloneContents (); */
    pub cloneContents: unsafe extern "C" fn (this: *const nsIDOMRange, _retval: *mut *const nsIDOMDocumentFragment) -> nsresult,

    /* void insertNode (in nsIDOMNode newNode); */
    pub insertNode: unsafe extern "C" fn (this: *const nsIDOMRange, newNode: *const nsIDOMNode) -> nsresult,

    /* void surroundContents (in nsIDOMNode newParent); */
    pub surroundContents: unsafe extern "C" fn (this: *const nsIDOMRange, newParent: *const nsIDOMNode) -> nsresult,

    /* nsIDOMRange cloneRange (); */
    pub cloneRange: unsafe extern "C" fn (this: *const nsIDOMRange, _retval: *mut *const nsIDOMRange) -> nsresult,

    /* DOMString toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsIDOMRange, _retval: *mut nsAString) -> nsresult,

    /* void detach (); */
    pub detach: unsafe extern "C" fn (this: *const nsIDOMRange) -> nsresult,

    /* nsIDOMDocumentFragment createContextualFragment (in DOMString fragment); */
    pub createContextualFragment: unsafe extern "C" fn (this: *const nsIDOMRange, fragment: *const nsAString, _retval: *mut *const nsIDOMDocumentFragment) -> nsresult,

    /* boolean isPointInRange (in nsIDOMNode parent, in long offset); */
    pub isPointInRange: unsafe extern "C" fn (this: *const nsIDOMRange, parent: *const nsIDOMNode, offset: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* short comparePoint (in nsIDOMNode parent, in long offset); */
    pub comparePoint: unsafe extern "C" fn (this: *const nsIDOMRange, parent: *const nsIDOMNode, offset: libc::int32_t, _retval: *mut libc::int16_t) -> nsresult,

    /* boolean intersectsNode (in nsIDOMNode node); */
    pub intersectsNode: unsafe extern "C" fn (this: *const nsIDOMRange, node: *const nsIDOMNode, _retval: *mut bool) -> nsresult,

    /* nsIDOMClientRectList getClientRects (); */
    pub getClientRects: unsafe extern "C" fn (this: *const nsIDOMRange, _retval: *mut *const nsIDOMClientRectList) -> nsresult,

    /* nsIDOMClientRect getBoundingClientRect (); */
    pub getBoundingClientRect: unsafe extern "C" fn (this: *const nsIDOMRange, _retval: *mut *const nsIDOMClientRect) -> nsresult,

}


impl nsIDOMRange {
    /* readonly attribute nsIDOMNode startContainer; */
    #[inline]
    pub unsafe fn get_startContainer(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_startContainer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long startOffset; */
    #[inline]
    pub unsafe fn get_startOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_startOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMNode endContainer; */
    #[inline]
    pub unsafe fn get_endContainer(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_endContainer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long endOffset; */
    #[inline]
    pub unsafe fn get_endOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_endOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean collapsed; */
    #[inline]
    pub unsafe fn get_collapsed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_collapsed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMNode commonAncestorContainer; */
    #[inline]
    pub unsafe fn get_commonAncestorContainer(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_commonAncestorContainer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setStart (in nsIDOMNode refNode, in long offset); */
    #[inline]
    pub unsafe fn setStart(&self, refNode: Option<&nsIDOMNode>, offset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setStart)(self as *const _, refNode.map_or(::std::ptr::null(), |x| x as *const _), offset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setEnd (in nsIDOMNode refNode, in long offset); */
    #[inline]
    pub unsafe fn setEnd(&self, refNode: Option<&nsIDOMNode>, offset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setEnd)(self as *const _, refNode.map_or(::std::ptr::null(), |x| x as *const _), offset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setStartBefore (in nsIDOMNode refNode); */
    #[inline]
    pub unsafe fn setStartBefore(&self, refNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).setStartBefore)(self as *const _, refNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setStartAfter (in nsIDOMNode refNode); */
    #[inline]
    pub unsafe fn setStartAfter(&self, refNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).setStartAfter)(self as *const _, refNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setEndBefore (in nsIDOMNode refNode); */
    #[inline]
    pub unsafe fn setEndBefore(&self, refNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).setEndBefore)(self as *const _, refNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setEndAfter (in nsIDOMNode refNode); */
    #[inline]
    pub unsafe fn setEndAfter(&self, refNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).setEndAfter)(self as *const _, refNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void collapse (in boolean toStart); */
    #[inline]
    pub unsafe fn collapse(&self, toStart: bool) -> Result<(), nsresult> {

        match ((*self.vtable).collapse)(self as *const _, toStart) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectNode (in nsIDOMNode refNode); */
    #[inline]
    pub unsafe fn selectNode(&self, refNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).selectNode)(self as *const _, refNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectNodeContents (in nsIDOMNode refNode); */
    #[inline]
    pub unsafe fn selectNodeContents(&self, refNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).selectNodeContents)(self as *const _, refNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* short compareBoundaryPoints (in unsigned short how, in nsIDOMRange sourceRange); */
    #[inline]
    pub unsafe fn compareBoundaryPoints(&self, how: libc::uint16_t, sourceRange: Option<&nsIDOMRange>) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).compareBoundaryPoints)(self as *const _, how, sourceRange.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void deleteContents (); */
    #[inline]
    pub unsafe fn deleteContents(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).deleteContents)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMDocumentFragment extractContents (); */
    #[inline]
    pub unsafe fn extractContents(&self, ) -> Result<Option<RefPtr<nsIDOMDocumentFragment>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).extractContents)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDocumentFragment cloneContents (); */
    #[inline]
    pub unsafe fn cloneContents(&self, ) -> Result<Option<RefPtr<nsIDOMDocumentFragment>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).cloneContents)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void insertNode (in nsIDOMNode newNode); */
    #[inline]
    pub unsafe fn insertNode(&self, newNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).insertNode)(self as *const _, newNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void surroundContents (in nsIDOMNode newParent); */
    #[inline]
    pub unsafe fn surroundContents(&self, newParent: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).surroundContents)(self as *const _, newParent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMRange cloneRange (); */
    #[inline]
    pub unsafe fn cloneRange(&self, ) -> Result<Option<RefPtr<nsIDOMRange>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).cloneRange)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
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

    /* void detach (); */
    #[inline]
    pub unsafe fn detach(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).detach)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMDocumentFragment createContextualFragment (in DOMString fragment); */
    #[inline]
    pub unsafe fn createContextualFragment(&self, fragment: &[u16]) -> Result<Option<RefPtr<nsIDOMDocumentFragment>>, nsresult> {
        let fragment = nsString::from(fragment);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createContextualFragment)(self as *const _, &*fragment, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean isPointInRange (in nsIDOMNode parent, in long offset); */
    #[inline]
    pub unsafe fn isPointInRange(&self, parent: Option<&nsIDOMNode>, offset: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isPointInRange)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), offset, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* short comparePoint (in nsIDOMNode parent, in long offset); */
    #[inline]
    pub unsafe fn comparePoint(&self, parent: Option<&nsIDOMNode>, offset: libc::int32_t) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).comparePoint)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), offset, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean intersectsNode (in nsIDOMNode node); */
    #[inline]
    pub unsafe fn intersectsNode(&self, node: Option<&nsIDOMNode>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).intersectsNode)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMClientRectList getClientRects (); */
    #[inline]
    pub unsafe fn getClientRects(&self, ) -> Result<Option<RefPtr<nsIDOMClientRectList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getClientRects)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMClientRect getBoundingClientRect (); */
    #[inline]
    pub unsafe fn getBoundingClientRect(&self, ) -> Result<Option<RefPtr<nsIDOMClientRect>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getBoundingClientRect)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


