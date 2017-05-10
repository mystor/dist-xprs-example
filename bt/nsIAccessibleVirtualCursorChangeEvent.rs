//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleVirtualCursorChangeEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleVirtualCursorChangeEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Some(&[
                    /* readonly attribute nsIAccessible oldAccessible; */
                    Method {
                        name: "get_oldAccessible",
                        abi: "C",
                        params: &[Param { name: "aOldAccessible", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long oldStartOffset; */
                    Method {
                        name: "get_oldStartOffset",
                        abi: "C",
                        params: &[Param { name: "aOldStartOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long oldEndOffset; */
                    Method {
                        name: "get_oldEndOffset",
                        abi: "C",
                        params: &[Param { name: "aOldEndOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute short reason; */
                    Method {
                        name: "get_reason",
                        abi: "C",
                        params: &[Param { name: "aReason", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

