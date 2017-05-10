//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILayoutHistoryState.idl
//


#[repr(C)]
pub struct nsILayoutHistoryState {
    vtable: *const nsILayoutHistoryStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILayoutHistoryState {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaef27cb3, 0x4df9, 0x4eeb,
            [0xb0, 0xb0, 0xac, 0x56, 0xcf, 0x86, 0x1d, 0x04])
    }
}

unsafe impl RefCounted for nsILayoutHistoryState {
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
pub trait nsILayoutHistoryStateCoerce {
    fn coerce_from(v: &nsILayoutHistoryState) -> &Self;
}

impl nsILayoutHistoryStateCoerce for nsILayoutHistoryState {
    #[inline]
    fn coerce_from(v: &nsILayoutHistoryState) -> &Self {
        v
    }
}

impl nsILayoutHistoryState {
    #[inline]
    pub fn coerce<T: nsILayoutHistoryStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILayoutHistoryState {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILayoutHistoryStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILayoutHistoryState) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILayoutHistoryStateVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean hasStates; */
    pub get_hasStates: unsafe extern "C" fn (this: *const nsILayoutHistoryState, aHasStates: *mut bool) -> nsresult,

    /* void getKeys ([optional] out uint32_t aCount, [array, size_is (aCount), retval] out string aKeys); */
    /// Unable to call function as its signature contains a non-rust type
    pub getKeys: *const ::libc::c_void,

    /* void getPresState (in ACString aKey, out float aScrollX, out float aScrollY, out boolean aAllowScrollOriginDowngrade, out float aRes, out boolean aScaleToRes); */
    pub getPresState: unsafe extern "C" fn (this: *const nsILayoutHistoryState, aKey: *const nsACString, aScrollX: *mut libc::c_float, aScrollY: *mut libc::c_float, aAllowScrollOriginDowngrade: *mut bool, aRes: *mut libc::c_float, aScaleToRes: *mut bool) -> nsresult,

    /* void addNewPresState (in ACString aKey, in float aScrollX, in float aScrollY, in boolean aAllowScrollOriginDowngrade, in float aRes, in boolean aScaleToRes); */
    pub addNewPresState: unsafe extern "C" fn (this: *const nsILayoutHistoryState, aKey: *const nsACString, aScrollX: libc::c_float, aScrollY: libc::c_float, aAllowScrollOriginDowngrade: bool, aRes: libc::c_float, aScaleToRes: bool) -> nsresult,

    /* [noscript,nostdcall,notxpcom] void AddState (in nsCString aKey, in nsPresStatePtr aState); */
    /// Unable to call function as its signature contains a non-rust type
    pub AddState: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsPresStatePtr GetState (in nsCString aKey); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetState: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void RemoveState (in nsCString aKey); */
    /// Unable to call function as its signature contains a non-rust type
    pub RemoveState: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] boolean HasStates (); */
    pub HasStates: unsafe extern "C" fn (this: *const nsILayoutHistoryState) -> bool,

    /* [noscript,nostdcall,notxpcom] void SetScrollPositionOnly (in constBool aFlag); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetScrollPositionOnly: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void ResetScrollState (); */
    pub ResetScrollState: unsafe extern "C" fn (this: *const nsILayoutHistoryState) -> libc::c_void,

}


impl nsILayoutHistoryState {
    /* readonly attribute boolean hasStates; */
    #[inline]
    pub unsafe fn get_hasStates(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasStates)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getKeys ([optional] out uint32_t aCount, [array, size_is (aCount), retval] out string aKeys); */


    /* void getPresState (in ACString aKey, out float aScrollX, out float aScrollY, out boolean aAllowScrollOriginDowngrade, out float aRes, out boolean aScaleToRes); */
    #[inline]
    pub unsafe fn getPresState(&self, aKey: &[u8]) -> Result<(libc::c_float, libc::c_float, bool, libc::c_float, bool), nsresult> {
        let aKey = nsCString::from(aKey);
        let mut aScrollX: libc::c_float = ::std::mem::zeroed();
        let mut aScrollY: libc::c_float = ::std::mem::zeroed();
        let mut aAllowScrollOriginDowngrade: bool = ::std::mem::zeroed();
        let mut aRes: libc::c_float = ::std::mem::zeroed();
        let mut aScaleToRes: bool = ::std::mem::zeroed();
        match ((*self.vtable).getPresState)(self as *const _, &*aKey, &mut aScrollX as *mut _, &mut aScrollY as *mut _, &mut aAllowScrollOriginDowngrade as *mut _, &mut aRes as *mut _, &mut aScaleToRes as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aScrollX, aScrollY, aAllowScrollOriginDowngrade, aRes, aScaleToRes))
    }

    /* void addNewPresState (in ACString aKey, in float aScrollX, in float aScrollY, in boolean aAllowScrollOriginDowngrade, in float aRes, in boolean aScaleToRes); */
    #[inline]
    pub unsafe fn addNewPresState(&self, aKey: &[u8], aScrollX: libc::c_float, aScrollY: libc::c_float, aAllowScrollOriginDowngrade: bool, aRes: libc::c_float, aScaleToRes: bool) -> Result<(), nsresult> {
        let aKey = nsCString::from(aKey);
        match ((*self.vtable).addNewPresState)(self as *const _, &*aKey, aScrollX, aScrollY, aAllowScrollOriginDowngrade, aRes, aScaleToRes) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,nostdcall,notxpcom] void AddState (in nsCString aKey, in nsPresStatePtr aState); */


    /* [noscript,nostdcall,notxpcom] nsPresStatePtr GetState (in nsCString aKey); */


    /* [noscript,nostdcall,notxpcom] void RemoveState (in nsCString aKey); */


    /* [noscript,nostdcall,notxpcom] boolean HasStates (); */
    #[inline]
    pub unsafe fn HasStates(&self, ) -> bool {

        let _retval = ((*self.vtable).HasStates)(self as *const _, );
        _retval
    }

    /* [noscript,nostdcall,notxpcom] void SetScrollPositionOnly (in constBool aFlag); */


    /* [noscript,nostdcall,notxpcom] void ResetScrollState (); */
    #[inline]
    pub unsafe fn ResetScrollState(&self, ) -> () {

        let _retval = ((*self.vtable).ResetScrollState)(self as *const _, );
        ()
    }

}


