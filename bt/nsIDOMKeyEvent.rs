//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMKeyEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMKeyEvent",
            base: Some("nsIDOMUIEvent"),
            methods: Some(&[
                    /* readonly attribute unsigned long charCode; */
                    Method {
                        name: "get_charCode",
                        abi: "C",
                        params: &[Param { name: "aCharCode", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long keyCode; */
                    Method {
                        name: "get_keyCode",
                        abi: "C",
                        params: &[Param { name: "aKeyCode", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean altKey; */
                    Method {
                        name: "get_altKey",
                        abi: "C",
                        params: &[Param { name: "aAltKey", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean ctrlKey; */
                    Method {
                        name: "get_ctrlKey",
                        abi: "C",
                        params: &[Param { name: "aCtrlKey", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean shiftKey; */
                    Method {
                        name: "get_shiftKey",
                        abi: "C",
                        params: &[Param { name: "aShiftKey", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean metaKey; */
                    Method {
                        name: "get_metaKey",
                        abi: "C",
                        params: &[Param { name: "aMetaKey", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void initKeyEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in boolean ctrlKeyArg, in boolean altKeyArg, in boolean shiftKeyArg, in boolean metaKeyArg, in unsigned long keyCodeArg, in unsigned long charCodeArg); */
                    Method {
                        name: "initKeyEvent",
                        abi: "C",
                        params: &[Param { name: "typeArg", ty: "*const nsAString" }, Param { name: "canBubbleArg", ty: "bool" }, Param { name: "cancelableArg", ty: "bool" }, Param { name: "viewArg", ty: "*const mozIDOMWindow" }, Param { name: "ctrlKeyArg", ty: "bool" }, Param { name: "altKeyArg", ty: "bool" }, Param { name: "shiftKeyArg", ty: "bool" }, Param { name: "metaKeyArg", ty: "bool" }, Param { name: "keyCodeArg", ty: "libc::uint32_t" }, Param { name: "charCodeArg", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* bool getModifierState (in DOMString keyArg); */
                    Method {
                        name: "getModifierState",
                        abi: "C",
                        params: &[Param { name: "keyArg", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long location; */
                    Method {
                        name: "get_location",
                        abi: "C",
                        params: &[Param { name: "aLocation", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean repeat; */
                    Method {
                        name: "get_repeat",
                        abi: "C",
                        params: &[Param { name: "aRepeat", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString key; */
                    Method {
                        name: "get_key",
                        abi: "C",
                        params: &[Param { name: "aKey", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

