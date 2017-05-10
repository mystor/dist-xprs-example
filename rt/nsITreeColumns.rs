//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITreeColumns.idl
//


pub mod nsITreeColumn_consts {
    pub const TYPE_TEXT: i64 = 1;
    pub const TYPE_CHECKBOX: i64 = 2;
    pub const TYPE_PROGRESSMETER: i64 = 3;
    pub const TYPE_PASSWORD: i64 = 4;
}


#[repr(C)]
pub struct nsITreeColumn {
    vtable: *const nsITreeColumnVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITreeColumn {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xae835ecf, 0x6b32, 0x4660,
            [0x9b, 0x43, 0x8a, 0x27, 0x0d, 0xf5, 0x6e, 0x02])
    }
}

unsafe impl RefCounted for nsITreeColumn {
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
pub trait nsITreeColumnCoerce {
    fn coerce_from(v: &nsITreeColumn) -> &Self;
}

impl nsITreeColumnCoerce for nsITreeColumn {
    #[inline]
    fn coerce_from(v: &nsITreeColumn) -> &Self {
        v
    }
}

impl nsITreeColumn {
    #[inline]
    pub fn coerce<T: nsITreeColumnCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITreeColumn {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITreeColumnCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITreeColumn) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITreeColumnVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMElement element; */
    pub get_element: unsafe extern "C" fn (this: *const nsITreeColumn, aElement: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute nsITreeColumns columns; */
    pub get_columns: unsafe extern "C" fn (this: *const nsITreeColumn, aColumns: *mut *const nsITreeColumns) -> nsresult,

    /* readonly attribute long x; */
    pub get_x: unsafe extern "C" fn (this: *const nsITreeColumn, aX: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long width; */
    pub get_width: unsafe extern "C" fn (this: *const nsITreeColumn, aWidth: *mut libc::int32_t) -> nsresult,

    /* readonly attribute AString id; */
    pub get_id: unsafe extern "C" fn (this: *const nsITreeColumn, aId: *mut nsAString) -> nsresult,

    /* [noscript] void getIdConst ([shared] out wstring idConst); */
    pub getIdConst: unsafe extern "C" fn (this: *const nsITreeColumn, idConst: *mut *const libc::int16_t) -> nsresult,

    /* [noscript] readonly attribute nsIAtom atom; */
    pub get_atom: unsafe extern "C" fn (this: *const nsITreeColumn, aAtom: *mut *const nsIAtom) -> nsresult,

    /* readonly attribute long index; */
    pub get_index: unsafe extern "C" fn (this: *const nsITreeColumn, aIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean primary; */
    pub get_primary: unsafe extern "C" fn (this: *const nsITreeColumn, aPrimary: *mut bool) -> nsresult,

    /* readonly attribute boolean cycler; */
    pub get_cycler: unsafe extern "C" fn (this: *const nsITreeColumn, aCycler: *mut bool) -> nsresult,

    /* readonly attribute boolean editable; */
    pub get_editable: unsafe extern "C" fn (this: *const nsITreeColumn, aEditable: *mut bool) -> nsresult,

    /* readonly attribute boolean selectable; */
    pub get_selectable: unsafe extern "C" fn (this: *const nsITreeColumn, aSelectable: *mut bool) -> nsresult,

    /* readonly attribute short type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsITreeColumn, aType: *mut libc::int16_t) -> nsresult,

    /* nsITreeColumn getNext (); */
    pub getNext: unsafe extern "C" fn (this: *const nsITreeColumn, _retval: *mut *const nsITreeColumn) -> nsresult,

    /* nsITreeColumn getPrevious (); */
    pub getPrevious: unsafe extern "C" fn (this: *const nsITreeColumn, _retval: *mut *const nsITreeColumn) -> nsresult,

    /* void invalidate (); */
    pub invalidate: unsafe extern "C" fn (this: *const nsITreeColumn) -> nsresult,

}


impl nsITreeColumn {
    /* readonly attribute nsIDOMElement element; */
    #[inline]
    pub unsafe fn get_element(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_element)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsITreeColumns columns; */
    #[inline]
    pub unsafe fn get_columns(&self, ) -> Result<Option<RefPtr<nsITreeColumns>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_columns)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long x; */
    #[inline]
    pub unsafe fn get_x(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_x)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_width)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

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

    /* [noscript] void getIdConst ([shared] out wstring idConst); */
    #[inline]
    pub unsafe fn getIdConst(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut idConst: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getIdConst)(self as *const _, &mut idConst as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(idConst)
    }

    /* [noscript] readonly attribute nsIAtom atom; */
    #[inline]
    pub unsafe fn get_atom(&self, ) -> Result<Option<RefPtr<nsIAtom>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_atom)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long index; */
    #[inline]
    pub unsafe fn get_index(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_index)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean primary; */
    #[inline]
    pub unsafe fn get_primary(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_primary)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean cycler; */
    #[inline]
    pub unsafe fn get_cycler(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_cycler)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean editable; */
    #[inline]
    pub unsafe fn get_editable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_editable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean selectable; */
    #[inline]
    pub unsafe fn get_selectable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_selectable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute short type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsITreeColumn getNext (); */
    #[inline]
    pub unsafe fn getNext(&self, ) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNext)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITreeColumn getPrevious (); */
    #[inline]
    pub unsafe fn getPrevious(&self, ) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPrevious)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void invalidate (); */
    #[inline]
    pub unsafe fn invalidate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).invalidate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsITreeColumns {
    vtable: *const nsITreeColumnsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITreeColumns {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf8a8d6b4, 0x6788, 0x438d,
            [0x90, 0x09, 0x71, 0x42, 0x79, 0x87, 0x67, 0xab])
    }
}

unsafe impl RefCounted for nsITreeColumns {
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
pub trait nsITreeColumnsCoerce {
    fn coerce_from(v: &nsITreeColumns) -> &Self;
}

impl nsITreeColumnsCoerce for nsITreeColumns {
    #[inline]
    fn coerce_from(v: &nsITreeColumns) -> &Self {
        v
    }
}

impl nsITreeColumns {
    #[inline]
    pub fn coerce<T: nsITreeColumnsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITreeColumns {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITreeColumnsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITreeColumns) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITreeColumnsVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsITreeBoxObject tree; */
    pub get_tree: unsafe extern "C" fn (this: *const nsITreeColumns, aTree: *mut *const nsITreeBoxObject) -> nsresult,

    /* readonly attribute long count; */
    pub get_count: unsafe extern "C" fn (this: *const nsITreeColumns, aCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsITreeColumns, aLength: *mut libc::int32_t) -> nsresult,

    /* nsITreeColumn getFirstColumn (); */
    pub getFirstColumn: unsafe extern "C" fn (this: *const nsITreeColumns, _retval: *mut *const nsITreeColumn) -> nsresult,

    /* nsITreeColumn getLastColumn (); */
    pub getLastColumn: unsafe extern "C" fn (this: *const nsITreeColumns, _retval: *mut *const nsITreeColumn) -> nsresult,

    /* nsITreeColumn getPrimaryColumn (); */
    pub getPrimaryColumn: unsafe extern "C" fn (this: *const nsITreeColumns, _retval: *mut *const nsITreeColumn) -> nsresult,

    /* nsITreeColumn getSortedColumn (); */
    pub getSortedColumn: unsafe extern "C" fn (this: *const nsITreeColumns, _retval: *mut *const nsITreeColumn) -> nsresult,

    /* nsITreeColumn getKeyColumn (); */
    pub getKeyColumn: unsafe extern "C" fn (this: *const nsITreeColumns, _retval: *mut *const nsITreeColumn) -> nsresult,

    /* nsITreeColumn getColumnFor (in nsIDOMElement element); */
    pub getColumnFor: unsafe extern "C" fn (this: *const nsITreeColumns, element: *const nsIDOMElement, _retval: *mut *const nsITreeColumn) -> nsresult,

    /* nsITreeColumn getNamedColumn (in AString id); */
    pub getNamedColumn: unsafe extern "C" fn (this: *const nsITreeColumns, id: *const nsAString, _retval: *mut *const nsITreeColumn) -> nsresult,

    /* nsITreeColumn getColumnAt (in long index); */
    pub getColumnAt: unsafe extern "C" fn (this: *const nsITreeColumns, index: libc::int32_t, _retval: *mut *const nsITreeColumn) -> nsresult,

    /* void invalidateColumns (); */
    pub invalidateColumns: unsafe extern "C" fn (this: *const nsITreeColumns) -> nsresult,

    /* void restoreNaturalOrder (); */
    pub restoreNaturalOrder: unsafe extern "C" fn (this: *const nsITreeColumns) -> nsresult,

}


impl nsITreeColumns {
    /* readonly attribute nsITreeBoxObject tree; */
    #[inline]
    pub unsafe fn get_tree(&self, ) -> Result<Option<RefPtr<nsITreeBoxObject>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_tree)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long count; */
    #[inline]
    pub unsafe fn get_count(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_count)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsITreeColumn getFirstColumn (); */
    #[inline]
    pub unsafe fn getFirstColumn(&self, ) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFirstColumn)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITreeColumn getLastColumn (); */
    #[inline]
    pub unsafe fn getLastColumn(&self, ) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getLastColumn)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITreeColumn getPrimaryColumn (); */
    #[inline]
    pub unsafe fn getPrimaryColumn(&self, ) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPrimaryColumn)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITreeColumn getSortedColumn (); */
    #[inline]
    pub unsafe fn getSortedColumn(&self, ) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSortedColumn)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITreeColumn getKeyColumn (); */
    #[inline]
    pub unsafe fn getKeyColumn(&self, ) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getKeyColumn)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITreeColumn getColumnFor (in nsIDOMElement element); */
    #[inline]
    pub unsafe fn getColumnFor(&self, element: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getColumnFor)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITreeColumn getNamedColumn (in AString id); */
    #[inline]
    pub unsafe fn getNamedColumn(&self, id: &[u16]) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let id = nsString::from(id);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNamedColumn)(self as *const _, &*id, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITreeColumn getColumnAt (in long index); */
    #[inline]
    pub unsafe fn getColumnAt(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsITreeColumn>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getColumnAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void invalidateColumns (); */
    #[inline]
    pub unsafe fn invalidateColumns(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).invalidateColumns)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void restoreNaturalOrder (); */
    #[inline]
    pub unsafe fn restoreNaturalOrder(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).restoreNaturalOrder)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


