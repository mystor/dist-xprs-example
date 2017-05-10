//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintProgress.idl
//


#[repr(C)]
pub struct nsIPrintProgress {
    vtable: *const nsIPrintProgressVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrintProgress {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x05f4fb88, 0xe568, 0x4d35,
            [0xb3, 0x94, 0xce, 0x0a, 0xa3, 0xee, 0xa6, 0xfc])
    }
}

unsafe impl RefCounted for nsIPrintProgress {
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
pub trait nsIPrintProgressCoerce {
    fn coerce_from(v: &nsIPrintProgress) -> &Self;
}

impl nsIPrintProgressCoerce for nsIPrintProgress {
    #[inline]
    fn coerce_from(v: &nsIPrintProgress) -> &Self {
        v
    }
}

impl nsIPrintProgress {
    #[inline]
    pub fn coerce<T: nsIPrintProgressCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrintProgress {
    type Target = nsIWebProgressListener;
    #[inline]
    fn deref(&self) -> &nsIWebProgressListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIWebProgressListenerCoerce> nsIPrintProgressCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintProgress) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrintProgressVTable {
    pub __base: nsIWebProgressListenerVTable,

    /* void openProgressDialog (in mozIDOMWindowProxy parent, in string dialogURL, in nsISupports parameters, in nsIObserver openDialogObserver, out boolean notifyOnOpen); */
    pub openProgressDialog: unsafe extern "C" fn (this: *const nsIPrintProgress, parent: *const mozIDOMWindowProxy, dialogURL: *const libc::c_char, parameters: *const nsISupports, openDialogObserver: *const nsIObserver, notifyOnOpen: *mut bool) -> nsresult,

    /* void closeProgressDialog (in boolean forceClose); */
    pub closeProgressDialog: unsafe extern "C" fn (this: *const nsIPrintProgress, forceClose: bool) -> nsresult,

    /* void registerListener (in nsIWebProgressListener listener); */
    pub registerListener: unsafe extern "C" fn (this: *const nsIPrintProgress, listener: *const nsIWebProgressListener) -> nsresult,

    /* void unregisterListener (in nsIWebProgressListener listener); */
    pub unregisterListener: unsafe extern "C" fn (this: *const nsIPrintProgress, listener: *const nsIWebProgressListener) -> nsresult,

    /* void doneIniting (); */
    pub doneIniting: unsafe extern "C" fn (this: *const nsIPrintProgress) -> nsresult,

    /* nsIPrompt getPrompter (); */
    pub getPrompter: unsafe extern "C" fn (this: *const nsIPrintProgress, _retval: *mut *const nsIPrompt) -> nsresult,

    /* attribute boolean processCanceledByUser; */
    pub get_processCanceledByUser: unsafe extern "C" fn (this: *const nsIPrintProgress, aProcessCanceledByUser: *mut bool) -> nsresult,
    pub set_processCanceledByUser: unsafe extern "C" fn (this: *const nsIPrintProgress, aProcessCanceledByUser: bool) -> nsresult,

}


impl nsIPrintProgress {
    /* void openProgressDialog (in mozIDOMWindowProxy parent, in string dialogURL, in nsISupports parameters, in nsIObserver openDialogObserver, out boolean notifyOnOpen); */
    #[inline]
    pub unsafe fn openProgressDialog(&self, parent: Option<&mozIDOMWindowProxy>, dialogURL: *const libc::c_char, parameters: Option<&nsISupports>, openDialogObserver: Option<&nsIObserver>) -> Result<bool, nsresult> {
        let mut notifyOnOpen: bool = ::std::mem::zeroed();
        match ((*self.vtable).openProgressDialog)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), dialogURL, parameters.map_or(::std::ptr::null(), |x| x as *const _), openDialogObserver.map_or(::std::ptr::null(), |x| x as *const _), &mut notifyOnOpen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(notifyOnOpen)
    }

    /* void closeProgressDialog (in boolean forceClose); */
    #[inline]
    pub unsafe fn closeProgressDialog(&self, forceClose: bool) -> Result<(), nsresult> {

        match ((*self.vtable).closeProgressDialog)(self as *const _, forceClose) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerListener (in nsIWebProgressListener listener); */
    #[inline]
    pub unsafe fn registerListener(&self, listener: Option<&nsIWebProgressListener>) -> Result<(), nsresult> {

        match ((*self.vtable).registerListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterListener (in nsIWebProgressListener listener); */
    #[inline]
    pub unsafe fn unregisterListener(&self, listener: Option<&nsIWebProgressListener>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void doneIniting (); */
    #[inline]
    pub unsafe fn doneIniting(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).doneIniting)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIPrompt getPrompter (); */
    #[inline]
    pub unsafe fn getPrompter(&self, ) -> Result<Option<RefPtr<nsIPrompt>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPrompter)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean processCanceledByUser; */
    #[inline]
    pub unsafe fn get_processCanceledByUser(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_processCanceledByUser)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_processCanceledByUser(&self, aProcessCanceledByUser: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_processCanceledByUser)(self as *const _, aProcessCanceledByUser) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


