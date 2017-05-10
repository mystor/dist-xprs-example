//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleStateChangeEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleStateChangeEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Some(&[
                    /* readonly attribute unsigned long state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isExtraState; */
                    Method {
                        name: "get_isExtraState",
                        abi: "C",
                        params: &[Param { name: "aIsExtraState", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isEnabled; */
                    Method {
                        name: "get_isEnabled",
                        abi: "C",
                        params: &[Param { name: "aIsEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

