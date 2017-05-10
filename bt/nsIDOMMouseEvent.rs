//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMMouseEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMMouseEvent",
            base: Some("nsIDOMUIEvent"),
            methods: Some(&[
                    /* readonly attribute long screenX; */
                    Method {
                        name: "get_screenX",
                        abi: "C",
                        params: &[Param { name: "aScreenX", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long screenY; */
                    Method {
                        name: "get_screenY",
                        abi: "C",
                        params: &[Param { name: "aScreenY", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long mozMovementX; */
                    Method {
                        name: "get_mozMovementX",
                        abi: "C",
                        params: &[Param { name: "aMozMovementX", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long mozMovementY; */
                    Method {
                        name: "get_mozMovementY",
                        abi: "C",
                        params: &[Param { name: "aMozMovementY", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long clientX; */
                    Method {
                        name: "get_clientX",
                        abi: "C",
                        params: &[Param { name: "aClientX", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long clientY; */
                    Method {
                        name: "get_clientY",
                        abi: "C",
                        params: &[Param { name: "aClientY", ty: "*mut libc::int32_t" }],
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

                    /* readonly attribute boolean altKey; */
                    Method {
                        name: "get_altKey",
                        abi: "C",
                        params: &[Param { name: "aAltKey", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean metaKey; */
                    Method {
                        name: "get_metaKey",
                        abi: "C",
                        params: &[Param { name: "aMetaKey", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute short button; */
                    Method {
                        name: "get_button",
                        abi: "C",
                        params: &[Param { name: "aButton", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned short buttons; */
                    Method {
                        name: "get_buttons",
                        abi: "C",
                        params: &[Param { name: "aButtons", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMEventTarget relatedTarget; */
                    Method {
                        name: "get_relatedTarget",
                        abi: "C",
                        params: &[Param { name: "aRelatedTarget", ty: "*mut *const nsIDOMEventTarget" }],
                        ret: "nsresult",
                    },

                    /* void initMouseEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in long detailArg, in long screenXArg, in long screenYArg, in long clientXArg, in long clientYArg, in boolean ctrlKeyArg, in boolean altKeyArg, in boolean shiftKeyArg, in boolean metaKeyArg, in unsigned short buttonArg, in nsIDOMEventTarget relatedTargetArg); */
                    Method {
                        name: "initMouseEvent",
                        abi: "C",
                        params: &[Param { name: "typeArg", ty: "*const nsAString" }, Param { name: "canBubbleArg", ty: "bool" }, Param { name: "cancelableArg", ty: "bool" }, Param { name: "viewArg", ty: "*const mozIDOMWindow" }, Param { name: "detailArg", ty: "libc::int32_t" }, Param { name: "screenXArg", ty: "libc::int32_t" }, Param { name: "screenYArg", ty: "libc::int32_t" }, Param { name: "clientXArg", ty: "libc::int32_t" }, Param { name: "clientYArg", ty: "libc::int32_t" }, Param { name: "ctrlKeyArg", ty: "bool" }, Param { name: "altKeyArg", ty: "bool" }, Param { name: "shiftKeyArg", ty: "bool" }, Param { name: "metaKeyArg", ty: "bool" }, Param { name: "buttonArg", ty: "libc::uint16_t" }, Param { name: "relatedTargetArg", ty: "*const nsIDOMEventTarget" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute float mozPressure; */
                    Method {
                        name: "get_mozPressure",
                        abi: "C",
                        params: &[Param { name: "aMozPressure", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned short mozInputSource; */
                    Method {
                        name: "get_mozInputSource",
                        abi: "C",
                        params: &[Param { name: "aMozInputSource", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* bool getModifierState (in DOMString keyArg); */
                    Method {
                        name: "getModifierState",
                        abi: "C",
                        params: &[Param { name: "keyArg", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

