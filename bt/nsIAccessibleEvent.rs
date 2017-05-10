//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleEvent",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long eventType; */
                    Method {
                        name: "get_eventType",
                        abi: "C",
                        params: &[Param { name: "aEventType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessible accessible; */
                    Method {
                        name: "get_accessible",
                        abi: "C",
                        params: &[Param { name: "aAccessible", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessibleDocument accessibleDocument; */
                    Method {
                        name: "get_accessibleDocument",
                        abi: "C",
                        params: &[Param { name: "aAccessibleDocument", ty: "*mut *const nsIAccessibleDocument" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode DOMNode; */
                    Method {
                        name: "get_DOMNode",
                        abi: "C",
                        params: &[Param { name: "aDOMNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isFromUserInput; */
                    Method {
                        name: "get_isFromUserInput",
                        abi: "C",
                        params: &[Param { name: "aIsFromUserInput", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

