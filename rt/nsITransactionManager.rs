//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransactionManager.idl
//


#[repr(C)]
pub struct nsITransactionManager {
    vtable: *const nsITransactionManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITransactionManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc77763df, 0x0fb9, 0x41a8,
            [0x80, 0x74, 0x8e, 0x88, 0x2f, 0x60, 0x57, 0x55])
    }
}

unsafe impl RefCounted for nsITransactionManager {
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
pub trait nsITransactionManagerCoerce {
    fn coerce_from(v: &nsITransactionManager) -> &Self;
}

impl nsITransactionManagerCoerce for nsITransactionManager {
    #[inline]
    fn coerce_from(v: &nsITransactionManager) -> &Self {
        v
    }
}

impl nsITransactionManager {
    #[inline]
    pub fn coerce<T: nsITransactionManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITransactionManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITransactionManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransactionManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITransactionManagerVTable {
    pub __base: nsISupportsVTable,

    /* void doTransaction (in nsITransaction aTransaction); */
    pub doTransaction: unsafe extern "C" fn (this: *const nsITransactionManager, aTransaction: *const nsITransaction) -> nsresult,

    /* void undoTransaction (); */
    pub undoTransaction: unsafe extern "C" fn (this: *const nsITransactionManager) -> nsresult,

    /* void redoTransaction (); */
    pub redoTransaction: unsafe extern "C" fn (this: *const nsITransactionManager) -> nsresult,

    /* void clear (); */
    pub clear: unsafe extern "C" fn (this: *const nsITransactionManager) -> nsresult,

    /* void clearUndoStack (); */
    pub clearUndoStack: unsafe extern "C" fn (this: *const nsITransactionManager) -> nsresult,

    /* void clearRedoStack (); */
    pub clearRedoStack: unsafe extern "C" fn (this: *const nsITransactionManager) -> nsresult,

    /* void beginBatch (in nsISupports aData); */
    pub beginBatch: unsafe extern "C" fn (this: *const nsITransactionManager, aData: *const nsISupports) -> nsresult,

    /* void endBatch (in boolean aAllowEmpty); */
    pub endBatch: unsafe extern "C" fn (this: *const nsITransactionManager, aAllowEmpty: bool) -> nsresult,

    /* readonly attribute long numberOfUndoItems; */
    pub get_numberOfUndoItems: unsafe extern "C" fn (this: *const nsITransactionManager, aNumberOfUndoItems: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long numberOfRedoItems; */
    pub get_numberOfRedoItems: unsafe extern "C" fn (this: *const nsITransactionManager, aNumberOfRedoItems: *mut libc::int32_t) -> nsresult,

    /* attribute long maxTransactionCount; */
    pub get_maxTransactionCount: unsafe extern "C" fn (this: *const nsITransactionManager, aMaxTransactionCount: *mut libc::int32_t) -> nsresult,
    pub set_maxTransactionCount: unsafe extern "C" fn (this: *const nsITransactionManager, aMaxTransactionCount: libc::int32_t) -> nsresult,

    /* void batchTopUndo (); */
    pub batchTopUndo: unsafe extern "C" fn (this: *const nsITransactionManager) -> nsresult,

    /* void removeTopUndo (); */
    pub removeTopUndo: unsafe extern "C" fn (this: *const nsITransactionManager) -> nsresult,

    /* nsITransaction peekUndoStack (); */
    pub peekUndoStack: unsafe extern "C" fn (this: *const nsITransactionManager, _retval: *mut *const nsITransaction) -> nsresult,

    /* nsITransaction peekRedoStack (); */
    pub peekRedoStack: unsafe extern "C" fn (this: *const nsITransactionManager, _retval: *mut *const nsITransaction) -> nsresult,

    /* nsITransactionList getUndoList (); */
    pub getUndoList: unsafe extern "C" fn (this: *const nsITransactionManager, _retval: *mut *const nsITransactionList) -> nsresult,

    /* nsITransactionList getRedoList (); */
    pub getRedoList: unsafe extern "C" fn (this: *const nsITransactionManager, _retval: *mut *const nsITransactionList) -> nsresult,

    /* void AddListener (in nsITransactionListener aListener); */
    pub AddListener: unsafe extern "C" fn (this: *const nsITransactionManager, aListener: *const nsITransactionListener) -> nsresult,

    /* void RemoveListener (in nsITransactionListener aListener); */
    pub RemoveListener: unsafe extern "C" fn (this: *const nsITransactionManager, aListener: *const nsITransactionListener) -> nsresult,

}


impl nsITransactionManager {
    /* void doTransaction (in nsITransaction aTransaction); */
    #[inline]
    pub unsafe fn doTransaction(&self, aTransaction: Option<&nsITransaction>) -> Result<(), nsresult> {

        match ((*self.vtable).doTransaction)(self as *const _, aTransaction.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void undoTransaction (); */
    #[inline]
    pub unsafe fn undoTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).undoTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void redoTransaction (); */
    #[inline]
    pub unsafe fn redoTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).redoTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clear (); */
    #[inline]
    pub unsafe fn clear(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clear)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearUndoStack (); */
    #[inline]
    pub unsafe fn clearUndoStack(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearUndoStack)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearRedoStack (); */
    #[inline]
    pub unsafe fn clearRedoStack(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearRedoStack)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beginBatch (in nsISupports aData); */
    #[inline]
    pub unsafe fn beginBatch(&self, aData: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).beginBatch)(self as *const _, aData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endBatch (in boolean aAllowEmpty); */
    #[inline]
    pub unsafe fn endBatch(&self, aAllowEmpty: bool) -> Result<(), nsresult> {

        match ((*self.vtable).endBatch)(self as *const _, aAllowEmpty) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long numberOfUndoItems; */
    #[inline]
    pub unsafe fn get_numberOfUndoItems(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numberOfUndoItems)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long numberOfRedoItems; */
    #[inline]
    pub unsafe fn get_numberOfRedoItems(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numberOfRedoItems)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute long maxTransactionCount; */
    #[inline]
    pub unsafe fn get_maxTransactionCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_maxTransactionCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_maxTransactionCount(&self, aMaxTransactionCount: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_maxTransactionCount)(self as *const _, aMaxTransactionCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void batchTopUndo (); */
    #[inline]
    pub unsafe fn batchTopUndo(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).batchTopUndo)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeTopUndo (); */
    #[inline]
    pub unsafe fn removeTopUndo(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).removeTopUndo)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsITransaction peekUndoStack (); */
    #[inline]
    pub unsafe fn peekUndoStack(&self, ) -> Result<Option<RefPtr<nsITransaction>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).peekUndoStack)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITransaction peekRedoStack (); */
    #[inline]
    pub unsafe fn peekRedoStack(&self, ) -> Result<Option<RefPtr<nsITransaction>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).peekRedoStack)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITransactionList getUndoList (); */
    #[inline]
    pub unsafe fn getUndoList(&self, ) -> Result<Option<RefPtr<nsITransactionList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getUndoList)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITransactionList getRedoList (); */
    #[inline]
    pub unsafe fn getRedoList(&self, ) -> Result<Option<RefPtr<nsITransactionList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRedoList)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void AddListener (in nsITransactionListener aListener); */
    #[inline]
    pub unsafe fn AddListener(&self, aListener: Option<&nsITransactionListener>) -> Result<(), nsresult> {

        match ((*self.vtable).AddListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void RemoveListener (in nsITransactionListener aListener); */
    #[inline]
    pub unsafe fn RemoveListener(&self, aListener: Option<&nsITransactionListener>) -> Result<(), nsresult> {

        match ((*self.vtable).RemoveListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


