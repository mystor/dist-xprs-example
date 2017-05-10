//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransactionListener.idl
//


#[repr(C)]
pub struct nsITransactionListener {
    vtable: *const nsITransactionListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITransactionListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x58e330c4, 0x7b48, 0x11d2,
            [0x98, 0xb9, 0x00, 0x80, 0x5f, 0x29, 0x7d, 0x89])
    }
}

unsafe impl RefCounted for nsITransactionListener {
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
pub trait nsITransactionListenerCoerce {
    fn coerce_from(v: &nsITransactionListener) -> &Self;
}

impl nsITransactionListenerCoerce for nsITransactionListener {
    #[inline]
    fn coerce_from(v: &nsITransactionListener) -> &Self {
        v
    }
}

impl nsITransactionListener {
    #[inline]
    pub fn coerce<T: nsITransactionListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITransactionListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITransactionListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransactionListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITransactionListenerVTable {
    pub __base: nsISupportsVTable,

    /* boolean willDo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
    pub willDo: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, _retval: *mut bool) -> nsresult,

    /* void didDo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aDoResult); */
    pub didDo: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, aDoResult: nsresult) -> nsresult,

    /* boolean willUndo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
    pub willUndo: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, _retval: *mut bool) -> nsresult,

    /* void didUndo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aUndoResult); */
    pub didUndo: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, aUndoResult: nsresult) -> nsresult,

    /* boolean willRedo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
    pub willRedo: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, _retval: *mut bool) -> nsresult,

    /* void didRedo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aRedoResult); */
    pub didRedo: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, aRedoResult: nsresult) -> nsresult,

    /* boolean willBeginBatch (in nsITransactionManager aManager); */
    pub willBeginBatch: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, _retval: *mut bool) -> nsresult,

    /* void didBeginBatch (in nsITransactionManager aManager, in nsresult aResult); */
    pub didBeginBatch: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aResult: nsresult) -> nsresult,

    /* boolean willEndBatch (in nsITransactionManager aManager); */
    pub willEndBatch: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, _retval: *mut bool) -> nsresult,

    /* void didEndBatch (in nsITransactionManager aManager, in nsresult aResult); */
    pub didEndBatch: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aResult: nsresult) -> nsresult,

    /* boolean willMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge); */
    pub willMerge: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTopTransaction: *const nsITransaction, aTransactionToMerge: *const nsITransaction, _retval: *mut bool) -> nsresult,

    /* void didMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge, in boolean aDidMerge, in nsresult aMergeResult); */
    pub didMerge: unsafe extern "C" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTopTransaction: *const nsITransaction, aTransactionToMerge: *const nsITransaction, aDidMerge: bool, aMergeResult: nsresult) -> nsresult,

}


impl nsITransactionListener {
    /* boolean willDo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
    #[inline]
    pub unsafe fn willDo(&self, aManager: Option<&nsITransactionManager>, aTransaction: Option<&nsITransaction>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).willDo)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), aTransaction.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void didDo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aDoResult); */
    #[inline]
    pub unsafe fn didDo(&self, aManager: Option<&nsITransactionManager>, aTransaction: Option<&nsITransaction>, aDoResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).didDo)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), aTransaction.map_or(::std::ptr::null(), |x| x as *const _), aDoResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean willUndo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
    #[inline]
    pub unsafe fn willUndo(&self, aManager: Option<&nsITransactionManager>, aTransaction: Option<&nsITransaction>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).willUndo)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), aTransaction.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void didUndo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aUndoResult); */
    #[inline]
    pub unsafe fn didUndo(&self, aManager: Option<&nsITransactionManager>, aTransaction: Option<&nsITransaction>, aUndoResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).didUndo)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), aTransaction.map_or(::std::ptr::null(), |x| x as *const _), aUndoResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean willRedo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
    #[inline]
    pub unsafe fn willRedo(&self, aManager: Option<&nsITransactionManager>, aTransaction: Option<&nsITransaction>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).willRedo)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), aTransaction.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void didRedo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aRedoResult); */
    #[inline]
    pub unsafe fn didRedo(&self, aManager: Option<&nsITransactionManager>, aTransaction: Option<&nsITransaction>, aRedoResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).didRedo)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), aTransaction.map_or(::std::ptr::null(), |x| x as *const _), aRedoResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean willBeginBatch (in nsITransactionManager aManager); */
    #[inline]
    pub unsafe fn willBeginBatch(&self, aManager: Option<&nsITransactionManager>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).willBeginBatch)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void didBeginBatch (in nsITransactionManager aManager, in nsresult aResult); */
    #[inline]
    pub unsafe fn didBeginBatch(&self, aManager: Option<&nsITransactionManager>, aResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).didBeginBatch)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean willEndBatch (in nsITransactionManager aManager); */
    #[inline]
    pub unsafe fn willEndBatch(&self, aManager: Option<&nsITransactionManager>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).willEndBatch)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void didEndBatch (in nsITransactionManager aManager, in nsresult aResult); */
    #[inline]
    pub unsafe fn didEndBatch(&self, aManager: Option<&nsITransactionManager>, aResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).didEndBatch)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean willMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge); */
    #[inline]
    pub unsafe fn willMerge(&self, aManager: Option<&nsITransactionManager>, aTopTransaction: Option<&nsITransaction>, aTransactionToMerge: Option<&nsITransaction>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).willMerge)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), aTopTransaction.map_or(::std::ptr::null(), |x| x as *const _), aTransactionToMerge.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void didMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge, in boolean aDidMerge, in nsresult aMergeResult); */
    #[inline]
    pub unsafe fn didMerge(&self, aManager: Option<&nsITransactionManager>, aTopTransaction: Option<&nsITransaction>, aTransactionToMerge: Option<&nsITransaction>, aDidMerge: bool, aMergeResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).didMerge)(self as *const _, aManager.map_or(::std::ptr::null(), |x| x as *const _), aTopTransaction.map_or(::std::ptr::null(), |x| x as *const _), aTransactionToMerge.map_or(::std::ptr::null(), |x| x as *const _), aDidMerge, aMergeResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


