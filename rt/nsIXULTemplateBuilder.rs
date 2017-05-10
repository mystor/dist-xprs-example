//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULTemplateBuilder.idl
//


#[repr(C)]
pub struct nsIXULTemplateBuilder {
    vtable: *const nsIXULTemplateBuilderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULTemplateBuilder {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa583b676, 0x5b02, 0x4f9c,
            [0xa0, 0xc9, 0xcb, 0x85, 0x0c, 0xb9, 0x98, 0x18])
    }
}

unsafe impl RefCounted for nsIXULTemplateBuilder {
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
pub trait nsIXULTemplateBuilderCoerce {
    fn coerce_from(v: &nsIXULTemplateBuilder) -> &Self;
}

impl nsIXULTemplateBuilderCoerce for nsIXULTemplateBuilder {
    #[inline]
    fn coerce_from(v: &nsIXULTemplateBuilder) -> &Self {
        v
    }
}

impl nsIXULTemplateBuilder {
    #[inline]
    pub fn coerce<T: nsIXULTemplateBuilderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULTemplateBuilder {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULTemplateBuilderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULTemplateBuilder) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULTemplateBuilderVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMElement root; */
    pub get_root: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aRoot: *mut *const nsIDOMElement) -> nsresult,

    /* attribute nsISupports datasource; */
    pub get_datasource: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aDatasource: *mut *const nsISupports) -> nsresult,
    pub set_datasource: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aDatasource: *const nsISupports) -> nsresult,

    /* readonly attribute nsIRDFCompositeDataSource database; */
    pub get_database: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aDatabase: *mut *const nsIRDFCompositeDataSource) -> nsresult,

    /* readonly attribute nsIXULTemplateResult rootResult; */
    pub get_rootResult: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aRootResult: *mut *const nsIXULTemplateResult) -> nsresult,

    /* [noscript] readonly attribute nsIXULTemplateQueryProcessor queryProcessor; */
    pub get_queryProcessor: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aQueryProcessor: *mut *const nsIXULTemplateQueryProcessor) -> nsresult,

    /* void rebuild (); */
    pub rebuild: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder) -> nsresult,

    /* void refresh (); */
    pub refresh: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder) -> nsresult,

    /* void addResult (in nsIXULTemplateResult aResult, in nsIDOMNode aQueryNode); */
    pub addResult: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aResult: *const nsIXULTemplateResult, aQueryNode: *const nsIDOMNode) -> nsresult,

    /* void removeResult (in nsIXULTemplateResult aResult); */
    pub removeResult: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aResult: *const nsIXULTemplateResult) -> nsresult,

    /* void replaceResult (in nsIXULTemplateResult aOldResult, in nsIXULTemplateResult aNewResult, in nsIDOMNode aQueryNode); */
    pub replaceResult: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aOldResult: *const nsIXULTemplateResult, aNewResult: *const nsIXULTemplateResult, aQueryNode: *const nsIDOMNode) -> nsresult,

    /* void resultBindingChanged (in nsIXULTemplateResult aResult); */
    pub resultBindingChanged: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aResult: *const nsIXULTemplateResult) -> nsresult,

    /* nsIXULTemplateResult getResultForId (in AString aId); */
    pub getResultForId: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aId: *const nsAString, _retval: *mut *const nsIXULTemplateResult) -> nsresult,

    /* nsIXULTemplateResult getResultForContent (in nsIDOMElement aElement); */
    pub getResultForContent: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aElement: *const nsIDOMElement, _retval: *mut *const nsIXULTemplateResult) -> nsresult,

    /* boolean hasGeneratedContent (in nsIRDFResource aNode, in nsIAtom aTag); */
    pub hasGeneratedContent: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aNode: *const nsIRDFResource, aTag: *const nsIAtom, _retval: *mut bool) -> nsresult,

    /* void addRuleFilter (in nsIDOMNode aRule, in nsIXULTemplateRuleFilter aFilter); */
    pub addRuleFilter: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aRule: *const nsIDOMNode, aFilter: *const nsIXULTemplateRuleFilter) -> nsresult,

    /* [noscript] void init (in nsIContent aElement); */
    pub init: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aElement: *const nsIContent) -> nsresult,

    /* [noscript] void createContents (in nsIContent aElement, in boolean aForceCreation); */
    pub createContents: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aElement: *const nsIContent, aForceCreation: bool) -> nsresult,

    /* void addListener (in nsIXULBuilderListener aListener); */
    pub addListener: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aListener: *const nsIXULBuilderListener) -> nsresult,

    /* void removeListener (in nsIXULBuilderListener aListener); */
    pub removeListener: unsafe extern "C" fn (this: *const nsIXULTemplateBuilder, aListener: *const nsIXULBuilderListener) -> nsresult,

}


impl nsIXULTemplateBuilder {
    /* readonly attribute nsIDOMElement root; */
    #[inline]
    pub unsafe fn get_root(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_root)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsISupports datasource; */
    #[inline]
    pub unsafe fn get_datasource(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_datasource)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_datasource(&self, aDatasource: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_datasource)(self as *const _, aDatasource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIRDFCompositeDataSource database; */
    #[inline]
    pub unsafe fn get_database(&self, ) -> Result<Option<RefPtr<nsIRDFCompositeDataSource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_database)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIXULTemplateResult rootResult; */
    #[inline]
    pub unsafe fn get_rootResult(&self, ) -> Result<Option<RefPtr<nsIXULTemplateResult>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rootResult)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] readonly attribute nsIXULTemplateQueryProcessor queryProcessor; */
    #[inline]
    pub unsafe fn get_queryProcessor(&self, ) -> Result<Option<RefPtr<nsIXULTemplateQueryProcessor>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_queryProcessor)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void rebuild (); */
    #[inline]
    pub unsafe fn rebuild(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).rebuild)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void refresh (); */
    #[inline]
    pub unsafe fn refresh(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).refresh)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addResult (in nsIXULTemplateResult aResult, in nsIDOMNode aQueryNode); */
    #[inline]
    pub unsafe fn addResult(&self, aResult: Option<&nsIXULTemplateResult>, aQueryNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).addResult)(self as *const _, aResult.map_or(::std::ptr::null(), |x| x as *const _), aQueryNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeResult (in nsIXULTemplateResult aResult); */
    #[inline]
    pub unsafe fn removeResult(&self, aResult: Option<&nsIXULTemplateResult>) -> Result<(), nsresult> {

        match ((*self.vtable).removeResult)(self as *const _, aResult.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void replaceResult (in nsIXULTemplateResult aOldResult, in nsIXULTemplateResult aNewResult, in nsIDOMNode aQueryNode); */
    #[inline]
    pub unsafe fn replaceResult(&self, aOldResult: Option<&nsIXULTemplateResult>, aNewResult: Option<&nsIXULTemplateResult>, aQueryNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).replaceResult)(self as *const _, aOldResult.map_or(::std::ptr::null(), |x| x as *const _), aNewResult.map_or(::std::ptr::null(), |x| x as *const _), aQueryNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resultBindingChanged (in nsIXULTemplateResult aResult); */
    #[inline]
    pub unsafe fn resultBindingChanged(&self, aResult: Option<&nsIXULTemplateResult>) -> Result<(), nsresult> {

        match ((*self.vtable).resultBindingChanged)(self as *const _, aResult.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIXULTemplateResult getResultForId (in AString aId); */
    #[inline]
    pub unsafe fn getResultForId(&self, aId: &[u16]) -> Result<Option<RefPtr<nsIXULTemplateResult>>, nsresult> {
        let aId = nsString::from(aId);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getResultForId)(self as *const _, &*aId, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIXULTemplateResult getResultForContent (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn getResultForContent(&self, aElement: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsIXULTemplateResult>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getResultForContent)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean hasGeneratedContent (in nsIRDFResource aNode, in nsIAtom aTag); */
    #[inline]
    pub unsafe fn hasGeneratedContent(&self, aNode: Option<&nsIRDFResource>, aTag: Option<&nsIAtom>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasGeneratedContent)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aTag.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addRuleFilter (in nsIDOMNode aRule, in nsIXULTemplateRuleFilter aFilter); */
    #[inline]
    pub unsafe fn addRuleFilter(&self, aRule: Option<&nsIDOMNode>, aFilter: Option<&nsIXULTemplateRuleFilter>) -> Result<(), nsresult> {

        match ((*self.vtable).addRuleFilter)(self as *const _, aRule.map_or(::std::ptr::null(), |x| x as *const _), aFilter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void init (in nsIContent aElement); */
    #[inline]
    pub unsafe fn init(&self, aElement: Option<&nsIContent>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void createContents (in nsIContent aElement, in boolean aForceCreation); */
    #[inline]
    pub unsafe fn createContents(&self, aElement: Option<&nsIContent>, aForceCreation: bool) -> Result<(), nsresult> {

        match ((*self.vtable).createContents)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aForceCreation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addListener (in nsIXULBuilderListener aListener); */
    #[inline]
    pub unsafe fn addListener(&self, aListener: Option<&nsIXULBuilderListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeListener (in nsIXULBuilderListener aListener); */
    #[inline]
    pub unsafe fn removeListener(&self, aListener: Option<&nsIXULBuilderListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIXULTreeBuilderObserver_consts {
    pub const DROP_BEFORE: i64 = -1;
    pub const DROP_ON: i64 = 0;
    pub const DROP_AFTER: i64 = 1;
}


#[repr(C)]
pub struct nsIXULTreeBuilderObserver {
    vtable: *const nsIXULTreeBuilderObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULTreeBuilderObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x57ced9a7, 0xec0b, 0x4a0e,
            [0x8a, 0xeb, 0x5d, 0xa3, 0x2e, 0xbe, 0x95, 0x1c])
    }
}

unsafe impl RefCounted for nsIXULTreeBuilderObserver {
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
pub trait nsIXULTreeBuilderObserverCoerce {
    fn coerce_from(v: &nsIXULTreeBuilderObserver) -> &Self;
}

impl nsIXULTreeBuilderObserverCoerce for nsIXULTreeBuilderObserver {
    #[inline]
    fn coerce_from(v: &nsIXULTreeBuilderObserver) -> &Self {
        v
    }
}

impl nsIXULTreeBuilderObserver {
    #[inline]
    pub fn coerce<T: nsIXULTreeBuilderObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULTreeBuilderObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULTreeBuilderObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULTreeBuilderObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULTreeBuilderObserverVTable {
    pub __base: nsISupportsVTable,

    /* boolean canDrop (in long index, in long orientation, in nsIDOMDataTransfer dataTransfer); */
    pub canDrop: unsafe extern "C" fn (this: *const nsIXULTreeBuilderObserver, index: libc::int32_t, orientation: libc::int32_t, dataTransfer: *const nsIDOMDataTransfer, _retval: *mut bool) -> nsresult,

    /* void onDrop (in long row, in long orientation, in nsIDOMDataTransfer dataTransfer); */
    pub onDrop: unsafe extern "C" fn (this: *const nsIXULTreeBuilderObserver, row: libc::int32_t, orientation: libc::int32_t, dataTransfer: *const nsIDOMDataTransfer) -> nsresult,

    /* void onToggleOpenState (in long index); */
    pub onToggleOpenState: unsafe extern "C" fn (this: *const nsIXULTreeBuilderObserver, index: libc::int32_t) -> nsresult,

    /* void onCycleHeader (in wstring colID, in nsIDOMElement elt); */
    pub onCycleHeader: unsafe extern "C" fn (this: *const nsIXULTreeBuilderObserver, colID: *const libc::int16_t, elt: *const nsIDOMElement) -> nsresult,

    /* void onCycleCell (in long row, in wstring colID); */
    pub onCycleCell: unsafe extern "C" fn (this: *const nsIXULTreeBuilderObserver, row: libc::int32_t, colID: *const libc::int16_t) -> nsresult,

    /* void onSelectionChanged (); */
    pub onSelectionChanged: unsafe extern "C" fn (this: *const nsIXULTreeBuilderObserver) -> nsresult,

    /* void onPerformAction (in wstring action); */
    pub onPerformAction: unsafe extern "C" fn (this: *const nsIXULTreeBuilderObserver, action: *const libc::int16_t) -> nsresult,

    /* void onPerformActionOnRow (in wstring action, in long row); */
    pub onPerformActionOnRow: unsafe extern "C" fn (this: *const nsIXULTreeBuilderObserver, action: *const libc::int16_t, row: libc::int32_t) -> nsresult,

    /* void onPerformActionOnCell (in wstring action, in long row, in wstring colID); */
    pub onPerformActionOnCell: unsafe extern "C" fn (this: *const nsIXULTreeBuilderObserver, action: *const libc::int16_t, row: libc::int32_t, colID: *const libc::int16_t) -> nsresult,

}


impl nsIXULTreeBuilderObserver {
    /* boolean canDrop (in long index, in long orientation, in nsIDOMDataTransfer dataTransfer); */
    #[inline]
    pub unsafe fn canDrop(&self, index: libc::int32_t, orientation: libc::int32_t, dataTransfer: Option<&nsIDOMDataTransfer>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canDrop)(self as *const _, index, orientation, dataTransfer.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void onDrop (in long row, in long orientation, in nsIDOMDataTransfer dataTransfer); */
    #[inline]
    pub unsafe fn onDrop(&self, row: libc::int32_t, orientation: libc::int32_t, dataTransfer: Option<&nsIDOMDataTransfer>) -> Result<(), nsresult> {

        match ((*self.vtable).onDrop)(self as *const _, row, orientation, dataTransfer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onToggleOpenState (in long index); */
    #[inline]
    pub unsafe fn onToggleOpenState(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onToggleOpenState)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onCycleHeader (in wstring colID, in nsIDOMElement elt); */
    #[inline]
    pub unsafe fn onCycleHeader(&self, colID: *const libc::int16_t, elt: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).onCycleHeader)(self as *const _, colID, elt.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onCycleCell (in long row, in wstring colID); */
    #[inline]
    pub unsafe fn onCycleCell(&self, row: libc::int32_t, colID: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).onCycleCell)(self as *const _, row, colID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSelectionChanged (); */
    #[inline]
    pub unsafe fn onSelectionChanged(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onSelectionChanged)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onPerformAction (in wstring action); */
    #[inline]
    pub unsafe fn onPerformAction(&self, action: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).onPerformAction)(self as *const _, action) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onPerformActionOnRow (in wstring action, in long row); */
    #[inline]
    pub unsafe fn onPerformActionOnRow(&self, action: *const libc::int16_t, row: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onPerformActionOnRow)(self as *const _, action, row) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onPerformActionOnCell (in wstring action, in long row, in wstring colID); */
    #[inline]
    pub unsafe fn onPerformActionOnCell(&self, action: *const libc::int16_t, row: libc::int32_t, colID: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).onPerformActionOnCell)(self as *const _, action, row, colID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIXULTreeBuilder {
    vtable: *const nsIXULTreeBuilderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULTreeBuilder {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x06b31b15, 0xebf5, 0x4e74,
            [0xa0, 0xe2, 0x6b, 0xc0, 0xa1, 0x8a, 0x39, 0x69])
    }
}

unsafe impl RefCounted for nsIXULTreeBuilder {
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
pub trait nsIXULTreeBuilderCoerce {
    fn coerce_from(v: &nsIXULTreeBuilder) -> &Self;
}

impl nsIXULTreeBuilderCoerce for nsIXULTreeBuilder {
    #[inline]
    fn coerce_from(v: &nsIXULTreeBuilder) -> &Self {
        v
    }
}

impl nsIXULTreeBuilder {
    #[inline]
    pub fn coerce<T: nsIXULTreeBuilderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULTreeBuilder {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULTreeBuilderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULTreeBuilder) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULTreeBuilderVTable {
    pub __base: nsISupportsVTable,

    /* nsIRDFResource getResourceAtIndex (in long aRowIndex); */
    pub getResourceAtIndex: unsafe extern "C" fn (this: *const nsIXULTreeBuilder, aRowIndex: libc::int32_t, _retval: *mut *const nsIRDFResource) -> nsresult,

    /* long getIndexOfResource (in nsIRDFResource resource); */
    pub getIndexOfResource: unsafe extern "C" fn (this: *const nsIXULTreeBuilder, resource: *const nsIRDFResource, _retval: *mut libc::int32_t) -> nsresult,

    /* void addObserver (in nsIXULTreeBuilderObserver aObserver); */
    pub addObserver: unsafe extern "C" fn (this: *const nsIXULTreeBuilder, aObserver: *const nsIXULTreeBuilderObserver) -> nsresult,

    /* void removeObserver (in nsIXULTreeBuilderObserver aObserver); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsIXULTreeBuilder, aObserver: *const nsIXULTreeBuilderObserver) -> nsresult,

    /* void sort (in nsIDOMElement aColumnElement); */
    pub sort: unsafe extern "C" fn (this: *const nsIXULTreeBuilder, aColumnElement: *const nsIDOMElement) -> nsresult,

}


impl nsIXULTreeBuilder {
    /* nsIRDFResource getResourceAtIndex (in long aRowIndex); */
    #[inline]
    pub unsafe fn getResourceAtIndex(&self, aRowIndex: libc::int32_t) -> Result<Option<RefPtr<nsIRDFResource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getResourceAtIndex)(self as *const _, aRowIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* long getIndexOfResource (in nsIRDFResource resource); */
    #[inline]
    pub unsafe fn getIndexOfResource(&self, resource: Option<&nsIRDFResource>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getIndexOfResource)(self as *const _, resource.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addObserver (in nsIXULTreeBuilderObserver aObserver); */
    #[inline]
    pub unsafe fn addObserver(&self, aObserver: Option<&nsIXULTreeBuilderObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in nsIXULTreeBuilderObserver aObserver); */
    #[inline]
    pub unsafe fn removeObserver(&self, aObserver: Option<&nsIXULTreeBuilderObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sort (in nsIDOMElement aColumnElement); */
    #[inline]
    pub unsafe fn sort(&self, aColumnElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).sort)(self as *const _, aColumnElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


