//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPerformanceStats.idl
//


#[repr(C)]
pub struct nsIPerformanceGroupDetails {
    vtable: *const nsIPerformanceGroupDetailsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPerformanceGroupDetails {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x994c56be, 0x939a, 0x4f20,
            [0x83, 0x64, 0x12, 0x4f, 0x64, 0x22, 0xd8, 0x6a])
    }
}

unsafe impl RefCounted for nsIPerformanceGroupDetails {
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
pub trait nsIPerformanceGroupDetailsCoerce {
    fn coerce_from(v: &nsIPerformanceGroupDetails) -> &Self;
}

impl nsIPerformanceGroupDetailsCoerce for nsIPerformanceGroupDetails {
    #[inline]
    fn coerce_from(v: &nsIPerformanceGroupDetails) -> &Self {
        v
    }
}

impl nsIPerformanceGroupDetails {
    #[inline]
    pub fn coerce<T: nsIPerformanceGroupDetailsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPerformanceGroupDetails {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPerformanceGroupDetailsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPerformanceGroupDetails) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPerformanceGroupDetailsVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString groupId; */
    pub get_groupId: unsafe extern "C" fn (this: *const nsIPerformanceGroupDetails, aGroupId: *mut nsAString) -> nsresult,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIPerformanceGroupDetails, aName: *mut nsAString) -> nsresult,

    /* readonly attribute uint64_t windowId; */
    pub get_windowId: unsafe extern "C" fn (this: *const nsIPerformanceGroupDetails, aWindowId: *mut uint64_t) -> nsresult,

    /* readonly attribute bool isSystem; */
    pub get_isSystem: unsafe extern "C" fn (this: *const nsIPerformanceGroupDetails, aIsSystem: *mut bool) -> nsresult,

    /* readonly attribute unsigned long long processId; */
    pub get_processId: unsafe extern "C" fn (this: *const nsIPerformanceGroupDetails, aProcessId: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute bool isContentProcess; */
    pub get_isContentProcess: unsafe extern "C" fn (this: *const nsIPerformanceGroupDetails, aIsContentProcess: *mut bool) -> nsresult,

}


impl nsIPerformanceGroupDetails {
    /* readonly attribute AString groupId; */
    #[inline]
    pub unsafe fn get_groupId(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_groupId)(self as *const _, &mut *_retval) {
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

    /* readonly attribute uint64_t windowId; */
    #[inline]
    pub unsafe fn get_windowId(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_windowId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool isSystem; */
    #[inline]
    pub unsafe fn get_isSystem(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSystem)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long processId; */
    #[inline]
    pub unsafe fn get_processId(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_processId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool isContentProcess; */
    #[inline]
    pub unsafe fn get_isContentProcess(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isContentProcess)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIPerformanceStats {
    vtable: *const nsIPerformanceStatsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPerformanceStats {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8a635d4b, 0xaa56, 0x466b,
            [0x9a, 0x7d, 0x9f, 0x91, 0xca, 0x94, 0x05, 0xef])
    }
}

unsafe impl RefCounted for nsIPerformanceStats {
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
pub trait nsIPerformanceStatsCoerce {
    fn coerce_from(v: &nsIPerformanceStats) -> &Self;
}

impl nsIPerformanceStatsCoerce for nsIPerformanceStats {
    #[inline]
    fn coerce_from(v: &nsIPerformanceStats) -> &Self {
        v
    }
}

impl nsIPerformanceStats {
    #[inline]
    pub fn coerce<T: nsIPerformanceStatsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPerformanceStats {
    type Target = nsIPerformanceGroupDetails;
    #[inline]
    fn deref(&self) -> &nsIPerformanceGroupDetails {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPerformanceGroupDetailsCoerce> nsIPerformanceStatsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPerformanceStats) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPerformanceStatsVTable {
    pub __base: nsIPerformanceGroupDetailsVTable,

    /* readonly attribute unsigned long long totalUserTime; */
    pub get_totalUserTime: unsafe extern "C" fn (this: *const nsIPerformanceStats, aTotalUserTime: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long long totalSystemTime; */
    pub get_totalSystemTime: unsafe extern "C" fn (this: *const nsIPerformanceStats, aTotalSystemTime: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long long totalCPOWTime; */
    pub get_totalCPOWTime: unsafe extern "C" fn (this: *const nsIPerformanceStats, aTotalCPOWTime: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long long ticks; */
    pub get_ticks: unsafe extern "C" fn (this: *const nsIPerformanceStats, aTicks: *mut libc::uint64_t) -> nsresult,

    /* void getDurations ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out unsigned long long aNumberOfOccurrences); */
    /// Unable to call function as its signature contains a non-rust type
    pub getDurations: *const ::libc::c_void,

}


impl nsIPerformanceStats {
    /* readonly attribute unsigned long long totalUserTime; */
    #[inline]
    pub unsafe fn get_totalUserTime(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_totalUserTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long totalSystemTime; */
    #[inline]
    pub unsafe fn get_totalSystemTime(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_totalSystemTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long totalCPOWTime; */
    #[inline]
    pub unsafe fn get_totalCPOWTime(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_totalCPOWTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long ticks; */
    #[inline]
    pub unsafe fn get_ticks(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_ticks)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getDurations ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out unsigned long long aNumberOfOccurrences); */


}


#[repr(C)]
pub struct nsIPerformanceSnapshot {
    vtable: *const nsIPerformanceSnapshotVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPerformanceSnapshot {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x13cc235b, 0x739e, 0x4690,
            [0xb0, 0xe3, 0xd8, 0x9c, 0xbe, 0x03, 0x6a, 0x93])
    }
}

unsafe impl RefCounted for nsIPerformanceSnapshot {
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
pub trait nsIPerformanceSnapshotCoerce {
    fn coerce_from(v: &nsIPerformanceSnapshot) -> &Self;
}

impl nsIPerformanceSnapshotCoerce for nsIPerformanceSnapshot {
    #[inline]
    fn coerce_from(v: &nsIPerformanceSnapshot) -> &Self {
        v
    }
}

impl nsIPerformanceSnapshot {
    #[inline]
    pub fn coerce<T: nsIPerformanceSnapshotCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPerformanceSnapshot {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPerformanceSnapshotCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPerformanceSnapshot) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPerformanceSnapshotVTable {
    pub __base: nsISupportsVTable,

    /* nsIArray getComponentsData (); */
    pub getComponentsData: unsafe extern "C" fn (this: *const nsIPerformanceSnapshot, _retval: *mut *const nsIArray) -> nsresult,

    /* nsIPerformanceStats getProcessData (); */
    pub getProcessData: unsafe extern "C" fn (this: *const nsIPerformanceSnapshot, _retval: *mut *const nsIPerformanceStats) -> nsresult,

}


impl nsIPerformanceSnapshot {
    /* nsIArray getComponentsData (); */
    #[inline]
    pub unsafe fn getComponentsData(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getComponentsData)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPerformanceStats getProcessData (); */
    #[inline]
    pub unsafe fn getProcessData(&self, ) -> Result<Option<RefPtr<nsIPerformanceStats>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getProcessData)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


pub mod nsIPerformanceAlert_consts {
    pub const REASON_SLOWDOWN: i64 = 1;
    pub const REASON_JANK_IN_ANIMATION: i64 = 2;
    pub const REASON_JANK_IN_INPUT: i64 = 4;
}


#[repr(C)]
pub struct nsIPerformanceAlert {
    vtable: *const nsIPerformanceAlertVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPerformanceAlert {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa85706ab, 0xd703, 0x4687,
            [0x88, 0x65, 0x78, 0xcd, 0x77, 0x1e, 0xab, 0x93])
    }
}

unsafe impl RefCounted for nsIPerformanceAlert {
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
pub trait nsIPerformanceAlertCoerce {
    fn coerce_from(v: &nsIPerformanceAlert) -> &Self;
}

impl nsIPerformanceAlertCoerce for nsIPerformanceAlert {
    #[inline]
    fn coerce_from(v: &nsIPerformanceAlert) -> &Self {
        v
    }
}

impl nsIPerformanceAlert {
    #[inline]
    pub fn coerce<T: nsIPerformanceAlertCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPerformanceAlert {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPerformanceAlertCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPerformanceAlert) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPerformanceAlertVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long reason; */
    pub get_reason: unsafe extern "C" fn (this: *const nsIPerformanceAlert, aReason: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long long highestJank; */
    pub get_highestJank: unsafe extern "C" fn (this: *const nsIPerformanceAlert, aHighestJank: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long long highestCPOW; */
    pub get_highestCPOW: unsafe extern "C" fn (this: *const nsIPerformanceAlert, aHighestCPOW: *mut libc::uint64_t) -> nsresult,

}


impl nsIPerformanceAlert {
    /* readonly attribute unsigned long reason; */
    #[inline]
    pub unsafe fn get_reason(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_reason)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long highestJank; */
    #[inline]
    pub unsafe fn get_highestJank(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_highestJank)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long highestCPOW; */
    #[inline]
    pub unsafe fn get_highestCPOW(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_highestCPOW)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIPerformanceObserver {
    vtable: *const nsIPerformanceObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPerformanceObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb746a929, 0x3fec, 0x420b,
            [0x8e, 0xd8, 0xc3, 0x5d, 0x71, 0x99, 0x5e, 0x05])
    }
}

unsafe impl RefCounted for nsIPerformanceObserver {
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
pub trait nsIPerformanceObserverCoerce {
    fn coerce_from(v: &nsIPerformanceObserver) -> &Self;
}

impl nsIPerformanceObserverCoerce for nsIPerformanceObserver {
    #[inline]
    fn coerce_from(v: &nsIPerformanceObserver) -> &Self {
        v
    }
}

impl nsIPerformanceObserver {
    #[inline]
    pub fn coerce<T: nsIPerformanceObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPerformanceObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPerformanceObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPerformanceObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPerformanceObserverVTable {
    pub __base: nsISupportsVTable,

    /* void observe (in nsIPerformanceGroupDetails target, in nsIPerformanceAlert alert); */
    pub observe: unsafe extern "C" fn (this: *const nsIPerformanceObserver, target: *const nsIPerformanceGroupDetails, alert: *const nsIPerformanceAlert) -> nsresult,

}


impl nsIPerformanceObserver {
    /* void observe (in nsIPerformanceGroupDetails target, in nsIPerformanceAlert alert); */
    #[inline]
    pub unsafe fn observe(&self, target: Option<&nsIPerformanceGroupDetails>, alert: Option<&nsIPerformanceAlert>) -> Result<(), nsresult> {

        match ((*self.vtable).observe)(self as *const _, target.map_or(::std::ptr::null(), |x| x as *const _), alert.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPerformanceObservable {
    vtable: *const nsIPerformanceObservableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPerformanceObservable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb85720d0, 0xe328, 0x4342,
            [0x9e, 0x46, 0x8c, 0xa1, 0xac, 0xf8, 0xc7, 0x0e])
    }
}

unsafe impl RefCounted for nsIPerformanceObservable {
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
pub trait nsIPerformanceObservableCoerce {
    fn coerce_from(v: &nsIPerformanceObservable) -> &Self;
}

impl nsIPerformanceObservableCoerce for nsIPerformanceObservable {
    #[inline]
    fn coerce_from(v: &nsIPerformanceObservable) -> &Self {
        v
    }
}

impl nsIPerformanceObservable {
    #[inline]
    pub fn coerce<T: nsIPerformanceObservableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPerformanceObservable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPerformanceObservableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPerformanceObservable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPerformanceObservableVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPerformanceGroupDetails target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIPerformanceObservable, aTarget: *mut *const nsIPerformanceGroupDetails) -> nsresult,

    /* void addJankObserver (in nsIPerformanceObserver observer); */
    pub addJankObserver: unsafe extern "C" fn (this: *const nsIPerformanceObservable, observer: *const nsIPerformanceObserver) -> nsresult,

    /* void removeJankObserver (in nsIPerformanceObserver observer); */
    pub removeJankObserver: unsafe extern "C" fn (this: *const nsIPerformanceObservable, observer: *const nsIPerformanceObserver) -> nsresult,

}


impl nsIPerformanceObservable {
    /* readonly attribute nsIPerformanceGroupDetails target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<Option<RefPtr<nsIPerformanceGroupDetails>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_target)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addJankObserver (in nsIPerformanceObserver observer); */
    #[inline]
    pub unsafe fn addJankObserver(&self, observer: Option<&nsIPerformanceObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addJankObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeJankObserver (in nsIPerformanceObserver observer); */
    #[inline]
    pub unsafe fn removeJankObserver(&self, observer: Option<&nsIPerformanceObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeJankObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPerformanceStatsService {
    vtable: *const nsIPerformanceStatsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPerformanceStatsService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x505bc42e, 0xbe38, 0x4a53,
            [0xba, 0xba, 0x92, 0xcb, 0x33, 0x69, 0x0c, 0xde])
    }
}

unsafe impl RefCounted for nsIPerformanceStatsService {
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
pub trait nsIPerformanceStatsServiceCoerce {
    fn coerce_from(v: &nsIPerformanceStatsService) -> &Self;
}

impl nsIPerformanceStatsServiceCoerce for nsIPerformanceStatsService {
    #[inline]
    fn coerce_from(v: &nsIPerformanceStatsService) -> &Self {
        v
    }
}

impl nsIPerformanceStatsService {
    #[inline]
    pub fn coerce<T: nsIPerformanceStatsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPerformanceStatsService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPerformanceStatsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPerformanceStatsService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPerformanceStatsServiceVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] attribute bool isMonitoringCPOW; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_isMonitoringCPOW: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_isMonitoringCPOW: *const ::libc::c_void,

    /* [implicit_jscontext] attribute bool isMonitoringJank; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_isMonitoringJank: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_isMonitoringJank: *const ::libc::c_void,

    /* [implicit_jscontext] attribute bool isMonitoringPerCompartment; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_isMonitoringPerCompartment: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_isMonitoringPerCompartment: *const ::libc::c_void,

    /* [implicit_jscontext] nsIPerformanceSnapshot getSnapshot (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSnapshot: *const ::libc::c_void,

    /* attribute unsigned long long jankAlertThreshold; */
    pub get_jankAlertThreshold: unsafe extern "C" fn (this: *const nsIPerformanceStatsService, aJankAlertThreshold: *mut libc::uint64_t) -> nsresult,
    pub set_jankAlertThreshold: unsafe extern "C" fn (this: *const nsIPerformanceStatsService, aJankAlertThreshold: libc::uint64_t) -> nsresult,

    /* attribute short animationJankLevelThreshold; */
    pub get_animationJankLevelThreshold: unsafe extern "C" fn (this: *const nsIPerformanceStatsService, aAnimationJankLevelThreshold: *mut libc::int16_t) -> nsresult,
    pub set_animationJankLevelThreshold: unsafe extern "C" fn (this: *const nsIPerformanceStatsService, aAnimationJankLevelThreshold: libc::int16_t) -> nsresult,

    /* attribute unsigned long long userInputDelayThreshold; */
    pub get_userInputDelayThreshold: unsafe extern "C" fn (this: *const nsIPerformanceStatsService, aUserInputDelayThreshold: *mut libc::uint64_t) -> nsresult,
    pub set_userInputDelayThreshold: unsafe extern "C" fn (this: *const nsIPerformanceStatsService, aUserInputDelayThreshold: libc::uint64_t) -> nsresult,

    /* attribute unsigned long jankAlertBufferingDelay; */
    pub get_jankAlertBufferingDelay: unsafe extern "C" fn (this: *const nsIPerformanceStatsService, aJankAlertBufferingDelay: *mut libc::uint32_t) -> nsresult,
    pub set_jankAlertBufferingDelay: unsafe extern "C" fn (this: *const nsIPerformanceStatsService, aJankAlertBufferingDelay: libc::uint32_t) -> nsresult,

    /* nsIPerformanceObservable getObservableWindow (in unsigned long long windowId); */
    pub getObservableWindow: unsafe extern "C" fn (this: *const nsIPerformanceStatsService, windowId: libc::uint64_t, _retval: *mut *const nsIPerformanceObservable) -> nsresult,

}


impl nsIPerformanceStatsService {
    /* [implicit_jscontext] attribute bool isMonitoringCPOW; */



    /* [implicit_jscontext] attribute bool isMonitoringJank; */



    /* [implicit_jscontext] attribute bool isMonitoringPerCompartment; */



    /* [implicit_jscontext] nsIPerformanceSnapshot getSnapshot (); */


    /* attribute unsigned long long jankAlertThreshold; */
    #[inline]
    pub unsafe fn get_jankAlertThreshold(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_jankAlertThreshold)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_jankAlertThreshold(&self, aJankAlertThreshold: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_jankAlertThreshold)(self as *const _, aJankAlertThreshold) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute short animationJankLevelThreshold; */
    #[inline]
    pub unsafe fn get_animationJankLevelThreshold(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_animationJankLevelThreshold)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_animationJankLevelThreshold(&self, aAnimationJankLevelThreshold: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_animationJankLevelThreshold)(self as *const _, aAnimationJankLevelThreshold) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long long userInputDelayThreshold; */
    #[inline]
    pub unsafe fn get_userInputDelayThreshold(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_userInputDelayThreshold)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_userInputDelayThreshold(&self, aUserInputDelayThreshold: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_userInputDelayThreshold)(self as *const _, aUserInputDelayThreshold) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long jankAlertBufferingDelay; */
    #[inline]
    pub unsafe fn get_jankAlertBufferingDelay(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_jankAlertBufferingDelay)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_jankAlertBufferingDelay(&self, aJankAlertBufferingDelay: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_jankAlertBufferingDelay)(self as *const _, aJankAlertBufferingDelay) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIPerformanceObservable getObservableWindow (in unsigned long long windowId); */
    #[inline]
    pub unsafe fn getObservableWindow(&self, windowId: libc::uint64_t) -> Result<Option<RefPtr<nsIPerformanceObservable>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getObservableWindow)(self as *const _, windowId, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


