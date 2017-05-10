//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransfer.idl
//


#[repr(C)]
pub struct nsITransfer {
    vtable: *const nsITransferVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITransfer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x37ec75d3, 0x97ad, 0x4da8,
            [0xaf, 0xaa, 0xea, 0xbe, 0x5b, 0x4a, 0xfd, 0x73])
    }
}

unsafe impl RefCounted for nsITransfer {
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
pub trait nsITransferCoerce {
    fn coerce_from(v: &nsITransfer) -> &Self;
}

impl nsITransferCoerce for nsITransfer {
    #[inline]
    fn coerce_from(v: &nsITransfer) -> &Self {
        v
    }
}

impl nsITransfer {
    #[inline]
    pub fn coerce<T: nsITransferCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITransfer {
    type Target = nsIWebProgressListener2;
    #[inline]
    fn deref(&self) -> &nsIWebProgressListener2 {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIWebProgressListener2Coerce> nsITransferCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransfer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITransferVTable {
    pub __base: nsIWebProgressListener2VTable,

    /* void init (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate); */
    pub init: unsafe extern "C" fn (this: *const nsITransfer, aSource: *const nsIURI, aTarget: *const nsIURI, aDisplayName: *const nsAString, aMIMEInfo: *const nsIMIMEInfo, startTime: PRTime, aTempFile: *const nsIFile, aCancelable: *const nsICancelable, aIsPrivate: bool) -> nsresult,

    /* void setSha256Hash (in ACString aHash); */
    pub setSha256Hash: unsafe extern "C" fn (this: *const nsITransfer, aHash: *const nsACString) -> nsresult,

    /* void setSignatureInfo (in nsIArray aSignatureInfo); */
    pub setSignatureInfo: unsafe extern "C" fn (this: *const nsITransfer, aSignatureInfo: *const nsIArray) -> nsresult,

    /* void setRedirects (in nsIArray aRedirects); */
    pub setRedirects: unsafe extern "C" fn (this: *const nsITransfer, aRedirects: *const nsIArray) -> nsresult,

}


impl nsITransfer {
    /* void init (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate); */
    #[inline]
    pub unsafe fn init(&self, aSource: Option<&nsIURI>, aTarget: Option<&nsIURI>, aDisplayName: &[u16], aMIMEInfo: Option<&nsIMIMEInfo>, startTime: PRTime, aTempFile: Option<&nsIFile>, aCancelable: Option<&nsICancelable>, aIsPrivate: bool) -> Result<(), nsresult> {
        let aDisplayName = nsString::from(aDisplayName);
        match ((*self.vtable).init)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _), &*aDisplayName, aMIMEInfo.map_or(::std::ptr::null(), |x| x as *const _), startTime, aTempFile.map_or(::std::ptr::null(), |x| x as *const _), aCancelable.map_or(::std::ptr::null(), |x| x as *const _), aIsPrivate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setSha256Hash (in ACString aHash); */
    #[inline]
    pub unsafe fn setSha256Hash(&self, aHash: &[u8]) -> Result<(), nsresult> {
        let aHash = nsCString::from(aHash);
        match ((*self.vtable).setSha256Hash)(self as *const _, &*aHash) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setSignatureInfo (in nsIArray aSignatureInfo); */
    #[inline]
    pub unsafe fn setSignatureInfo(&self, aSignatureInfo: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).setSignatureInfo)(self as *const _, aSignatureInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setRedirects (in nsIArray aRedirects); */
    #[inline]
    pub unsafe fn setRedirects(&self, aRedirects: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).setRedirects)(self as *const _, aRedirects.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


