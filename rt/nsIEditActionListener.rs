//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditActionListener.idl
//


#[repr(C)]
pub struct nsIEditActionListener {
    vtable: *const nsIEditActionListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditActionListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb22907b1, 0xee93, 0x11d2,
            [0x8d, 0x50, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74])
    }
}

unsafe impl RefCounted for nsIEditActionListener {
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
pub trait nsIEditActionListenerCoerce {
    fn coerce_from(v: &nsIEditActionListener) -> &Self;
}

impl nsIEditActionListenerCoerce for nsIEditActionListener {
    #[inline]
    fn coerce_from(v: &nsIEditActionListener) -> &Self {
        v
    }
}

impl nsIEditActionListener {
    #[inline]
    pub fn coerce<T: nsIEditActionListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditActionListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEditActionListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditActionListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditActionListenerVTable {
    pub __base: nsISupportsVTable,

    /* void WillCreateNode (in DOMString aTag, in nsIDOMNode aParent, in long aPosition); */
    pub WillCreateNode: unsafe extern "C" fn (this: *const nsIEditActionListener, aTag: *const nsAString, aParent: *const nsIDOMNode, aPosition: libc::int32_t) -> nsresult,

    /* void DidCreateNode (in DOMString aTag, in nsIDOMNode aNode, in nsIDOMNode aParent, in long aPosition, in nsresult aResult); */
    pub DidCreateNode: unsafe extern "C" fn (this: *const nsIEditActionListener, aTag: *const nsAString, aNode: *const nsIDOMNode, aParent: *const nsIDOMNode, aPosition: libc::int32_t, aResult: nsresult) -> nsresult,

    /* void WillInsertNode (in nsIDOMNode aNode, in nsIDOMNode aParent, in long aPosition); */
    pub WillInsertNode: unsafe extern "C" fn (this: *const nsIEditActionListener, aNode: *const nsIDOMNode, aParent: *const nsIDOMNode, aPosition: libc::int32_t) -> nsresult,

    /* void DidInsertNode (in nsIDOMNode aNode, in nsIDOMNode aParent, in long aPosition, in nsresult aResult); */
    pub DidInsertNode: unsafe extern "C" fn (this: *const nsIEditActionListener, aNode: *const nsIDOMNode, aParent: *const nsIDOMNode, aPosition: libc::int32_t, aResult: nsresult) -> nsresult,

    /* void WillDeleteNode (in nsIDOMNode aChild); */
    pub WillDeleteNode: unsafe extern "C" fn (this: *const nsIEditActionListener, aChild: *const nsIDOMNode) -> nsresult,

    /* void DidDeleteNode (in nsIDOMNode aChild, in nsresult aResult); */
    pub DidDeleteNode: unsafe extern "C" fn (this: *const nsIEditActionListener, aChild: *const nsIDOMNode, aResult: nsresult) -> nsresult,

    /* void WillSplitNode (in nsIDOMNode aExistingRightNode, in long aOffset); */
    pub WillSplitNode: unsafe extern "C" fn (this: *const nsIEditActionListener, aExistingRightNode: *const nsIDOMNode, aOffset: libc::int32_t) -> nsresult,

    /* void DidSplitNode (in nsIDOMNode aExistingRightNode, in long aOffset, in nsIDOMNode aNewLeftNode, in nsresult aResult); */
    pub DidSplitNode: unsafe extern "C" fn (this: *const nsIEditActionListener, aExistingRightNode: *const nsIDOMNode, aOffset: libc::int32_t, aNewLeftNode: *const nsIDOMNode, aResult: nsresult) -> nsresult,

    /* void WillJoinNodes (in nsIDOMNode aLeftNode, in nsIDOMNode aRightNode, in nsIDOMNode aParent); */
    pub WillJoinNodes: unsafe extern "C" fn (this: *const nsIEditActionListener, aLeftNode: *const nsIDOMNode, aRightNode: *const nsIDOMNode, aParent: *const nsIDOMNode) -> nsresult,

    /* void DidJoinNodes (in nsIDOMNode aLeftNode, in nsIDOMNode aRightNode, in nsIDOMNode aParent, in nsresult aResult); */
    pub DidJoinNodes: unsafe extern "C" fn (this: *const nsIEditActionListener, aLeftNode: *const nsIDOMNode, aRightNode: *const nsIDOMNode, aParent: *const nsIDOMNode, aResult: nsresult) -> nsresult,

    /* void WillInsertText (in nsIDOMCharacterData aTextNode, in long aOffset, in DOMString aString); */
    pub WillInsertText: unsafe extern "C" fn (this: *const nsIEditActionListener, aTextNode: *const nsIDOMCharacterData, aOffset: libc::int32_t, aString: *const nsAString) -> nsresult,

    /* void DidInsertText (in nsIDOMCharacterData aTextNode, in long aOffset, in DOMString aString, in nsresult aResult); */
    pub DidInsertText: unsafe extern "C" fn (this: *const nsIEditActionListener, aTextNode: *const nsIDOMCharacterData, aOffset: libc::int32_t, aString: *const nsAString, aResult: nsresult) -> nsresult,

    /* void WillDeleteText (in nsIDOMCharacterData aTextNode, in long aOffset, in long aLength); */
    pub WillDeleteText: unsafe extern "C" fn (this: *const nsIEditActionListener, aTextNode: *const nsIDOMCharacterData, aOffset: libc::int32_t, aLength: libc::int32_t) -> nsresult,

    /* void DidDeleteText (in nsIDOMCharacterData aTextNode, in long aOffset, in long aLength, in nsresult aResult); */
    pub DidDeleteText: unsafe extern "C" fn (this: *const nsIEditActionListener, aTextNode: *const nsIDOMCharacterData, aOffset: libc::int32_t, aLength: libc::int32_t, aResult: nsresult) -> nsresult,

    /* void WillDeleteSelection (in nsISelection aSelection); */
    pub WillDeleteSelection: unsafe extern "C" fn (this: *const nsIEditActionListener, aSelection: *const nsISelection) -> nsresult,

    /* void DidDeleteSelection (in nsISelection aSelection); */
    pub DidDeleteSelection: unsafe extern "C" fn (this: *const nsIEditActionListener, aSelection: *const nsISelection) -> nsresult,

}


impl nsIEditActionListener {
    /* void WillCreateNode (in DOMString aTag, in nsIDOMNode aParent, in long aPosition); */
    #[inline]
    pub unsafe fn WillCreateNode(&self, aTag: &[u16], aParent: Option<&nsIDOMNode>, aPosition: libc::int32_t) -> Result<(), nsresult> {
        let aTag = nsString::from(aTag);
        match ((*self.vtable).WillCreateNode)(self as *const _, &*aTag, aParent.map_or(::std::ptr::null(), |x| x as *const _), aPosition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void DidCreateNode (in DOMString aTag, in nsIDOMNode aNode, in nsIDOMNode aParent, in long aPosition, in nsresult aResult); */
    #[inline]
    pub unsafe fn DidCreateNode(&self, aTag: &[u16], aNode: Option<&nsIDOMNode>, aParent: Option<&nsIDOMNode>, aPosition: libc::int32_t, aResult: nsresult) -> Result<(), nsresult> {
        let aTag = nsString::from(aTag);
        match ((*self.vtable).DidCreateNode)(self as *const _, &*aTag, aNode.map_or(::std::ptr::null(), |x| x as *const _), aParent.map_or(::std::ptr::null(), |x| x as *const _), aPosition, aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void WillInsertNode (in nsIDOMNode aNode, in nsIDOMNode aParent, in long aPosition); */
    #[inline]
    pub unsafe fn WillInsertNode(&self, aNode: Option<&nsIDOMNode>, aParent: Option<&nsIDOMNode>, aPosition: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).WillInsertNode)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aParent.map_or(::std::ptr::null(), |x| x as *const _), aPosition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void DidInsertNode (in nsIDOMNode aNode, in nsIDOMNode aParent, in long aPosition, in nsresult aResult); */
    #[inline]
    pub unsafe fn DidInsertNode(&self, aNode: Option<&nsIDOMNode>, aParent: Option<&nsIDOMNode>, aPosition: libc::int32_t, aResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).DidInsertNode)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aParent.map_or(::std::ptr::null(), |x| x as *const _), aPosition, aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void WillDeleteNode (in nsIDOMNode aChild); */
    #[inline]
    pub unsafe fn WillDeleteNode(&self, aChild: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).WillDeleteNode)(self as *const _, aChild.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void DidDeleteNode (in nsIDOMNode aChild, in nsresult aResult); */
    #[inline]
    pub unsafe fn DidDeleteNode(&self, aChild: Option<&nsIDOMNode>, aResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).DidDeleteNode)(self as *const _, aChild.map_or(::std::ptr::null(), |x| x as *const _), aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void WillSplitNode (in nsIDOMNode aExistingRightNode, in long aOffset); */
    #[inline]
    pub unsafe fn WillSplitNode(&self, aExistingRightNode: Option<&nsIDOMNode>, aOffset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).WillSplitNode)(self as *const _, aExistingRightNode.map_or(::std::ptr::null(), |x| x as *const _), aOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void DidSplitNode (in nsIDOMNode aExistingRightNode, in long aOffset, in nsIDOMNode aNewLeftNode, in nsresult aResult); */
    #[inline]
    pub unsafe fn DidSplitNode(&self, aExistingRightNode: Option<&nsIDOMNode>, aOffset: libc::int32_t, aNewLeftNode: Option<&nsIDOMNode>, aResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).DidSplitNode)(self as *const _, aExistingRightNode.map_or(::std::ptr::null(), |x| x as *const _), aOffset, aNewLeftNode.map_or(::std::ptr::null(), |x| x as *const _), aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void WillJoinNodes (in nsIDOMNode aLeftNode, in nsIDOMNode aRightNode, in nsIDOMNode aParent); */
    #[inline]
    pub unsafe fn WillJoinNodes(&self, aLeftNode: Option<&nsIDOMNode>, aRightNode: Option<&nsIDOMNode>, aParent: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).WillJoinNodes)(self as *const _, aLeftNode.map_or(::std::ptr::null(), |x| x as *const _), aRightNode.map_or(::std::ptr::null(), |x| x as *const _), aParent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void DidJoinNodes (in nsIDOMNode aLeftNode, in nsIDOMNode aRightNode, in nsIDOMNode aParent, in nsresult aResult); */
    #[inline]
    pub unsafe fn DidJoinNodes(&self, aLeftNode: Option<&nsIDOMNode>, aRightNode: Option<&nsIDOMNode>, aParent: Option<&nsIDOMNode>, aResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).DidJoinNodes)(self as *const _, aLeftNode.map_or(::std::ptr::null(), |x| x as *const _), aRightNode.map_or(::std::ptr::null(), |x| x as *const _), aParent.map_or(::std::ptr::null(), |x| x as *const _), aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void WillInsertText (in nsIDOMCharacterData aTextNode, in long aOffset, in DOMString aString); */
    #[inline]
    pub unsafe fn WillInsertText(&self, aTextNode: Option<&nsIDOMCharacterData>, aOffset: libc::int32_t, aString: &[u16]) -> Result<(), nsresult> {
        let aString = nsString::from(aString);
        match ((*self.vtable).WillInsertText)(self as *const _, aTextNode.map_or(::std::ptr::null(), |x| x as *const _), aOffset, &*aString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void DidInsertText (in nsIDOMCharacterData aTextNode, in long aOffset, in DOMString aString, in nsresult aResult); */
    #[inline]
    pub unsafe fn DidInsertText(&self, aTextNode: Option<&nsIDOMCharacterData>, aOffset: libc::int32_t, aString: &[u16], aResult: nsresult) -> Result<(), nsresult> {
        let aString = nsString::from(aString);
        match ((*self.vtable).DidInsertText)(self as *const _, aTextNode.map_or(::std::ptr::null(), |x| x as *const _), aOffset, &*aString, aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void WillDeleteText (in nsIDOMCharacterData aTextNode, in long aOffset, in long aLength); */
    #[inline]
    pub unsafe fn WillDeleteText(&self, aTextNode: Option<&nsIDOMCharacterData>, aOffset: libc::int32_t, aLength: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).WillDeleteText)(self as *const _, aTextNode.map_or(::std::ptr::null(), |x| x as *const _), aOffset, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void DidDeleteText (in nsIDOMCharacterData aTextNode, in long aOffset, in long aLength, in nsresult aResult); */
    #[inline]
    pub unsafe fn DidDeleteText(&self, aTextNode: Option<&nsIDOMCharacterData>, aOffset: libc::int32_t, aLength: libc::int32_t, aResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).DidDeleteText)(self as *const _, aTextNode.map_or(::std::ptr::null(), |x| x as *const _), aOffset, aLength, aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void WillDeleteSelection (in nsISelection aSelection); */
    #[inline]
    pub unsafe fn WillDeleteSelection(&self, aSelection: Option<&nsISelection>) -> Result<(), nsresult> {

        match ((*self.vtable).WillDeleteSelection)(self as *const _, aSelection.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void DidDeleteSelection (in nsISelection aSelection); */
    #[inline]
    pub unsafe fn DidDeleteSelection(&self, aSelection: Option<&nsISelection>) -> Result<(), nsresult> {

        match ((*self.vtable).DidDeleteSelection)(self as *const _, aSelection.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


