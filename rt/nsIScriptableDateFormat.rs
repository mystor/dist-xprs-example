//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptableDateFormat.idl
//


pub type nsDateFormatSelector = libc::int32_t;


pub type nsTimeFormatSelector = libc::int32_t;


pub mod nsIScriptableDateFormat_consts {
    pub const dateFormatNone: i64 = 0;
    pub const dateFormatLong: i64 = 1;
    pub const dateFormatShort: i64 = 2;
    pub const dateFormatYearMonth: i64 = 3;
    pub const dateFormatWeekday: i64 = 4;
    pub const timeFormatNone: i64 = 0;
    pub const timeFormatSeconds: i64 = 1;
    pub const timeFormatNoSeconds: i64 = 2;
    pub const timeFormatSecondsForce24Hour: i64 = 3;
    pub const timeFormatNoSecondsForce24Hour: i64 = 4;
}


#[repr(C)]
pub struct nsIScriptableDateFormat {
    vtable: *const nsIScriptableDateFormatVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptableDateFormat {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0c89efb0, 0x1aae, 0x11d3,
            [0x91, 0x41, 0x00, 0x60, 0x08, 0xa6, 0xed, 0xf6])
    }
}

unsafe impl RefCounted for nsIScriptableDateFormat {
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
pub trait nsIScriptableDateFormatCoerce {
    fn coerce_from(v: &nsIScriptableDateFormat) -> &Self;
}

impl nsIScriptableDateFormatCoerce for nsIScriptableDateFormat {
    #[inline]
    fn coerce_from(v: &nsIScriptableDateFormat) -> &Self {
        v
    }
}

impl nsIScriptableDateFormat {
    #[inline]
    pub fn coerce<T: nsIScriptableDateFormatCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptableDateFormat {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScriptableDateFormatCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableDateFormat) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptableDateFormatVTable {
    pub __base: nsISupportsVTable,

    /* wstring FormatDateTime (in wstring locale, in long dateFormatSelector, in long timeFormatSelector, in long year, in long month, in long day, in long hour, in long minute, in long second); */
    pub FormatDateTime: unsafe extern "C" fn (this: *const nsIScriptableDateFormat, locale: *const libc::int16_t, dateFormatSelector: libc::int32_t, timeFormatSelector: libc::int32_t, year: libc::int32_t, month: libc::int32_t, day: libc::int32_t, hour: libc::int32_t, minute: libc::int32_t, second: libc::int32_t, _retval: *mut *const libc::int16_t) -> nsresult,

    /* wstring FormatDate (in wstring locale, in long dateFormatSelector, in long year, in long month, in long day); */
    pub FormatDate: unsafe extern "C" fn (this: *const nsIScriptableDateFormat, locale: *const libc::int16_t, dateFormatSelector: libc::int32_t, year: libc::int32_t, month: libc::int32_t, day: libc::int32_t, _retval: *mut *const libc::int16_t) -> nsresult,

    /* wstring FormatTime (in wstring locale, in long timeFormatSelector, in long hour, in long minute, in long second); */
    pub FormatTime: unsafe extern "C" fn (this: *const nsIScriptableDateFormat, locale: *const libc::int16_t, timeFormatSelector: libc::int32_t, hour: libc::int32_t, minute: libc::int32_t, second: libc::int32_t, _retval: *mut *const libc::int16_t) -> nsresult,

}


impl nsIScriptableDateFormat {
    /* wstring FormatDateTime (in wstring locale, in long dateFormatSelector, in long timeFormatSelector, in long year, in long month, in long day, in long hour, in long minute, in long second); */
    #[inline]
    pub unsafe fn FormatDateTime(&self, locale: *const libc::int16_t, dateFormatSelector: libc::int32_t, timeFormatSelector: libc::int32_t, year: libc::int32_t, month: libc::int32_t, day: libc::int32_t, hour: libc::int32_t, minute: libc::int32_t, second: libc::int32_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).FormatDateTime)(self as *const _, locale, dateFormatSelector, timeFormatSelector, year, month, day, hour, minute, second, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* wstring FormatDate (in wstring locale, in long dateFormatSelector, in long year, in long month, in long day); */
    #[inline]
    pub unsafe fn FormatDate(&self, locale: *const libc::int16_t, dateFormatSelector: libc::int32_t, year: libc::int32_t, month: libc::int32_t, day: libc::int32_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).FormatDate)(self as *const _, locale, dateFormatSelector, year, month, day, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* wstring FormatTime (in wstring locale, in long timeFormatSelector, in long hour, in long minute, in long second); */
    #[inline]
    pub unsafe fn FormatTime(&self, locale: *const libc::int16_t, timeFormatSelector: libc::int32_t, hour: libc::int32_t, minute: libc::int32_t, second: libc::int32_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).FormatTime)(self as *const _, locale, timeFormatSelector, hour, minute, second, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


