//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessible.idl
//


#[repr(C)]
pub struct nsIAccessible {
    vtable: *const nsIAccessibleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessible {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xde2869d9, 0x563c, 0x4943,
            [0x99, 0x6b, 0x31, 0xa4, 0xda, 0xa4, 0xd0, 0x97])
    }
}

unsafe impl RefCounted for nsIAccessible {
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
pub trait nsIAccessibleCoerce {
    fn coerce_from(v: &nsIAccessible) -> &Self;
}

impl nsIAccessibleCoerce for nsIAccessible {
    #[inline]
    fn coerce_from(v: &nsIAccessible) -> &Self {
        v
    }
}

impl nsIAccessible {
    #[inline]
    pub fn coerce<T: nsIAccessibleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessible {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessible) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAccessible parent; */
    pub get_parent: unsafe extern "C" fn (this: *const nsIAccessible, aParent: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIAccessible nextSibling; */
    pub get_nextSibling: unsafe extern "C" fn (this: *const nsIAccessible, aNextSibling: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIAccessible previousSibling; */
    pub get_previousSibling: unsafe extern "C" fn (this: *const nsIAccessible, aPreviousSibling: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIAccessible firstChild; */
    pub get_firstChild: unsafe extern "C" fn (this: *const nsIAccessible, aFirstChild: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIAccessible lastChild; */
    pub get_lastChild: unsafe extern "C" fn (this: *const nsIAccessible, aLastChild: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIArray children; */
    pub get_children: unsafe extern "C" fn (this: *const nsIAccessible, aChildren: *mut *const nsIArray) -> nsresult,

    /* readonly attribute long childCount; */
    pub get_childCount: unsafe extern "C" fn (this: *const nsIAccessible, aChildCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long indexInParent; */
    pub get_indexInParent: unsafe extern "C" fn (this: *const nsIAccessible, aIndexInParent: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsIDOMNode DOMNode; */
    pub get_DOMNode: unsafe extern "C" fn (this: *const nsIAccessible, aDOMNode: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute DOMString id; */
    pub get_id: unsafe extern "C" fn (this: *const nsIAccessible, aId: *mut nsAString) -> nsresult,

    /* readonly attribute nsIAccessibleDocument document; */
    pub get_document: unsafe extern "C" fn (this: *const nsIAccessible, aDocument: *mut *const nsIAccessibleDocument) -> nsresult,

    /* readonly attribute nsIAccessibleDocument rootDocument; */
    pub get_rootDocument: unsafe extern "C" fn (this: *const nsIAccessible, aRootDocument: *mut *const nsIAccessibleDocument) -> nsresult,

    /* readonly attribute DOMString language; */
    pub get_language: unsafe extern "C" fn (this: *const nsIAccessible, aLanguage: *mut nsAString) -> nsresult,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIAccessible, aName: *mut nsAString) -> nsresult,

    /* readonly attribute AString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIAccessible, aValue: *mut nsAString) -> nsresult,

    /* readonly attribute AString description; */
    pub get_description: unsafe extern "C" fn (this: *const nsIAccessible, aDescription: *mut nsAString) -> nsresult,

    /* readonly attribute AString accessKey; */
    pub get_accessKey: unsafe extern "C" fn (this: *const nsIAccessible, aAccessKey: *mut nsAString) -> nsresult,

    /* readonly attribute AString keyboardShortcut; */
    pub get_keyboardShortcut: unsafe extern "C" fn (this: *const nsIAccessible, aKeyboardShortcut: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned long role; */
    pub get_role: unsafe extern "C" fn (this: *const nsIAccessible, aRole: *mut libc::uint32_t) -> nsresult,

    /* void getState (out unsigned long aState, out unsigned long aExtraState); */
    pub getState: unsafe extern "C" fn (this: *const nsIAccessible, aState: *mut libc::uint32_t, aExtraState: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute AString help; */
    pub get_help: unsafe extern "C" fn (this: *const nsIAccessible, aHelp: *mut nsAString) -> nsresult,

    /* readonly attribute nsIAccessible focusedChild; */
    pub get_focusedChild: unsafe extern "C" fn (this: *const nsIAccessible, aFocusedChild: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIPersistentProperties attributes; */
    pub get_attributes: unsafe extern "C" fn (this: *const nsIAccessible, aAttributes: *mut *const nsIPersistentProperties) -> nsresult,

    /* void groupPosition (out long aGroupLevel, out long aSimilarItemsInGroup, out long aPositionInGroup); */
    pub groupPosition: unsafe extern "C" fn (this: *const nsIAccessible, aGroupLevel: *mut libc::int32_t, aSimilarItemsInGroup: *mut libc::int32_t, aPositionInGroup: *mut libc::int32_t) -> nsresult,

    /* nsIAccessible getChildAtPoint (in long x, in long y); */
    pub getChildAtPoint: unsafe extern "C" fn (this: *const nsIAccessible, x: libc::int32_t, y: libc::int32_t, _retval: *mut *const nsIAccessible) -> nsresult,

    /* nsIAccessible getDeepestChildAtPoint (in long x, in long y); */
    pub getDeepestChildAtPoint: unsafe extern "C" fn (this: *const nsIAccessible, x: libc::int32_t, y: libc::int32_t, _retval: *mut *const nsIAccessible) -> nsresult,

    /* nsIAccessible getChildAt (in long aChildIndex); */
    pub getChildAt: unsafe extern "C" fn (this: *const nsIAccessible, aChildIndex: libc::int32_t, _retval: *mut *const nsIAccessible) -> nsresult,

    /* nsIAccessibleRelation getRelationByType (in unsigned long aRelationType); */
    pub getRelationByType: unsafe extern "C" fn (this: *const nsIAccessible, aRelationType: libc::uint32_t, _retval: *mut *const nsIAccessibleRelation) -> nsresult,

    /* nsIArray getRelations (); */
    pub getRelations: unsafe extern "C" fn (this: *const nsIAccessible, _retval: *mut *const nsIArray) -> nsresult,

    /* void getBounds (out long x, out long y, out long width, out long height); */
    pub getBounds: unsafe extern "C" fn (this: *const nsIAccessible, x: *mut libc::int32_t, y: *mut libc::int32_t, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* void setSelected (in boolean isSelected); */
    pub setSelected: unsafe extern "C" fn (this: *const nsIAccessible, isSelected: bool) -> nsresult,

    /* void takeSelection (); */
    pub takeSelection: unsafe extern "C" fn (this: *const nsIAccessible) -> nsresult,

    /* void takeFocus (); */
    pub takeFocus: unsafe extern "C" fn (this: *const nsIAccessible) -> nsresult,

    /* readonly attribute uint8_t actionCount; */
    pub get_actionCount: unsafe extern "C" fn (this: *const nsIAccessible, aActionCount: *mut uint8_t) -> nsresult,

    /* AString getActionName (in uint8_t index); */
    pub getActionName: unsafe extern "C" fn (this: *const nsIAccessible, index: uint8_t, _retval: *mut nsAString) -> nsresult,

    /* AString getActionDescription (in uint8_t aIndex); */
    pub getActionDescription: unsafe extern "C" fn (this: *const nsIAccessible, aIndex: uint8_t, _retval: *mut nsAString) -> nsresult,

    /* void doAction (in uint8_t index); */
    pub doAction: unsafe extern "C" fn (this: *const nsIAccessible, index: uint8_t) -> nsresult,

    /* void scrollTo (in unsigned long aScrollType); */
    pub scrollTo: unsafe extern "C" fn (this: *const nsIAccessible, aScrollType: libc::uint32_t) -> nsresult,

    /* void scrollToPoint (in unsigned long coordinateType, in long x, in long y); */
    pub scrollToPoint: unsafe extern "C" fn (this: *const nsIAccessible, coordinateType: libc::uint32_t, x: libc::int32_t, y: libc::int32_t) -> nsresult,

}


impl nsIAccessible {
    /* readonly attribute nsIAccessible parent; */
    #[inline]
    pub unsafe fn get_parent(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAccessible nextSibling; */
    #[inline]
    pub unsafe fn get_nextSibling(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_nextSibling)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAccessible previousSibling; */
    #[inline]
    pub unsafe fn get_previousSibling(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_previousSibling)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAccessible firstChild; */
    #[inline]
    pub unsafe fn get_firstChild(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_firstChild)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAccessible lastChild; */
    #[inline]
    pub unsafe fn get_lastChild(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_lastChild)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIArray children; */
    #[inline]
    pub unsafe fn get_children(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_children)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long childCount; */
    #[inline]
    pub unsafe fn get_childCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_childCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long indexInParent; */
    #[inline]
    pub unsafe fn get_indexInParent(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_indexInParent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMNode DOMNode; */
    #[inline]
    pub unsafe fn get_DOMNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_DOMNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_id)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIAccessibleDocument document; */
    #[inline]
    pub unsafe fn get_document(&self, ) -> Result<Option<RefPtr<nsIAccessibleDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_document)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAccessibleDocument rootDocument; */
    #[inline]
    pub unsafe fn get_rootDocument(&self, ) -> Result<Option<RefPtr<nsIAccessibleDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rootDocument)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString language; */
    #[inline]
    pub unsafe fn get_language(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_language)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_value)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString description; */
    #[inline]
    pub unsafe fn get_description(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_description)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString accessKey; */
    #[inline]
    pub unsafe fn get_accessKey(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_accessKey)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString keyboardShortcut; */
    #[inline]
    pub unsafe fn get_keyboardShortcut(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_keyboardShortcut)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long role; */
    #[inline]
    pub unsafe fn get_role(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_role)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getState (out unsigned long aState, out unsigned long aExtraState); */
    #[inline]
    pub unsafe fn getState(&self, ) -> Result<(libc::uint32_t, libc::uint32_t), nsresult> {
        let mut aState: libc::uint32_t = ::std::mem::zeroed();
        let mut aExtraState: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getState)(self as *const _, &mut aState as *mut _, &mut aExtraState as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aState, aExtraState))
    }

    /* readonly attribute AString help; */
    #[inline]
    pub unsafe fn get_help(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_help)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIAccessible focusedChild; */
    #[inline]
    pub unsafe fn get_focusedChild(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_focusedChild)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIPersistentProperties attributes; */
    #[inline]
    pub unsafe fn get_attributes(&self, ) -> Result<Option<RefPtr<nsIPersistentProperties>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_attributes)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void groupPosition (out long aGroupLevel, out long aSimilarItemsInGroup, out long aPositionInGroup); */
    #[inline]
    pub unsafe fn groupPosition(&self, ) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut aGroupLevel: libc::int32_t = ::std::mem::zeroed();
        let mut aSimilarItemsInGroup: libc::int32_t = ::std::mem::zeroed();
        let mut aPositionInGroup: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).groupPosition)(self as *const _, &mut aGroupLevel as *mut _, &mut aSimilarItemsInGroup as *mut _, &mut aPositionInGroup as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aGroupLevel, aSimilarItemsInGroup, aPositionInGroup))
    }

    /* nsIAccessible getChildAtPoint (in long x, in long y); */
    #[inline]
    pub unsafe fn getChildAtPoint(&self, x: libc::int32_t, y: libc::int32_t) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChildAtPoint)(self as *const _, x, y, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAccessible getDeepestChildAtPoint (in long x, in long y); */
    #[inline]
    pub unsafe fn getDeepestChildAtPoint(&self, x: libc::int32_t, y: libc::int32_t) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getDeepestChildAtPoint)(self as *const _, x, y, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAccessible getChildAt (in long aChildIndex); */
    #[inline]
    pub unsafe fn getChildAt(&self, aChildIndex: libc::int32_t) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChildAt)(self as *const _, aChildIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAccessibleRelation getRelationByType (in unsigned long aRelationType); */
    #[inline]
    pub unsafe fn getRelationByType(&self, aRelationType: libc::uint32_t) -> Result<Option<RefPtr<nsIAccessibleRelation>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRelationByType)(self as *const _, aRelationType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIArray getRelations (); */
    #[inline]
    pub unsafe fn getRelations(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRelations)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getBounds (out long x, out long y, out long width, out long height); */
    #[inline]
    pub unsafe fn getBounds(&self, ) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut x: libc::int32_t = ::std::mem::zeroed();
        let mut y: libc::int32_t = ::std::mem::zeroed();
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getBounds)(self as *const _, &mut x as *mut _, &mut y as *mut _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((x, y, width, height))
    }

    /* void setSelected (in boolean isSelected); */
    #[inline]
    pub unsafe fn setSelected(&self, isSelected: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setSelected)(self as *const _, isSelected) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void takeSelection (); */
    #[inline]
    pub unsafe fn takeSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).takeSelection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void takeFocus (); */
    #[inline]
    pub unsafe fn takeFocus(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).takeFocus)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute uint8_t actionCount; */
    #[inline]
    pub unsafe fn get_actionCount(&self, ) -> Result<uint8_t, nsresult> {
        let mut _retval: uint8_t = ::std::mem::zeroed();
        match ((*self.vtable).get_actionCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getActionName (in uint8_t index); */
    #[inline]
    pub unsafe fn getActionName(&self, index: uint8_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getActionName)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getActionDescription (in uint8_t aIndex); */
    #[inline]
    pub unsafe fn getActionDescription(&self, aIndex: uint8_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getActionDescription)(self as *const _, aIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void doAction (in uint8_t index); */
    #[inline]
    pub unsafe fn doAction(&self, index: uint8_t) -> Result<(), nsresult> {

        match ((*self.vtable).doAction)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollTo (in unsigned long aScrollType); */
    #[inline]
    pub unsafe fn scrollTo(&self, aScrollType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollTo)(self as *const _, aScrollType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollToPoint (in unsigned long coordinateType, in long x, in long y); */
    #[inline]
    pub unsafe fn scrollToPoint(&self, coordinateType: libc::uint32_t, x: libc::int32_t, y: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollToPoint)(self as *const _, coordinateType, x, y) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


