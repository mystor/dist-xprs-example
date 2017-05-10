//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentFilter.idl
//


#[repr(C)]
pub struct nsIContentFilter {
    vtable: *const nsIContentFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentFilter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc18c49a8, 0x62f0, 0x4045,
            [0x98, 0x84, 0x4a, 0xa9, 0x1e, 0x38, 0x8f, 0x14])
    }
}

unsafe impl RefCounted for nsIContentFilter {
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
pub trait nsIContentFilterCoerce {
    fn coerce_from(v: &nsIContentFilter) -> &Self;
}

impl nsIContentFilterCoerce for nsIContentFilter {
    #[inline]
    fn coerce_from(v: &nsIContentFilter) -> &Self {
        v
    }
}

impl nsIContentFilter {
    #[inline]
    pub fn coerce<T: nsIContentFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentFilter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentFilter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentFilterVTable {
    pub __base: nsISupportsVTable,

    /* void notifyOfInsertion (in AString mimeType, in nsIURL contentSourceURL, in nsIDOMDocument sourceDocument, in boolean willDeleteSelection, inout nsIDOMNode docFragment, inout nsIDOMNode contentStartNode, inout long contentStartOffset, inout nsIDOMNode contentEndNode, inout long contentEndOffset, inout nsIDOMNode insertionPointNode, inout long insertionPointOffset, out boolean continueWithInsertion); */
    pub notifyOfInsertion: unsafe extern "C" fn (this: *const nsIContentFilter, mimeType: *const nsAString, contentSourceURL: *const nsIURL, sourceDocument: *const nsIDOMDocument, willDeleteSelection: bool, docFragment: *mut *const nsIDOMNode, contentStartNode: *mut *const nsIDOMNode, contentStartOffset: *mut libc::int32_t, contentEndNode: *mut *const nsIDOMNode, contentEndOffset: *mut libc::int32_t, insertionPointNode: *mut *const nsIDOMNode, insertionPointOffset: *mut libc::int32_t, continueWithInsertion: *mut bool) -> nsresult,

}


impl nsIContentFilter {
    /* void notifyOfInsertion (in AString mimeType, in nsIURL contentSourceURL, in nsIDOMDocument sourceDocument, in boolean willDeleteSelection, inout nsIDOMNode docFragment, inout nsIDOMNode contentStartNode, inout long contentStartOffset, inout nsIDOMNode contentEndNode, inout long contentEndOffset, inout nsIDOMNode insertionPointNode, inout long insertionPointOffset, out boolean continueWithInsertion); */
    #[inline]
    pub unsafe fn notifyOfInsertion(&self, mimeType: &[u16], contentSourceURL: Option<&nsIURL>, sourceDocument: Option<&nsIDOMDocument>, willDeleteSelection: bool) -> Result<(Option<RefPtr<nsIDOMNode>>, Option<RefPtr<nsIDOMNode>>, libc::int32_t, Option<RefPtr<nsIDOMNode>>, libc::int32_t, Option<RefPtr<nsIDOMNode>>, libc::int32_t, bool), nsresult> {
        let mimeType = nsString::from(mimeType);
        let mut docFragment = GetterAddrefs::new();
        let mut contentStartNode = GetterAddrefs::new();
        let mut contentStartOffset: libc::int32_t = ::std::mem::zeroed();
        let mut contentEndNode = GetterAddrefs::new();
        let mut contentEndOffset: libc::int32_t = ::std::mem::zeroed();
        let mut insertionPointNode = GetterAddrefs::new();
        let mut insertionPointOffset: libc::int32_t = ::std::mem::zeroed();
        let mut continueWithInsertion: bool = ::std::mem::zeroed();
        match ((*self.vtable).notifyOfInsertion)(self as *const _, &*mimeType, contentSourceURL.map_or(::std::ptr::null(), |x| x as *const _), sourceDocument.map_or(::std::ptr::null(), |x| x as *const _), willDeleteSelection, docFragment.ptr(), contentStartNode.ptr(), &mut contentStartOffset as *mut _, contentEndNode.ptr(), &mut contentEndOffset as *mut _, insertionPointNode.ptr(), &mut insertionPointOffset as *mut _, &mut continueWithInsertion as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((docFragment.refptr(), contentStartNode.refptr(), contentStartOffset, contentEndNode.refptr(), contentEndOffset, insertionPointNode.refptr(), insertionPointOffset, continueWithInsertion))
    }

}


