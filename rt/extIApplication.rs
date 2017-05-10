//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/extIApplication.idl
//


#[repr(C)]
pub struct extIConsole {
    vtable: *const extIConsoleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extIConsole {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xae8482e0, 0xaa5a, 0x11db,
            [0xab, 0xbd, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for extIConsole {
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
pub trait extIConsoleCoerce {
    fn coerce_from(v: &extIConsole) -> &Self;
}

impl extIConsoleCoerce for extIConsole {
    #[inline]
    fn coerce_from(v: &extIConsole) -> &Self {
        v
    }
}

impl extIConsole {
    #[inline]
    pub fn coerce<T: extIConsoleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extIConsole {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extIConsoleCoerce for T {
    #[inline]
    fn coerce_from(v: &extIConsole) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extIConsoleVTable {
    pub __base: nsISupportsVTable,

    /* void log (in AString aMsg); */
    pub log: unsafe extern "C" fn (this: *const extIConsole, aMsg: *const nsAString) -> nsresult,

    /* void open (); */
    pub open: unsafe extern "C" fn (this: *const extIConsole) -> nsresult,

}


impl extIConsole {
    /* void log (in AString aMsg); */
    #[inline]
    pub unsafe fn log(&self, aMsg: &[u16]) -> Result<(), nsresult> {
        let aMsg = nsString::from(aMsg);
        match ((*self.vtable).log)(self as *const _, &*aMsg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void open (); */
    #[inline]
    pub unsafe fn open(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).open)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct extIEventItem {
    vtable: *const extIEventItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extIEventItem {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x05281820, 0xab62, 0x11db,
            [0xab, 0xbd, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for extIEventItem {
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
pub trait extIEventItemCoerce {
    fn coerce_from(v: &extIEventItem) -> &Self;
}

impl extIEventItemCoerce for extIEventItem {
    #[inline]
    fn coerce_from(v: &extIEventItem) -> &Self {
        v
    }
}

impl extIEventItem {
    #[inline]
    pub fn coerce<T: extIEventItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extIEventItem {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extIEventItemCoerce for T {
    #[inline]
    fn coerce_from(v: &extIEventItem) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extIEventItemVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString type; */
    pub get_type_: unsafe extern "C" fn (this: *const extIEventItem, aType: *mut nsAString) -> nsresult,

    /* readonly attribute nsIVariant data; */
    pub get_data: unsafe extern "C" fn (this: *const extIEventItem, aData: *mut *const nsIVariant) -> nsresult,

    /* void preventDefault (); */
    pub preventDefault: unsafe extern "C" fn (this: *const extIEventItem) -> nsresult,

}


impl extIEventItem {
    /* readonly attribute AString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIVariant data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_data)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void preventDefault (); */
    #[inline]
    pub unsafe fn preventDefault(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).preventDefault)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct extIEventListener {
    vtable: *const extIEventListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extIEventListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2dfe3a50, 0xab2f, 0x11db,
            [0xab, 0xbd, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for extIEventListener {
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
pub trait extIEventListenerCoerce {
    fn coerce_from(v: &extIEventListener) -> &Self;
}

impl extIEventListenerCoerce for extIEventListener {
    #[inline]
    fn coerce_from(v: &extIEventListener) -> &Self {
        v
    }
}

impl extIEventListener {
    #[inline]
    pub fn coerce<T: extIEventListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extIEventListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extIEventListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &extIEventListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extIEventListenerVTable {
    pub __base: nsISupportsVTable,

    /* void handleEvent (in extIEventItem aEvent); */
    pub handleEvent: unsafe extern "C" fn (this: *const extIEventListener, aEvent: *const extIEventItem) -> nsresult,

}


impl extIEventListener {
    /* void handleEvent (in extIEventItem aEvent); */
    #[inline]
    pub unsafe fn handleEvent(&self, aEvent: Option<&extIEventItem>) -> Result<(), nsresult> {

        match ((*self.vtable).handleEvent)(self as *const _, aEvent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct extIEvents {
    vtable: *const extIEventsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extIEvents {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3a8ec9d0, 0xab19, 0x11db,
            [0xab, 0xbd, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for extIEvents {
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
pub trait extIEventsCoerce {
    fn coerce_from(v: &extIEvents) -> &Self;
}

impl extIEventsCoerce for extIEvents {
    #[inline]
    fn coerce_from(v: &extIEvents) -> &Self {
        v
    }
}

impl extIEvents {
    #[inline]
    pub fn coerce<T: extIEventsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extIEvents {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extIEventsCoerce for T {
    #[inline]
    fn coerce_from(v: &extIEvents) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extIEventsVTable {
    pub __base: nsISupportsVTable,

    /* void addListener (in AString aEvent, in extIEventListener aListener); */
    pub addListener: unsafe extern "C" fn (this: *const extIEvents, aEvent: *const nsAString, aListener: *const extIEventListener) -> nsresult,

    /* void removeListener (in AString aEvent, in extIEventListener aListener); */
    pub removeListener: unsafe extern "C" fn (this: *const extIEvents, aEvent: *const nsAString, aListener: *const extIEventListener) -> nsresult,

}


impl extIEvents {
    /* void addListener (in AString aEvent, in extIEventListener aListener); */
    #[inline]
    pub unsafe fn addListener(&self, aEvent: &[u16], aListener: Option<&extIEventListener>) -> Result<(), nsresult> {
        let aEvent = nsString::from(aEvent);
        match ((*self.vtable).addListener)(self as *const _, &*aEvent, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeListener (in AString aEvent, in extIEventListener aListener); */
    #[inline]
    pub unsafe fn removeListener(&self, aEvent: &[u16], aListener: Option<&extIEventListener>) -> Result<(), nsresult> {
        let aEvent = nsString::from(aEvent);
        match ((*self.vtable).removeListener)(self as *const _, &*aEvent, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct extIPreferenceBranch {
    vtable: *const extIPreferenceBranchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extIPreferenceBranch {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xce697d40, 0xaa5a, 0x11db,
            [0xab, 0xbd, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for extIPreferenceBranch {
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
pub trait extIPreferenceBranchCoerce {
    fn coerce_from(v: &extIPreferenceBranch) -> &Self;
}

impl extIPreferenceBranchCoerce for extIPreferenceBranch {
    #[inline]
    fn coerce_from(v: &extIPreferenceBranch) -> &Self {
        v
    }
}

impl extIPreferenceBranch {
    #[inline]
    pub fn coerce<T: extIPreferenceBranchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extIPreferenceBranch {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extIPreferenceBranchCoerce for T {
    #[inline]
    fn coerce_from(v: &extIPreferenceBranch) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extIPreferenceBranchVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString root; */
    pub get_root: unsafe extern "C" fn (this: *const extIPreferenceBranch, aRoot: *mut nsAString) -> nsresult,

    /* readonly attribute nsIVariant all; */
    pub get_all: unsafe extern "C" fn (this: *const extIPreferenceBranch, aAll: *mut *const nsIVariant) -> nsresult,

    /* readonly attribute extIEvents events; */
    pub get_events: unsafe extern "C" fn (this: *const extIPreferenceBranch, aEvents: *mut *const extIEvents) -> nsresult,

    /* boolean has (in AString aName); */
    pub has: unsafe extern "C" fn (this: *const extIPreferenceBranch, aName: *const nsAString, _retval: *mut bool) -> nsresult,

    /* extIPreference get (in AString aName); */
    pub get: unsafe extern "C" fn (this: *const extIPreferenceBranch, aName: *const nsAString, _retval: *mut *const extIPreference) -> nsresult,

    /* nsIVariant getValue (in AString aName, in nsIVariant aDefaultValue); */
    pub getValue: unsafe extern "C" fn (this: *const extIPreferenceBranch, aName: *const nsAString, aDefaultValue: *const nsIVariant, _retval: *mut *const nsIVariant) -> nsresult,

    /* void setValue (in AString aName, in nsIVariant aValue); */
    pub setValue: unsafe extern "C" fn (this: *const extIPreferenceBranch, aName: *const nsAString, aValue: *const nsIVariant) -> nsresult,

    /* void reset (); */
    pub reset: unsafe extern "C" fn (this: *const extIPreferenceBranch) -> nsresult,

}


impl extIPreferenceBranch {
    /* readonly attribute AString root; */
    #[inline]
    pub unsafe fn get_root(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_root)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIVariant all; */
    #[inline]
    pub unsafe fn get_all(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_all)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute extIEvents events; */
    #[inline]
    pub unsafe fn get_events(&self, ) -> Result<Option<RefPtr<extIEvents>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_events)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean has (in AString aName); */
    #[inline]
    pub unsafe fn has(&self, aName: &[u16]) -> Result<bool, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).has)(self as *const _, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* extIPreference get (in AString aName); */
    #[inline]
    pub unsafe fn get(&self, aName: &[u16]) -> Result<Option<RefPtr<extIPreference>>, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get)(self as *const _, &*aName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIVariant getValue (in AString aName, in nsIVariant aDefaultValue); */
    #[inline]
    pub unsafe fn getValue(&self, aName: &[u16], aDefaultValue: Option<&nsIVariant>) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getValue)(self as *const _, &*aName, aDefaultValue.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setValue (in AString aName, in nsIVariant aValue); */
    #[inline]
    pub unsafe fn setValue(&self, aName: &[u16], aValue: Option<&nsIVariant>) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).setValue)(self as *const _, &*aName, aValue.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct extIPreference {
    vtable: *const extIPreferenceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extIPreference {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2c7462e2, 0x72c2, 0x4473,
            [0x90, 0x07, 0x0e, 0x6a, 0xe7, 0x1e, 0x23, 0xca])
    }
}

unsafe impl RefCounted for extIPreference {
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
pub trait extIPreferenceCoerce {
    fn coerce_from(v: &extIPreference) -> &Self;
}

impl extIPreferenceCoerce for extIPreference {
    #[inline]
    fn coerce_from(v: &extIPreference) -> &Self {
        v
    }
}

impl extIPreference {
    #[inline]
    pub fn coerce<T: extIPreferenceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extIPreference {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extIPreferenceCoerce for T {
    #[inline]
    fn coerce_from(v: &extIPreference) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extIPreferenceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const extIPreference, aName: *mut nsAString) -> nsresult,

    /* readonly attribute AString type; */
    pub get_type_: unsafe extern "C" fn (this: *const extIPreference, aType: *mut nsAString) -> nsresult,

    /* attribute nsIVariant value; */
    pub get_value: unsafe extern "C" fn (this: *const extIPreference, aValue: *mut *const nsIVariant) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const extIPreference, aValue: *const nsIVariant) -> nsresult,

    /* attribute boolean locked; */
    pub get_locked: unsafe extern "C" fn (this: *const extIPreference, aLocked: *mut bool) -> nsresult,
    pub set_locked: unsafe extern "C" fn (this: *const extIPreference, aLocked: bool) -> nsresult,

    /* readonly attribute boolean modified; */
    pub get_modified: unsafe extern "C" fn (this: *const extIPreference, aModified: *mut bool) -> nsresult,

    /* readonly attribute extIPreferenceBranch branch; */
    pub get_branch: unsafe extern "C" fn (this: *const extIPreference, aBranch: *mut *const extIPreferenceBranch) -> nsresult,

    /* readonly attribute extIEvents events; */
    pub get_events: unsafe extern "C" fn (this: *const extIPreference, aEvents: *mut *const extIEvents) -> nsresult,

    /* void reset (); */
    pub reset: unsafe extern "C" fn (this: *const extIPreference) -> nsresult,

}


impl extIPreference {
    /* readonly attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIVariant value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_value)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_value(&self, aValue: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).set_value)(self as *const _, aValue.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean locked; */
    #[inline]
    pub unsafe fn get_locked(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_locked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_locked(&self, aLocked: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_locked)(self as *const _, aLocked) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean modified; */
    #[inline]
    pub unsafe fn get_modified(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_modified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute extIPreferenceBranch branch; */
    #[inline]
    pub unsafe fn get_branch(&self, ) -> Result<Option<RefPtr<extIPreferenceBranch>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_branch)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute extIEvents events; */
    #[inline]
    pub unsafe fn get_events(&self, ) -> Result<Option<RefPtr<extIEvents>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_events)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct extIExtension {
    vtable: *const extIExtensionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extIExtension {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x10cee02c, 0xf6e0, 0x4d61,
            [0xab, 0x27, 0xc1, 0x65, 0x72, 0xb1, 0x8c, 0x46])
    }
}

unsafe impl RefCounted for extIExtension {
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
pub trait extIExtensionCoerce {
    fn coerce_from(v: &extIExtension) -> &Self;
}

impl extIExtensionCoerce for extIExtension {
    #[inline]
    fn coerce_from(v: &extIExtension) -> &Self {
        v
    }
}

impl extIExtension {
    #[inline]
    pub fn coerce<T: extIExtensionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extIExtension {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extIExtensionCoerce for T {
    #[inline]
    fn coerce_from(v: &extIExtension) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extIExtensionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString id; */
    pub get_id: unsafe extern "C" fn (this: *const extIExtension, aId: *mut nsAString) -> nsresult,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const extIExtension, aName: *mut nsAString) -> nsresult,

    /* readonly attribute boolean enabled; */
    pub get_enabled: unsafe extern "C" fn (this: *const extIExtension, aEnabled: *mut bool) -> nsresult,

    /* readonly attribute AString version; */
    pub get_version: unsafe extern "C" fn (this: *const extIExtension, aVersion: *mut nsAString) -> nsresult,

    /* readonly attribute boolean firstRun; */
    pub get_firstRun: unsafe extern "C" fn (this: *const extIExtension, aFirstRun: *mut bool) -> nsresult,

    /* readonly attribute extIPreferenceBranch prefs; */
    pub get_prefs: unsafe extern "C" fn (this: *const extIExtension, aPrefs: *mut *const extIPreferenceBranch) -> nsresult,

    /* readonly attribute extISessionStorage storage; */
    pub get_storage: unsafe extern "C" fn (this: *const extIExtension, aStorage: *mut *const extISessionStorage) -> nsresult,

    /* readonly attribute extIEvents events; */
    pub get_events: unsafe extern "C" fn (this: *const extIExtension, aEvents: *mut *const extIEvents) -> nsresult,

}


impl extIExtension {
    /* readonly attribute AString id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_id)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean enabled; */
    #[inline]
    pub unsafe fn get_enabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_enabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString version; */
    #[inline]
    pub unsafe fn get_version(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean firstRun; */
    #[inline]
    pub unsafe fn get_firstRun(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_firstRun)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute extIPreferenceBranch prefs; */
    #[inline]
    pub unsafe fn get_prefs(&self, ) -> Result<Option<RefPtr<extIPreferenceBranch>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_prefs)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute extISessionStorage storage; */
    #[inline]
    pub unsafe fn get_storage(&self, ) -> Result<Option<RefPtr<extISessionStorage>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_storage)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute extIEvents events; */
    #[inline]
    pub unsafe fn get_events(&self, ) -> Result<Option<RefPtr<extIEvents>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_events)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct extIExtensions {
    vtable: *const extIExtensionsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extIExtensions {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xde281930, 0xaa5a, 0x11db,
            [0xab, 0xbd, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for extIExtensions {
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
pub trait extIExtensionsCoerce {
    fn coerce_from(v: &extIExtensions) -> &Self;
}

impl extIExtensionsCoerce for extIExtensions {
    #[inline]
    fn coerce_from(v: &extIExtensions) -> &Self {
        v
    }
}

impl extIExtensions {
    #[inline]
    pub fn coerce<T: extIExtensionsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extIExtensions {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extIExtensionsCoerce for T {
    #[inline]
    fn coerce_from(v: &extIExtensions) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extIExtensionsVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIVariant all; */
    pub get_all: unsafe extern "C" fn (this: *const extIExtensions, aAll: *mut *const nsIVariant) -> nsresult,

    /* boolean has (in AString aId); */
    pub has: unsafe extern "C" fn (this: *const extIExtensions, aId: *const nsAString, _retval: *mut bool) -> nsresult,

    /* extIExtension get (in AString aId); */
    pub get: unsafe extern "C" fn (this: *const extIExtensions, aId: *const nsAString, _retval: *mut *const extIExtension) -> nsresult,

}


impl extIExtensions {
    /* readonly attribute nsIVariant all; */
    #[inline]
    pub unsafe fn get_all(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_all)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean has (in AString aId); */
    #[inline]
    pub unsafe fn has(&self, aId: &[u16]) -> Result<bool, nsresult> {
        let aId = nsString::from(aId);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).has)(self as *const _, &*aId, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* extIExtension get (in AString aId); */
    #[inline]
    pub unsafe fn get(&self, aId: &[u16]) -> Result<Option<RefPtr<extIExtension>>, nsresult> {
        let aId = nsString::from(aId);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get)(self as *const _, &*aId, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct extIExtensionsCallback {
    vtable: *const extIExtensionsCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extIExtensionsCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2571cbb5, 0x550d, 0x4400,
            [0x80, 0x38, 0x75, 0xdf, 0x9b, 0x55, 0x3f, 0x98])
    }
}

unsafe impl RefCounted for extIExtensionsCallback {
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
pub trait extIExtensionsCallbackCoerce {
    fn coerce_from(v: &extIExtensionsCallback) -> &Self;
}

impl extIExtensionsCallbackCoerce for extIExtensionsCallback {
    #[inline]
    fn coerce_from(v: &extIExtensionsCallback) -> &Self {
        v
    }
}

impl extIExtensionsCallback {
    #[inline]
    pub fn coerce<T: extIExtensionsCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extIExtensionsCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extIExtensionsCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &extIExtensionsCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extIExtensionsCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void callback (in nsIVariant extensions); */
    pub callback: unsafe extern "C" fn (this: *const extIExtensionsCallback, extensions: *const nsIVariant) -> nsresult,

}


impl extIExtensionsCallback {
    /* void callback (in nsIVariant extensions); */
    #[inline]
    pub unsafe fn callback(&self, extensions: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).callback)(self as *const _, extensions.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct extISessionStorage {
    vtable: *const extISessionStorageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extISessionStorage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0787ac44, 0x29b9, 0x4889,
            [0xb9, 0x7f, 0x13, 0x57, 0x3a, 0xec, 0x69, 0x71])
    }
}

unsafe impl RefCounted for extISessionStorage {
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
pub trait extISessionStorageCoerce {
    fn coerce_from(v: &extISessionStorage) -> &Self;
}

impl extISessionStorageCoerce for extISessionStorage {
    #[inline]
    fn coerce_from(v: &extISessionStorage) -> &Self {
        v
    }
}

impl extISessionStorage {
    #[inline]
    pub fn coerce<T: extISessionStorageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extISessionStorage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extISessionStorageCoerce for T {
    #[inline]
    fn coerce_from(v: &extISessionStorage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extISessionStorageVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute extIEvents events; */
    pub get_events: unsafe extern "C" fn (this: *const extISessionStorage, aEvents: *mut *const extIEvents) -> nsresult,

    /* boolean has (in AString aName); */
    pub has: unsafe extern "C" fn (this: *const extISessionStorage, aName: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void set (in AString aName, in nsIVariant aValue); */
    pub set: unsafe extern "C" fn (this: *const extISessionStorage, aName: *const nsAString, aValue: *const nsIVariant) -> nsresult,

    /* nsIVariant get (in AString aName, in nsIVariant aDefaultValue); */
    pub get: unsafe extern "C" fn (this: *const extISessionStorage, aName: *const nsAString, aDefaultValue: *const nsIVariant, _retval: *mut *const nsIVariant) -> nsresult,

}


impl extISessionStorage {
    /* readonly attribute extIEvents events; */
    #[inline]
    pub unsafe fn get_events(&self, ) -> Result<Option<RefPtr<extIEvents>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_events)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean has (in AString aName); */
    #[inline]
    pub unsafe fn has(&self, aName: &[u16]) -> Result<bool, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).has)(self as *const _, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void set (in AString aName, in nsIVariant aValue); */
    #[inline]
    pub unsafe fn set(&self, aName: &[u16], aValue: Option<&nsIVariant>) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set)(self as *const _, &*aName, aValue.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIVariant get (in AString aName, in nsIVariant aDefaultValue); */
    #[inline]
    pub unsafe fn get(&self, aName: &[u16], aDefaultValue: Option<&nsIVariant>) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get)(self as *const _, &*aName, aDefaultValue.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct extIApplication {
    vtable: *const extIApplicationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for extIApplication {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2be87909, 0x0817, 0x4292,
            [0xac, 0xfa, 0xfc, 0x39, 0xbe, 0x53, 0xbe, 0x3f])
    }
}

unsafe impl RefCounted for extIApplication {
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
pub trait extIApplicationCoerce {
    fn coerce_from(v: &extIApplication) -> &Self;
}

impl extIApplicationCoerce for extIApplication {
    #[inline]
    fn coerce_from(v: &extIApplication) -> &Self {
        v
    }
}

impl extIApplication {
    #[inline]
    pub fn coerce<T: extIApplicationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for extIApplication {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> extIApplicationCoerce for T {
    #[inline]
    fn coerce_from(v: &extIApplication) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct extIApplicationVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString id; */
    pub get_id: unsafe extern "C" fn (this: *const extIApplication, aId: *mut nsAString) -> nsresult,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const extIApplication, aName: *mut nsAString) -> nsresult,

    /* readonly attribute AString version; */
    pub get_version: unsafe extern "C" fn (this: *const extIApplication, aVersion: *mut nsAString) -> nsresult,

    /* readonly attribute extIConsole console; */
    pub get_console: unsafe extern "C" fn (this: *const extIApplication, aConsole: *mut *const extIConsole) -> nsresult,

    /* void getExtensions (in extIExtensionsCallback aCallback); */
    pub getExtensions: unsafe extern "C" fn (this: *const extIApplication, aCallback: *const extIExtensionsCallback) -> nsresult,

    /* readonly attribute extIPreferenceBranch prefs; */
    pub get_prefs: unsafe extern "C" fn (this: *const extIApplication, aPrefs: *mut *const extIPreferenceBranch) -> nsresult,

    /* readonly attribute extISessionStorage storage; */
    pub get_storage: unsafe extern "C" fn (this: *const extIApplication, aStorage: *mut *const extISessionStorage) -> nsresult,

    /* readonly attribute extIEvents events; */
    pub get_events: unsafe extern "C" fn (this: *const extIApplication, aEvents: *mut *const extIEvents) -> nsresult,

    /* boolean quit (); */
    pub quit: unsafe extern "C" fn (this: *const extIApplication, _retval: *mut bool) -> nsresult,

    /* boolean restart (); */
    pub restart: unsafe extern "C" fn (this: *const extIApplication, _retval: *mut bool) -> nsresult,

}


impl extIApplication {
    /* readonly attribute AString id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_id)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString version; */
    #[inline]
    pub unsafe fn get_version(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute extIConsole console; */
    #[inline]
    pub unsafe fn get_console(&self, ) -> Result<Option<RefPtr<extIConsole>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_console)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getExtensions (in extIExtensionsCallback aCallback); */
    #[inline]
    pub unsafe fn getExtensions(&self, aCallback: Option<&extIExtensionsCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).getExtensions)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute extIPreferenceBranch prefs; */
    #[inline]
    pub unsafe fn get_prefs(&self, ) -> Result<Option<RefPtr<extIPreferenceBranch>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_prefs)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute extISessionStorage storage; */
    #[inline]
    pub unsafe fn get_storage(&self, ) -> Result<Option<RefPtr<extISessionStorage>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_storage)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute extIEvents events; */
    #[inline]
    pub unsafe fn get_events(&self, ) -> Result<Option<RefPtr<extIEvents>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_events)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean quit (); */
    #[inline]
    pub unsafe fn quit(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).quit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean restart (); */
    #[inline]
    pub unsafe fn restart(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).restart)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


