//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMKeyEvent.idl
//


pub mod nsIDOMKeyEvent_consts {
    pub const DOM_VK_CANCEL: i64 = 3;
    pub const DOM_VK_HELP: i64 = 6;
    pub const DOM_VK_BACK_SPACE: i64 = 8;
    pub const DOM_VK_TAB: i64 = 9;
    pub const DOM_VK_CLEAR: i64 = 12;
    pub const DOM_VK_RETURN: i64 = 13;
    pub const DOM_VK_SHIFT: i64 = 16;
    pub const DOM_VK_CONTROL: i64 = 17;
    pub const DOM_VK_ALT: i64 = 18;
    pub const DOM_VK_PAUSE: i64 = 19;
    pub const DOM_VK_CAPS_LOCK: i64 = 20;
    pub const DOM_VK_KANA: i64 = 21;
    pub const DOM_VK_HANGUL: i64 = 21;
    pub const DOM_VK_EISU: i64 = 22;
    pub const DOM_VK_JUNJA: i64 = 23;
    pub const DOM_VK_FINAL: i64 = 24;
    pub const DOM_VK_HANJA: i64 = 25;
    pub const DOM_VK_KANJI: i64 = 25;
    pub const DOM_VK_ESCAPE: i64 = 27;
    pub const DOM_VK_CONVERT: i64 = 28;
    pub const DOM_VK_NONCONVERT: i64 = 29;
    pub const DOM_VK_ACCEPT: i64 = 30;
    pub const DOM_VK_MODECHANGE: i64 = 31;
    pub const DOM_VK_SPACE: i64 = 32;
    pub const DOM_VK_PAGE_UP: i64 = 33;
    pub const DOM_VK_PAGE_DOWN: i64 = 34;
    pub const DOM_VK_END: i64 = 35;
    pub const DOM_VK_HOME: i64 = 36;
    pub const DOM_VK_LEFT: i64 = 37;
    pub const DOM_VK_UP: i64 = 38;
    pub const DOM_VK_RIGHT: i64 = 39;
    pub const DOM_VK_DOWN: i64 = 40;
    pub const DOM_VK_SELECT: i64 = 41;
    pub const DOM_VK_PRINT: i64 = 42;
    pub const DOM_VK_EXECUTE: i64 = 43;
    pub const DOM_VK_PRINTSCREEN: i64 = 44;
    pub const DOM_VK_INSERT: i64 = 45;
    pub const DOM_VK_DELETE: i64 = 46;
    pub const DOM_VK_0: i64 = 48;
    pub const DOM_VK_1: i64 = 49;
    pub const DOM_VK_2: i64 = 50;
    pub const DOM_VK_3: i64 = 51;
    pub const DOM_VK_4: i64 = 52;
    pub const DOM_VK_5: i64 = 53;
    pub const DOM_VK_6: i64 = 54;
    pub const DOM_VK_7: i64 = 55;
    pub const DOM_VK_8: i64 = 56;
    pub const DOM_VK_9: i64 = 57;
    pub const DOM_VK_COLON: i64 = 58;
    pub const DOM_VK_SEMICOLON: i64 = 59;
    pub const DOM_VK_LESS_THAN: i64 = 60;
    pub const DOM_VK_EQUALS: i64 = 61;
    pub const DOM_VK_GREATER_THAN: i64 = 62;
    pub const DOM_VK_QUESTION_MARK: i64 = 63;
    pub const DOM_VK_AT: i64 = 64;
    pub const DOM_VK_A: i64 = 65;
    pub const DOM_VK_B: i64 = 66;
    pub const DOM_VK_C: i64 = 67;
    pub const DOM_VK_D: i64 = 68;
    pub const DOM_VK_E: i64 = 69;
    pub const DOM_VK_F: i64 = 70;
    pub const DOM_VK_G: i64 = 71;
    pub const DOM_VK_H: i64 = 72;
    pub const DOM_VK_I: i64 = 73;
    pub const DOM_VK_J: i64 = 74;
    pub const DOM_VK_K: i64 = 75;
    pub const DOM_VK_L: i64 = 76;
    pub const DOM_VK_M: i64 = 77;
    pub const DOM_VK_N: i64 = 78;
    pub const DOM_VK_O: i64 = 79;
    pub const DOM_VK_P: i64 = 80;
    pub const DOM_VK_Q: i64 = 81;
    pub const DOM_VK_R: i64 = 82;
    pub const DOM_VK_S: i64 = 83;
    pub const DOM_VK_T: i64 = 84;
    pub const DOM_VK_U: i64 = 85;
    pub const DOM_VK_V: i64 = 86;
    pub const DOM_VK_W: i64 = 87;
    pub const DOM_VK_X: i64 = 88;
    pub const DOM_VK_Y: i64 = 89;
    pub const DOM_VK_Z: i64 = 90;
    pub const DOM_VK_WIN: i64 = 91;
    pub const DOM_VK_CONTEXT_MENU: i64 = 93;
    pub const DOM_VK_SLEEP: i64 = 95;
    pub const DOM_VK_NUMPAD0: i64 = 96;
    pub const DOM_VK_NUMPAD1: i64 = 97;
    pub const DOM_VK_NUMPAD2: i64 = 98;
    pub const DOM_VK_NUMPAD3: i64 = 99;
    pub const DOM_VK_NUMPAD4: i64 = 100;
    pub const DOM_VK_NUMPAD5: i64 = 101;
    pub const DOM_VK_NUMPAD6: i64 = 102;
    pub const DOM_VK_NUMPAD7: i64 = 103;
    pub const DOM_VK_NUMPAD8: i64 = 104;
    pub const DOM_VK_NUMPAD9: i64 = 105;
    pub const DOM_VK_MULTIPLY: i64 = 106;
    pub const DOM_VK_ADD: i64 = 107;
    pub const DOM_VK_SEPARATOR: i64 = 108;
    pub const DOM_VK_SUBTRACT: i64 = 109;
    pub const DOM_VK_DECIMAL: i64 = 110;
    pub const DOM_VK_DIVIDE: i64 = 111;
    pub const DOM_VK_F1: i64 = 112;
    pub const DOM_VK_F2: i64 = 113;
    pub const DOM_VK_F3: i64 = 114;
    pub const DOM_VK_F4: i64 = 115;
    pub const DOM_VK_F5: i64 = 116;
    pub const DOM_VK_F6: i64 = 117;
    pub const DOM_VK_F7: i64 = 118;
    pub const DOM_VK_F8: i64 = 119;
    pub const DOM_VK_F9: i64 = 120;
    pub const DOM_VK_F10: i64 = 121;
    pub const DOM_VK_F11: i64 = 122;
    pub const DOM_VK_F12: i64 = 123;
    pub const DOM_VK_F13: i64 = 124;
    pub const DOM_VK_F14: i64 = 125;
    pub const DOM_VK_F15: i64 = 126;
    pub const DOM_VK_F16: i64 = 127;
    pub const DOM_VK_F17: i64 = 128;
    pub const DOM_VK_F18: i64 = 129;
    pub const DOM_VK_F19: i64 = 130;
    pub const DOM_VK_F20: i64 = 131;
    pub const DOM_VK_F21: i64 = 132;
    pub const DOM_VK_F22: i64 = 133;
    pub const DOM_VK_F23: i64 = 134;
    pub const DOM_VK_F24: i64 = 135;
    pub const DOM_VK_NUM_LOCK: i64 = 144;
    pub const DOM_VK_SCROLL_LOCK: i64 = 145;
    pub const DOM_VK_WIN_OEM_FJ_JISHO: i64 = 146;
    pub const DOM_VK_WIN_OEM_FJ_MASSHOU: i64 = 147;
    pub const DOM_VK_WIN_OEM_FJ_TOUROKU: i64 = 148;
    pub const DOM_VK_WIN_OEM_FJ_LOYA: i64 = 149;
    pub const DOM_VK_WIN_OEM_FJ_ROYA: i64 = 150;
    pub const DOM_VK_CIRCUMFLEX: i64 = 160;
    pub const DOM_VK_EXCLAMATION: i64 = 161;
    pub const DOM_VK_DOUBLE_QUOTE: i64 = 162;
    pub const DOM_VK_HASH: i64 = 163;
    pub const DOM_VK_DOLLAR: i64 = 164;
    pub const DOM_VK_PERCENT: i64 = 165;
    pub const DOM_VK_AMPERSAND: i64 = 166;
    pub const DOM_VK_UNDERSCORE: i64 = 167;
    pub const DOM_VK_OPEN_PAREN: i64 = 168;
    pub const DOM_VK_CLOSE_PAREN: i64 = 169;
    pub const DOM_VK_ASTERISK: i64 = 170;
    pub const DOM_VK_PLUS: i64 = 171;
    pub const DOM_VK_PIPE: i64 = 172;
    pub const DOM_VK_HYPHEN_MINUS: i64 = 173;
    pub const DOM_VK_OPEN_CURLY_BRACKET: i64 = 174;
    pub const DOM_VK_CLOSE_CURLY_BRACKET: i64 = 175;
    pub const DOM_VK_TILDE: i64 = 176;
    pub const DOM_VK_VOLUME_MUTE: i64 = 181;
    pub const DOM_VK_VOLUME_DOWN: i64 = 182;
    pub const DOM_VK_VOLUME_UP: i64 = 183;
    pub const DOM_VK_COMMA: i64 = 188;
    pub const DOM_VK_PERIOD: i64 = 190;
    pub const DOM_VK_SLASH: i64 = 191;
    pub const DOM_VK_BACK_QUOTE: i64 = 192;
    pub const DOM_VK_OPEN_BRACKET: i64 = 219;
    pub const DOM_VK_BACK_SLASH: i64 = 220;
    pub const DOM_VK_CLOSE_BRACKET: i64 = 221;
    pub const DOM_VK_QUOTE: i64 = 222;
    pub const DOM_VK_META: i64 = 224;
    pub const DOM_VK_ALTGR: i64 = 225;
    pub const DOM_VK_WIN_ICO_HELP: i64 = 227;
    pub const DOM_VK_WIN_ICO_00: i64 = 228;
    pub const DOM_VK_WIN_ICO_CLEAR: i64 = 230;
    pub const DOM_VK_WIN_OEM_RESET: i64 = 233;
    pub const DOM_VK_WIN_OEM_JUMP: i64 = 234;
    pub const DOM_VK_WIN_OEM_PA1: i64 = 235;
    pub const DOM_VK_WIN_OEM_PA2: i64 = 236;
    pub const DOM_VK_WIN_OEM_PA3: i64 = 237;
    pub const DOM_VK_WIN_OEM_WSCTRL: i64 = 238;
    pub const DOM_VK_WIN_OEM_CUSEL: i64 = 239;
    pub const DOM_VK_WIN_OEM_ATTN: i64 = 240;
    pub const DOM_VK_WIN_OEM_FINISH: i64 = 241;
    pub const DOM_VK_WIN_OEM_COPY: i64 = 242;
    pub const DOM_VK_WIN_OEM_AUTO: i64 = 243;
    pub const DOM_VK_WIN_OEM_ENLW: i64 = 244;
    pub const DOM_VK_WIN_OEM_BACKTAB: i64 = 245;
    pub const DOM_VK_ATTN: i64 = 246;
    pub const DOM_VK_CRSEL: i64 = 247;
    pub const DOM_VK_EXSEL: i64 = 248;
    pub const DOM_VK_EREOF: i64 = 249;
    pub const DOM_VK_PLAY: i64 = 250;
    pub const DOM_VK_ZOOM: i64 = 251;
    pub const DOM_VK_PA1: i64 = 253;
    pub const DOM_VK_WIN_OEM_CLEAR: i64 = 254;
    pub const DOM_KEY_LOCATION_STANDARD: i64 = 0;
    pub const DOM_KEY_LOCATION_LEFT: i64 = 1;
    pub const DOM_KEY_LOCATION_RIGHT: i64 = 2;
    pub const DOM_KEY_LOCATION_NUMPAD: i64 = 3;
}


#[repr(C)]
pub struct nsIDOMKeyEvent {
    vtable: *const nsIDOMKeyEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMKeyEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2e52eb99, 0x670d, 0x469a,
            [0xb5, 0x1f, 0x8e, 0xfe, 0xe2, 0xdd, 0x09, 0x1d])
    }
}

unsafe impl RefCounted for nsIDOMKeyEvent {
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
pub trait nsIDOMKeyEventCoerce {
    fn coerce_from(v: &nsIDOMKeyEvent) -> &Self;
}

impl nsIDOMKeyEventCoerce for nsIDOMKeyEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMKeyEvent) -> &Self {
        v
    }
}

impl nsIDOMKeyEvent {
    #[inline]
    pub fn coerce<T: nsIDOMKeyEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMKeyEvent {
    type Target = nsIDOMUIEvent;
    #[inline]
    fn deref(&self) -> &nsIDOMUIEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMUIEventCoerce> nsIDOMKeyEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMKeyEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMKeyEventVTable {
    pub __base: nsIDOMUIEventVTable,

    /* readonly attribute unsigned long charCode; */
    pub get_charCode: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, aCharCode: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long keyCode; */
    pub get_keyCode: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, aKeyCode: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean altKey; */
    pub get_altKey: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, aAltKey: *mut bool) -> nsresult,

    /* readonly attribute boolean ctrlKey; */
    pub get_ctrlKey: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, aCtrlKey: *mut bool) -> nsresult,

    /* readonly attribute boolean shiftKey; */
    pub get_shiftKey: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, aShiftKey: *mut bool) -> nsresult,

    /* readonly attribute boolean metaKey; */
    pub get_metaKey: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, aMetaKey: *mut bool) -> nsresult,

    /* void initKeyEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in boolean ctrlKeyArg, in boolean altKeyArg, in boolean shiftKeyArg, in boolean metaKeyArg, in unsigned long keyCodeArg, in unsigned long charCodeArg); */
    pub initKeyEvent: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, typeArg: *const nsAString, canBubbleArg: bool, cancelableArg: bool, viewArg: *const mozIDOMWindow, ctrlKeyArg: bool, altKeyArg: bool, shiftKeyArg: bool, metaKeyArg: bool, keyCodeArg: libc::uint32_t, charCodeArg: libc::uint32_t) -> nsresult,

    /* bool getModifierState (in DOMString keyArg); */
    pub getModifierState: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, keyArg: *const nsAString, _retval: *mut bool) -> nsresult,

    /* readonly attribute unsigned long location; */
    pub get_location: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, aLocation: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean repeat; */
    pub get_repeat: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, aRepeat: *mut bool) -> nsresult,

    /* readonly attribute DOMString key; */
    pub get_key: unsafe extern "C" fn (this: *const nsIDOMKeyEvent, aKey: *mut nsAString) -> nsresult,

}


impl nsIDOMKeyEvent {
    /* readonly attribute unsigned long charCode; */
    #[inline]
    pub unsafe fn get_charCode(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_charCode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long keyCode; */
    #[inline]
    pub unsafe fn get_keyCode(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_keyCode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean altKey; */
    #[inline]
    pub unsafe fn get_altKey(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_altKey)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean ctrlKey; */
    #[inline]
    pub unsafe fn get_ctrlKey(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_ctrlKey)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean shiftKey; */
    #[inline]
    pub unsafe fn get_shiftKey(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_shiftKey)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean metaKey; */
    #[inline]
    pub unsafe fn get_metaKey(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_metaKey)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void initKeyEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in boolean ctrlKeyArg, in boolean altKeyArg, in boolean shiftKeyArg, in boolean metaKeyArg, in unsigned long keyCodeArg, in unsigned long charCodeArg); */
    #[inline]
    pub unsafe fn initKeyEvent(&self, typeArg: &[u16], canBubbleArg: bool, cancelableArg: bool, viewArg: Option<&mozIDOMWindow>, ctrlKeyArg: bool, altKeyArg: bool, shiftKeyArg: bool, metaKeyArg: bool, keyCodeArg: libc::uint32_t, charCodeArg: libc::uint32_t) -> Result<(), nsresult> {
        let typeArg = nsString::from(typeArg);
        match ((*self.vtable).initKeyEvent)(self as *const _, &*typeArg, canBubbleArg, cancelableArg, viewArg.map_or(::std::ptr::null(), |x| x as *const _), ctrlKeyArg, altKeyArg, shiftKeyArg, metaKeyArg, keyCodeArg, charCodeArg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool getModifierState (in DOMString keyArg); */
    #[inline]
    pub unsafe fn getModifierState(&self, keyArg: &[u16]) -> Result<bool, nsresult> {
        let keyArg = nsString::from(keyArg);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getModifierState)(self as *const _, &*keyArg, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long location; */
    #[inline]
    pub unsafe fn get_location(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_location)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean repeat; */
    #[inline]
    pub unsafe fn get_repeat(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_repeat)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString key; */
    #[inline]
    pub unsafe fn get_key(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_key)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


