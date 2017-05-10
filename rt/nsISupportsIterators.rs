//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISupportsIterators.idl
//


#[repr(C)]
pub struct nsIOutputIterator {
    vtable: *const nsIOutputIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIOutputIterator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7330650e, 0x1dd2, 0x11b2,
            [0xa0, 0xc2, 0x9f, 0xf8, 0x6e, 0xe9, 0x7b, 0xed])
    }
}

unsafe impl RefCounted for nsIOutputIterator {
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
pub trait nsIOutputIteratorCoerce {
    fn coerce_from(v: &nsIOutputIterator) -> &Self;
}

impl nsIOutputIteratorCoerce for nsIOutputIterator {
    #[inline]
    fn coerce_from(v: &nsIOutputIterator) -> &Self {
        v
    }
}

impl nsIOutputIterator {
    #[inline]
    pub fn coerce<T: nsIOutputIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIOutputIterator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIOutputIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOutputIterator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIOutputIteratorVTable {
    pub __base: nsISupportsVTable,

    /* void putElement (in nsISupports anElementToPut); */
    pub putElement: unsafe extern "C" fn (this: *const nsIOutputIterator, anElementToPut: *const nsISupports) -> nsresult,

    /* void stepForward (); */
    pub stepForward: unsafe extern "C" fn (this: *const nsIOutputIterator) -> nsresult,

}


impl nsIOutputIterator {
    /* void putElement (in nsISupports anElementToPut); */
    #[inline]
    pub unsafe fn putElement(&self, anElementToPut: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).putElement)(self as *const _, anElementToPut.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stepForward (); */
    #[inline]
    pub unsafe fn stepForward(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stepForward)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIInputIterator {
    vtable: *const nsIInputIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInputIterator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x85585e12, 0x1dd2, 0x11b2,
            [0xa9, 0x30, 0xf6, 0x92, 0x90, 0x58, 0x26, 0x9a])
    }
}

unsafe impl RefCounted for nsIInputIterator {
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
pub trait nsIInputIteratorCoerce {
    fn coerce_from(v: &nsIInputIterator) -> &Self;
}

impl nsIInputIteratorCoerce for nsIInputIterator {
    #[inline]
    fn coerce_from(v: &nsIInputIterator) -> &Self {
        v
    }
}

impl nsIInputIterator {
    #[inline]
    pub fn coerce<T: nsIInputIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInputIterator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInputIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputIterator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInputIteratorVTable {
    pub __base: nsISupportsVTable,

    /* nsISupports getElement (); */
    pub getElement: unsafe extern "C" fn (this: *const nsIInputIterator, _retval: *mut *const nsISupports) -> nsresult,

    /* void stepForward (); */
    pub stepForward: unsafe extern "C" fn (this: *const nsIInputIterator) -> nsresult,

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    pub isEqualTo: unsafe extern "C" fn (this: *const nsIInputIterator, anotherIterator: *const nsISupports, _retval: *mut bool) -> nsresult,

    /* nsISupports clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsIInputIterator, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsIInputIterator {
    /* nsISupports getElement (); */
    #[inline]
    pub unsafe fn getElement(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void stepForward (); */
    #[inline]
    pub unsafe fn stepForward(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stepForward)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    #[inline]
    pub unsafe fn isEqualTo(&self, anotherIterator: Option<&nsISupports>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isEqualTo)(self as *const _, anotherIterator.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIForwardIterator {
    vtable: *const nsIForwardIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIForwardIterator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8da01646, 0x1dd2, 0x11b2,
            [0x98, 0xa7, 0xc7, 0x00, 0x90, 0x45, 0xbe, 0x7e])
    }
}

unsafe impl RefCounted for nsIForwardIterator {
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
pub trait nsIForwardIteratorCoerce {
    fn coerce_from(v: &nsIForwardIterator) -> &Self;
}

impl nsIForwardIteratorCoerce for nsIForwardIterator {
    #[inline]
    fn coerce_from(v: &nsIForwardIterator) -> &Self {
        v
    }
}

impl nsIForwardIterator {
    #[inline]
    pub fn coerce<T: nsIForwardIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIForwardIterator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIForwardIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIForwardIterator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIForwardIteratorVTable {
    pub __base: nsISupportsVTable,

    /* nsISupports getElement (); */
    pub getElement: unsafe extern "C" fn (this: *const nsIForwardIterator, _retval: *mut *const nsISupports) -> nsresult,

    /* void putElement (in nsISupports anElementToPut); */
    pub putElement: unsafe extern "C" fn (this: *const nsIForwardIterator, anElementToPut: *const nsISupports) -> nsresult,

    /* void stepForward (); */
    pub stepForward: unsafe extern "C" fn (this: *const nsIForwardIterator) -> nsresult,

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    pub isEqualTo: unsafe extern "C" fn (this: *const nsIForwardIterator, anotherIterator: *const nsISupports, _retval: *mut bool) -> nsresult,

    /* nsISupports clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsIForwardIterator, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsIForwardIterator {
    /* nsISupports getElement (); */
    #[inline]
    pub unsafe fn getElement(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void putElement (in nsISupports anElementToPut); */
    #[inline]
    pub unsafe fn putElement(&self, anElementToPut: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).putElement)(self as *const _, anElementToPut.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stepForward (); */
    #[inline]
    pub unsafe fn stepForward(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stepForward)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    #[inline]
    pub unsafe fn isEqualTo(&self, anotherIterator: Option<&nsISupports>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isEqualTo)(self as *const _, anotherIterator.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIBidirectionalIterator {
    vtable: *const nsIBidirectionalIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBidirectionalIterator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x948defaa, 0x1dd1, 0x11b2,
            [0x89, 0xf6, 0x8c, 0xe8, 0x1f, 0x5e, 0xbd, 0xa9])
    }
}

unsafe impl RefCounted for nsIBidirectionalIterator {
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
pub trait nsIBidirectionalIteratorCoerce {
    fn coerce_from(v: &nsIBidirectionalIterator) -> &Self;
}

impl nsIBidirectionalIteratorCoerce for nsIBidirectionalIterator {
    #[inline]
    fn coerce_from(v: &nsIBidirectionalIterator) -> &Self {
        v
    }
}

impl nsIBidirectionalIterator {
    #[inline]
    pub fn coerce<T: nsIBidirectionalIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBidirectionalIterator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBidirectionalIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBidirectionalIterator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBidirectionalIteratorVTable {
    pub __base: nsISupportsVTable,

    /* nsISupports getElement (); */
    pub getElement: unsafe extern "C" fn (this: *const nsIBidirectionalIterator, _retval: *mut *const nsISupports) -> nsresult,

    /* void putElement (in nsISupports anElementToPut); */
    pub putElement: unsafe extern "C" fn (this: *const nsIBidirectionalIterator, anElementToPut: *const nsISupports) -> nsresult,

    /* void stepForward (); */
    pub stepForward: unsafe extern "C" fn (this: *const nsIBidirectionalIterator) -> nsresult,

    /* void stepBackward (); */
    pub stepBackward: unsafe extern "C" fn (this: *const nsIBidirectionalIterator) -> nsresult,

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    pub isEqualTo: unsafe extern "C" fn (this: *const nsIBidirectionalIterator, anotherIterator: *const nsISupports, _retval: *mut bool) -> nsresult,

    /* nsISupports clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsIBidirectionalIterator, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsIBidirectionalIterator {
    /* nsISupports getElement (); */
    #[inline]
    pub unsafe fn getElement(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void putElement (in nsISupports anElementToPut); */
    #[inline]
    pub unsafe fn putElement(&self, anElementToPut: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).putElement)(self as *const _, anElementToPut.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stepForward (); */
    #[inline]
    pub unsafe fn stepForward(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stepForward)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stepBackward (); */
    #[inline]
    pub unsafe fn stepBackward(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stepBackward)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    #[inline]
    pub unsafe fn isEqualTo(&self, anotherIterator: Option<&nsISupports>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isEqualTo)(self as *const _, anotherIterator.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIRandomAccessIterator {
    vtable: *const nsIRandomAccessIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRandomAccessIterator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9bd6fdb0, 0x1dd1, 0x11b2,
            [0x91, 0x01, 0xd1, 0x53, 0x75, 0x96, 0x82, 0x30])
    }
}

unsafe impl RefCounted for nsIRandomAccessIterator {
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
pub trait nsIRandomAccessIteratorCoerce {
    fn coerce_from(v: &nsIRandomAccessIterator) -> &Self;
}

impl nsIRandomAccessIteratorCoerce for nsIRandomAccessIterator {
    #[inline]
    fn coerce_from(v: &nsIRandomAccessIterator) -> &Self {
        v
    }
}

impl nsIRandomAccessIterator {
    #[inline]
    pub fn coerce<T: nsIRandomAccessIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRandomAccessIterator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRandomAccessIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRandomAccessIterator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRandomAccessIteratorVTable {
    pub __base: nsISupportsVTable,

    /* nsISupports getElement (); */
    pub getElement: unsafe extern "C" fn (this: *const nsIRandomAccessIterator, _retval: *mut *const nsISupports) -> nsresult,

    /* nsISupports getElementAt (in int32_t anOffset); */
    pub getElementAt: unsafe extern "C" fn (this: *const nsIRandomAccessIterator, anOffset: int32_t, _retval: *mut *const nsISupports) -> nsresult,

    /* void putElement (in nsISupports anElementToPut); */
    pub putElement: unsafe extern "C" fn (this: *const nsIRandomAccessIterator, anElementToPut: *const nsISupports) -> nsresult,

    /* void putElementAt (in int32_t anOffset, in nsISupports anElementToPut); */
    pub putElementAt: unsafe extern "C" fn (this: *const nsIRandomAccessIterator, anOffset: int32_t, anElementToPut: *const nsISupports) -> nsresult,

    /* void stepForward (); */
    pub stepForward: unsafe extern "C" fn (this: *const nsIRandomAccessIterator) -> nsresult,

    /* void stepForwardBy (in int32_t anOffset); */
    pub stepForwardBy: unsafe extern "C" fn (this: *const nsIRandomAccessIterator, anOffset: int32_t) -> nsresult,

    /* void stepBackward (); */
    pub stepBackward: unsafe extern "C" fn (this: *const nsIRandomAccessIterator) -> nsresult,

    /* void stepBackwardBy (in int32_t anOffset); */
    pub stepBackwardBy: unsafe extern "C" fn (this: *const nsIRandomAccessIterator, anOffset: int32_t) -> nsresult,

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    pub isEqualTo: unsafe extern "C" fn (this: *const nsIRandomAccessIterator, anotherIterator: *const nsISupports, _retval: *mut bool) -> nsresult,

    /* nsISupports clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsIRandomAccessIterator, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsIRandomAccessIterator {
    /* nsISupports getElement (); */
    #[inline]
    pub unsafe fn getElement(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISupports getElementAt (in int32_t anOffset); */
    #[inline]
    pub unsafe fn getElementAt(&self, anOffset: int32_t) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElementAt)(self as *const _, anOffset, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void putElement (in nsISupports anElementToPut); */
    #[inline]
    pub unsafe fn putElement(&self, anElementToPut: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).putElement)(self as *const _, anElementToPut.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void putElementAt (in int32_t anOffset, in nsISupports anElementToPut); */
    #[inline]
    pub unsafe fn putElementAt(&self, anOffset: int32_t, anElementToPut: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).putElementAt)(self as *const _, anOffset, anElementToPut.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stepForward (); */
    #[inline]
    pub unsafe fn stepForward(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stepForward)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stepForwardBy (in int32_t anOffset); */
    #[inline]
    pub unsafe fn stepForwardBy(&self, anOffset: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).stepForwardBy)(self as *const _, anOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stepBackward (); */
    #[inline]
    pub unsafe fn stepBackward(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stepBackward)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stepBackwardBy (in int32_t anOffset); */
    #[inline]
    pub unsafe fn stepBackwardBy(&self, anOffset: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).stepBackwardBy)(self as *const _, anOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    #[inline]
    pub unsafe fn isEqualTo(&self, anotherIterator: Option<&nsISupports>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isEqualTo)(self as *const _, anotherIterator.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


