//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIRefObject.idl
//


#[repr(C)]
pub struct nsIURIRefObject {
    vtable: *const nsIURIRefObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURIRefObject {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2226927e, 0x1dd2, 0x11b2,
            [0xb5, 0x7f, 0xfa, 0xab, 0x47, 0x28, 0x85, 0x63])
    }
}

unsafe impl RefCounted for nsIURIRefObject {
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
pub trait nsIURIRefObjectCoerce {
    fn coerce_from(v: &nsIURIRefObject) -> &Self;
}

impl nsIURIRefObjectCoerce for nsIURIRefObject {
    #[inline]
    fn coerce_from(v: &nsIURIRefObject) -> &Self {
        v
    }
}

impl nsIURIRefObject {
    #[inline]
    pub fn coerce<T: nsIURIRefObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURIRefObject {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURIRefObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIRefObject) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURIRefObjectVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIDOMNode node; */
    pub get_node: unsafe extern "C" fn (this: *const nsIURIRefObject, aNode: *mut *const nsIDOMNode) -> nsresult,
    pub set_node: unsafe extern "C" fn (this: *const nsIURIRefObject, aNode: *const nsIDOMNode) -> nsresult,

    /* void Reset (); */
    pub Reset: unsafe extern "C" fn (this: *const nsIURIRefObject) -> nsresult,

    /* DOMString GetNextURI (); */
    pub GetNextURI: unsafe extern "C" fn (this: *const nsIURIRefObject, _retval: *mut nsAString) -> nsresult,

    /* void RewriteAllURIs (in DOMString aOldPat, in DOMString aNewPat, in boolean aMakeRel); */
    pub RewriteAllURIs: unsafe extern "C" fn (this: *const nsIURIRefObject, aOldPat: *const nsAString, aNewPat: *const nsAString, aMakeRel: bool) -> nsresult,

}


impl nsIURIRefObject {
    /* attribute nsIDOMNode node; */
    #[inline]
    pub unsafe fn get_node(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_node)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_node(&self, aNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).set_node)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void Reset (); */
    #[inline]
    pub unsafe fn Reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).Reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMString GetNextURI (); */
    #[inline]
    pub unsafe fn GetNextURI(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).GetNextURI)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void RewriteAllURIs (in DOMString aOldPat, in DOMString aNewPat, in boolean aMakeRel); */
    #[inline]
    pub unsafe fn RewriteAllURIs(&self, aOldPat: &[u16], aNewPat: &[u16], aMakeRel: bool) -> Result<(), nsresult> {
        let aOldPat = nsString::from(aOldPat);
        let aNewPat = nsString::from(aNewPat);
        match ((*self.vtable).RewriteAllURIs)(self as *const _, &*aOldPat, &*aNewPat, aMakeRel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


