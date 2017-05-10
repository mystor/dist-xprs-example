//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULControlElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULControlElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean disabled; */
                    Method {
                        name: "get_disabled",
                        abi: "C",
                        params: &[Param { name: "aDisabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_disabled",
                        abi: "C",
                        params: &[Param { name: "aDisabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute long tabIndex; */
                    Method {
                        name: "get_tabIndex",
                        abi: "C",
                        params: &[Param { name: "aTabIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_tabIndex",
                        abi: "C",
                        params: &[Param { name: "aTabIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

