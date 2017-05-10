//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleRole.idl
//


pub mod nsIAccessibleRole_consts {
    pub const ROLE_NOTHING: i64 = 0;
    pub const ROLE_TITLEBAR: i64 = 1;
    pub const ROLE_MENUBAR: i64 = 2;
    pub const ROLE_SCROLLBAR: i64 = 3;
    pub const ROLE_GRIP: i64 = 4;
    pub const ROLE_SOUND: i64 = 5;
    pub const ROLE_CURSOR: i64 = 6;
    pub const ROLE_CARET: i64 = 7;
    pub const ROLE_ALERT: i64 = 8;
    pub const ROLE_WINDOW: i64 = 9;
    pub const ROLE_INTERNAL_FRAME: i64 = 10;
    pub const ROLE_MENUPOPUP: i64 = 11;
    pub const ROLE_MENUITEM: i64 = 12;
    pub const ROLE_TOOLTIP: i64 = 13;
    pub const ROLE_APPLICATION: i64 = 14;
    pub const ROLE_DOCUMENT: i64 = 15;
    pub const ROLE_PANE: i64 = 16;
    pub const ROLE_CHART: i64 = 17;
    pub const ROLE_DIALOG: i64 = 18;
    pub const ROLE_BORDER: i64 = 19;
    pub const ROLE_GROUPING: i64 = 20;
    pub const ROLE_SEPARATOR: i64 = 21;
    pub const ROLE_TOOLBAR: i64 = 22;
    pub const ROLE_STATUSBAR: i64 = 23;
    pub const ROLE_TABLE: i64 = 24;
    pub const ROLE_COLUMNHEADER: i64 = 25;
    pub const ROLE_ROWHEADER: i64 = 26;
    pub const ROLE_COLUMN: i64 = 27;
    pub const ROLE_ROW: i64 = 28;
    pub const ROLE_CELL: i64 = 29;
    pub const ROLE_LINK: i64 = 30;
    pub const ROLE_HELPBALLOON: i64 = 31;
    pub const ROLE_CHARACTER: i64 = 32;
    pub const ROLE_LIST: i64 = 33;
    pub const ROLE_LISTITEM: i64 = 34;
    pub const ROLE_OUTLINE: i64 = 35;
    pub const ROLE_OUTLINEITEM: i64 = 36;
    pub const ROLE_PAGETAB: i64 = 37;
    pub const ROLE_PROPERTYPAGE: i64 = 38;
    pub const ROLE_INDICATOR: i64 = 39;
    pub const ROLE_GRAPHIC: i64 = 40;
    pub const ROLE_STATICTEXT: i64 = 41;
    pub const ROLE_TEXT_LEAF: i64 = 42;
    pub const ROLE_PUSHBUTTON: i64 = 43;
    pub const ROLE_CHECKBUTTON: i64 = 44;
    pub const ROLE_RADIOBUTTON: i64 = 45;
    pub const ROLE_COMBOBOX: i64 = 46;
    pub const ROLE_DROPLIST: i64 = 47;
    pub const ROLE_PROGRESSBAR: i64 = 48;
    pub const ROLE_DIAL: i64 = 49;
    pub const ROLE_HOTKEYFIELD: i64 = 50;
    pub const ROLE_SLIDER: i64 = 51;
    pub const ROLE_SPINBUTTON: i64 = 52;
    pub const ROLE_DIAGRAM: i64 = 53;
    pub const ROLE_ANIMATION: i64 = 54;
    pub const ROLE_EQUATION: i64 = 55;
    pub const ROLE_BUTTONDROPDOWN: i64 = 56;
    pub const ROLE_BUTTONMENU: i64 = 57;
    pub const ROLE_BUTTONDROPDOWNGRID: i64 = 58;
    pub const ROLE_WHITESPACE: i64 = 59;
    pub const ROLE_PAGETABLIST: i64 = 60;
    pub const ROLE_CLOCK: i64 = 61;
    pub const ROLE_SPLITBUTTON: i64 = 62;
    pub const ROLE_IPADDRESS: i64 = 63;
    pub const ROLE_ACCEL_LABEL: i64 = 64;
    pub const ROLE_ARROW: i64 = 65;
    pub const ROLE_CANVAS: i64 = 66;
    pub const ROLE_CHECK_MENU_ITEM: i64 = 67;
    pub const ROLE_COLOR_CHOOSER: i64 = 68;
    pub const ROLE_DATE_EDITOR: i64 = 69;
    pub const ROLE_DESKTOP_ICON: i64 = 70;
    pub const ROLE_DESKTOP_FRAME: i64 = 71;
    pub const ROLE_DIRECTORY_PANE: i64 = 72;
    pub const ROLE_FILE_CHOOSER: i64 = 73;
    pub const ROLE_FONT_CHOOSER: i64 = 74;
    pub const ROLE_CHROME_WINDOW: i64 = 75;
    pub const ROLE_GLASS_PANE: i64 = 76;
    pub const ROLE_HTML_CONTAINER: i64 = 77;
    pub const ROLE_ICON: i64 = 78;
    pub const ROLE_LABEL: i64 = 79;
    pub const ROLE_LAYERED_PANE: i64 = 80;
    pub const ROLE_OPTION_PANE: i64 = 81;
    pub const ROLE_PASSWORD_TEXT: i64 = 82;
    pub const ROLE_POPUP_MENU: i64 = 83;
    pub const ROLE_RADIO_MENU_ITEM: i64 = 84;
    pub const ROLE_ROOT_PANE: i64 = 85;
    pub const ROLE_SCROLL_PANE: i64 = 86;
    pub const ROLE_SPLIT_PANE: i64 = 87;
    pub const ROLE_TABLE_COLUMN_HEADER: i64 = 88;
    pub const ROLE_TABLE_ROW_HEADER: i64 = 89;
    pub const ROLE_TEAR_OFF_MENU_ITEM: i64 = 90;
    pub const ROLE_TERMINAL: i64 = 91;
    pub const ROLE_TEXT_CONTAINER: i64 = 92;
    pub const ROLE_TOGGLE_BUTTON: i64 = 93;
    pub const ROLE_TREE_TABLE: i64 = 94;
    pub const ROLE_VIEWPORT: i64 = 95;
    pub const ROLE_HEADER: i64 = 96;
    pub const ROLE_FOOTER: i64 = 97;
    pub const ROLE_PARAGRAPH: i64 = 98;
    pub const ROLE_RULER: i64 = 99;
    pub const ROLE_AUTOCOMPLETE: i64 = 100;
    pub const ROLE_EDITBAR: i64 = 101;
    pub const ROLE_ENTRY: i64 = 102;
    pub const ROLE_CAPTION: i64 = 103;
    pub const ROLE_DOCUMENT_FRAME: i64 = 104;
    pub const ROLE_HEADING: i64 = 105;
    pub const ROLE_PAGE: i64 = 106;
    pub const ROLE_SECTION: i64 = 107;
    pub const ROLE_REDUNDANT_OBJECT: i64 = 108;
    pub const ROLE_FORM: i64 = 109;
    pub const ROLE_IME: i64 = 110;
    pub const ROLE_APP_ROOT: i64 = 111;
    pub const ROLE_PARENT_MENUITEM: i64 = 112;
    pub const ROLE_CALENDAR: i64 = 113;
    pub const ROLE_COMBOBOX_LIST: i64 = 114;
    pub const ROLE_COMBOBOX_OPTION: i64 = 115;
    pub const ROLE_IMAGE_MAP: i64 = 116;
    pub const ROLE_OPTION: i64 = 117;
    pub const ROLE_RICH_OPTION: i64 = 118;
    pub const ROLE_LISTBOX: i64 = 119;
    pub const ROLE_FLAT_EQUATION: i64 = 120;
    pub const ROLE_GRID_CELL: i64 = 121;
    pub const ROLE_EMBEDDED_OBJECT: i64 = 122;
    pub const ROLE_NOTE: i64 = 123;
    pub const ROLE_FIGURE: i64 = 124;
    pub const ROLE_CHECK_RICH_OPTION: i64 = 125;
    pub const ROLE_DEFINITION_LIST: i64 = 126;
    pub const ROLE_TERM: i64 = 127;
    pub const ROLE_DEFINITION: i64 = 128;
    pub const ROLE_KEY: i64 = 129;
    pub const ROLE_SWITCH: i64 = 130;
    pub const ROLE_MATHML_MATH: i64 = 131;
    pub const ROLE_MATHML_IDENTIFIER: i64 = 132;
    pub const ROLE_MATHML_NUMBER: i64 = 133;
    pub const ROLE_MATHML_OPERATOR: i64 = 134;
    pub const ROLE_MATHML_TEXT: i64 = 135;
    pub const ROLE_MATHML_STRING_LITERAL: i64 = 136;
    pub const ROLE_MATHML_GLYPH: i64 = 137;
    pub const ROLE_MATHML_ROW: i64 = 138;
    pub const ROLE_MATHML_FRACTION: i64 = 139;
    pub const ROLE_MATHML_SQUARE_ROOT: i64 = 140;
    pub const ROLE_MATHML_ROOT: i64 = 141;
    pub const ROLE_MATHML_FENCED: i64 = 142;
    pub const ROLE_MATHML_ENCLOSED: i64 = 143;
    pub const ROLE_MATHML_STYLE: i64 = 144;
    pub const ROLE_MATHML_SUB: i64 = 145;
    pub const ROLE_MATHML_SUP: i64 = 146;
    pub const ROLE_MATHML_SUB_SUP: i64 = 147;
    pub const ROLE_MATHML_UNDER: i64 = 148;
    pub const ROLE_MATHML_OVER: i64 = 149;
    pub const ROLE_MATHML_UNDER_OVER: i64 = 150;
    pub const ROLE_MATHML_MULTISCRIPTS: i64 = 151;
    pub const ROLE_MATHML_TABLE: i64 = 152;
    pub const ROLE_MATHML_LABELED_ROW: i64 = 153;
    pub const ROLE_MATHML_TABLE_ROW: i64 = 154;
    pub const ROLE_MATHML_CELL: i64 = 155;
    pub const ROLE_MATHML_ACTION: i64 = 156;
    pub const ROLE_MATHML_ERROR: i64 = 157;
    pub const ROLE_MATHML_STACK: i64 = 158;
    pub const ROLE_MATHML_LONG_DIVISION: i64 = 159;
    pub const ROLE_MATHML_STACK_GROUP: i64 = 160;
    pub const ROLE_MATHML_STACK_ROW: i64 = 161;
    pub const ROLE_MATHML_STACK_CARRIES: i64 = 162;
    pub const ROLE_MATHML_STACK_CARRY: i64 = 163;
    pub const ROLE_MATHML_STACK_LINE: i64 = 164;
    pub const ROLE_RADIO_GROUP: i64 = 165;
    pub const ROLE_TEXT: i64 = 166;
    pub const ROLE_DETAILS: i64 = 167;
    pub const ROLE_SUMMARY: i64 = 168;
}


#[repr(C)]
pub struct nsIAccessibleRole {
    vtable: *const nsIAccessibleRoleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleRole {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x05a9f33f, 0xdcfd, 0x4e7b,
            [0xb8, 0x25, 0x13, 0x8b, 0xa7, 0x84, 0xc1, 0xf5])
    }
}

unsafe impl RefCounted for nsIAccessibleRole {
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
pub trait nsIAccessibleRoleCoerce {
    fn coerce_from(v: &nsIAccessibleRole) -> &Self;
}

impl nsIAccessibleRoleCoerce for nsIAccessibleRole {
    #[inline]
    fn coerce_from(v: &nsIAccessibleRole) -> &Self {
        v
    }
}

impl nsIAccessibleRole {
    #[inline]
    pub fn coerce<T: nsIAccessibleRoleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleRole {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleRoleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleRole) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleRoleVTable {
    pub __base: nsISupportsVTable,

}


impl nsIAccessibleRole {
}


