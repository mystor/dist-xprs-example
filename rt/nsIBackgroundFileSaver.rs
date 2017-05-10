//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBackgroundFileSaver.idl
//


#[repr(C)]
pub struct nsIBackgroundFileSaver {
    vtable: *const nsIBackgroundFileSaverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBackgroundFileSaver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc43544a4, 0x682c, 0x4262,
            [0xb4, 0x07, 0x24, 0x53, 0xd2, 0x6e, 0x66, 0x0d])
    }
}

unsafe impl RefCounted for nsIBackgroundFileSaver {
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
pub trait nsIBackgroundFileSaverCoerce {
    fn coerce_from(v: &nsIBackgroundFileSaver) -> &Self;
}

impl nsIBackgroundFileSaverCoerce for nsIBackgroundFileSaver {
    #[inline]
    fn coerce_from(v: &nsIBackgroundFileSaver) -> &Self {
        v
    }
}

impl nsIBackgroundFileSaver {
    #[inline]
    pub fn coerce<T: nsIBackgroundFileSaverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBackgroundFileSaver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBackgroundFileSaverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBackgroundFileSaver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBackgroundFileSaverVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIBackgroundFileSaverObserver observer; */
    pub get_observer: unsafe extern "C" fn (this: *const nsIBackgroundFileSaver, aObserver: *mut *const nsIBackgroundFileSaverObserver) -> nsresult,
    pub set_observer: unsafe extern "C" fn (this: *const nsIBackgroundFileSaver, aObserver: *const nsIBackgroundFileSaverObserver) -> nsresult,

    /* readonly attribute nsIArray signatureInfo; */
    pub get_signatureInfo: unsafe extern "C" fn (this: *const nsIBackgroundFileSaver, aSignatureInfo: *mut *const nsIArray) -> nsresult,

    /* readonly attribute ACString sha256Hash; */
    pub get_sha256Hash: unsafe extern "C" fn (this: *const nsIBackgroundFileSaver, aSha256Hash: *mut nsACString) -> nsresult,

    /* void enableSignatureInfo (); */
    pub enableSignatureInfo: unsafe extern "C" fn (this: *const nsIBackgroundFileSaver) -> nsresult,

    /* void enableSha256 (); */
    pub enableSha256: unsafe extern "C" fn (this: *const nsIBackgroundFileSaver) -> nsresult,

    /* void enableAppend (); */
    pub enableAppend: unsafe extern "C" fn (this: *const nsIBackgroundFileSaver) -> nsresult,

    /* void setTarget (in nsIFile aTarget, in bool aKeepPartial); */
    pub setTarget: unsafe extern "C" fn (this: *const nsIBackgroundFileSaver, aTarget: *const nsIFile, aKeepPartial: bool) -> nsresult,

    /* void finish (in nsresult aStatus); */
    pub finish: unsafe extern "C" fn (this: *const nsIBackgroundFileSaver, aStatus: nsresult) -> nsresult,

}


impl nsIBackgroundFileSaver {
    /* attribute nsIBackgroundFileSaverObserver observer; */
    #[inline]
    pub unsafe fn get_observer(&self, ) -> Result<Option<RefPtr<nsIBackgroundFileSaverObserver>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_observer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_observer(&self, aObserver: Option<&nsIBackgroundFileSaverObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).set_observer)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIArray signatureInfo; */
    #[inline]
    pub unsafe fn get_signatureInfo(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_signatureInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute ACString sha256Hash; */
    #[inline]
    pub unsafe fn get_sha256Hash(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_sha256Hash)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void enableSignatureInfo (); */
    #[inline]
    pub unsafe fn enableSignatureInfo(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).enableSignatureInfo)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void enableSha256 (); */
    #[inline]
    pub unsafe fn enableSha256(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).enableSha256)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void enableAppend (); */
    #[inline]
    pub unsafe fn enableAppend(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).enableAppend)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setTarget (in nsIFile aTarget, in bool aKeepPartial); */
    #[inline]
    pub unsafe fn setTarget(&self, aTarget: Option<&nsIFile>, aKeepPartial: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setTarget)(self as *const _, aTarget.map_or(::std::ptr::null(), |x| x as *const _), aKeepPartial) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void finish (in nsresult aStatus); */
    #[inline]
    pub unsafe fn finish(&self, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).finish)(self as *const _, aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIBackgroundFileSaverObserver {
    vtable: *const nsIBackgroundFileSaverObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBackgroundFileSaverObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xee7058c3, 0x6e54, 0x4411,
            [0xb7, 0x6b, 0x3c, 0xe8, 0x7b, 0x76, 0xfc, 0xb6])
    }
}

unsafe impl RefCounted for nsIBackgroundFileSaverObserver {
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
pub trait nsIBackgroundFileSaverObserverCoerce {
    fn coerce_from(v: &nsIBackgroundFileSaverObserver) -> &Self;
}

impl nsIBackgroundFileSaverObserverCoerce for nsIBackgroundFileSaverObserver {
    #[inline]
    fn coerce_from(v: &nsIBackgroundFileSaverObserver) -> &Self {
        v
    }
}

impl nsIBackgroundFileSaverObserver {
    #[inline]
    pub fn coerce<T: nsIBackgroundFileSaverObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBackgroundFileSaverObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBackgroundFileSaverObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBackgroundFileSaverObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBackgroundFileSaverObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onTargetChange (in nsIBackgroundFileSaver aSaver, in nsIFile aTarget); */
    pub onTargetChange: unsafe extern "C" fn (this: *const nsIBackgroundFileSaverObserver, aSaver: *const nsIBackgroundFileSaver, aTarget: *const nsIFile) -> nsresult,

    /* void onSaveComplete (in nsIBackgroundFileSaver aSaver, in nsresult aStatus); */
    pub onSaveComplete: unsafe extern "C" fn (this: *const nsIBackgroundFileSaverObserver, aSaver: *const nsIBackgroundFileSaver, aStatus: nsresult) -> nsresult,

}


impl nsIBackgroundFileSaverObserver {
    /* void onTargetChange (in nsIBackgroundFileSaver aSaver, in nsIFile aTarget); */
    #[inline]
    pub unsafe fn onTargetChange(&self, aSaver: Option<&nsIBackgroundFileSaver>, aTarget: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).onTargetChange)(self as *const _, aSaver.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSaveComplete (in nsIBackgroundFileSaver aSaver, in nsresult aStatus); */
    #[inline]
    pub unsafe fn onSaveComplete(&self, aSaver: Option<&nsIBackgroundFileSaver>, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onSaveComplete)(self as *const _, aSaver.map_or(::std::ptr::null(), |x| x as *const _), aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


