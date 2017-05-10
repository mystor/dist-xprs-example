//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFocusManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFocusManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute mozIDOMWindowProxy activeWindow; */
                    Method {
                        name: "get_activeWindow",
                        abi: "C",
                        params: &[Param { name: "aActiveWindow", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_activeWindow",
                        abi: "C",
                        params: &[Param { name: "aActiveWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* attribute mozIDOMWindowProxy focusedWindow; */
                    Method {
                        name: "get_focusedWindow",
                        abi: "C",
                        params: &[Param { name: "aFocusedWindow", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_focusedWindow",
                        abi: "C",
                        params: &[Param { name: "aFocusedWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement focusedElement; */
                    Method {
                        name: "get_focusedElement",
                        abi: "C",
                        params: &[Param { name: "aFocusedElement", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* uint32_t getLastFocusMethod (in mozIDOMWindowProxy window); */
                    Method {
                        name: "getLastFocusMethod",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void setFocus (in nsIDOMElement aElement, in unsigned long aFlags); */
                    Method {
                        name: "setFocus",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }, Param { name: "aFlags", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement moveFocus (in mozIDOMWindowProxy aWindow, in nsIDOMElement aStartElement, in unsigned long aType, in unsigned long aFlags); */
                    Method {
                        name: "moveFocus",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "aStartElement", ty: "*const nsIDOMElement" }, Param { name: "aType", ty: "libc::uint32_t" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void clearFocus (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "clearFocus",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMElement getFocusedElementForWindow (in mozIDOMWindowProxy aWindow, in boolean aDeep, out mozIDOMWindowProxy aFocusedWindow); */
                    Method {
                        name: "getFocusedElementForWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "aDeep", ty: "bool" }, Param { name: "aFocusedWindow", ty: "*mut *const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void moveCaretToFocus (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "moveCaretToFocus",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* boolean elementIsFocusable (in nsIDOMElement aElement, in unsigned long aFlags); */
                    Method {
                        name: "elementIsFocusable",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void windowRaised (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "windowRaised",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void windowLowered (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "windowLowered",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void windowShown (in mozIDOMWindowProxy aWindow, in boolean aNeedsFocus); */
                    Method {
                        name: "windowShown",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "aNeedsFocus", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void windowHidden (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "windowHidden",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void fireDelayedEvents (in nsIDocument aDocument); */
                    Method {
                        name: "fireDelayedEvents",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*const nsIDocument" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void focusPlugin (in nsIContent aPlugin); */
                    Method {
                        name: "focusPlugin",
                        abi: "C",
                        params: &[Param { name: "aPlugin", ty: "*const nsIContent" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void parentActivated (in mozIDOMWindowProxy aWindow, in bool active); */
                    Method {
                        name: "parentActivated",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "active", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

