//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISupportsPriority.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISupportsPriority",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute long priority; */
                    Method {
                        name: "get_priority",
                        abi: "C",
                        params: &[Param { name: "aPriority", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_priority",
                        abi: "C",
                        params: &[Param { name: "aPriority", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void adjustPriority (in long delta); */
                    Method {
                        name: "adjustPriority",
                        abi: "C",
                        params: &[Param { name: "delta", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

