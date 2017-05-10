//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULCommandEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULCommandEvent",
            base: Some("nsIDOMUIEvent"),
            methods: Some(&[
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

                    /* readonly attribute nsIDOMEvent sourceEvent; */
                    Method {
                        name: "get_sourceEvent",
                        abi: "C",
                        params: &[Param { name: "aSourceEvent", ty: "*mut *const nsIDOMEvent" }],
                        ret: "nsresult",
                    },

                    /* void initCommandEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in long detailArg, in boolean ctrlKeyArg, in boolean altKeyArg, in boolean shiftKeyArg, in boolean metaKeyArg, in nsIDOMEvent sourceEvent); */
                    Method {
                        name: "initCommandEvent",
                        abi: "C",
                        params: &[Param { name: "typeArg", ty: "*const nsAString" }, Param { name: "canBubbleArg", ty: "bool" }, Param { name: "cancelableArg", ty: "bool" }, Param { name: "viewArg", ty: "*const mozIDOMWindow" }, Param { name: "detailArg", ty: "libc::int32_t" }, Param { name: "ctrlKeyArg", ty: "bool" }, Param { name: "altKeyArg", ty: "bool" }, Param { name: "shiftKeyArg", ty: "bool" }, Param { name: "metaKeyArg", ty: "bool" }, Param { name: "sourceEvent", ty: "*const nsIDOMEvent" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

