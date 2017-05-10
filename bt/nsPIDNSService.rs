//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPIDNSService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsPIDNSService",
            base: Some("nsIDNSService"),
            methods: Some(&[
                    /* void init (); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void shutdown (); */
                    Method {
                        name: "shutdown",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* attribute boolean prefetchEnabled; */
                    Method {
                        name: "get_prefetchEnabled",
                        abi: "C",
                        params: &[Param { name: "aPrefetchEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_prefetchEnabled",
                        abi: "C",
                        params: &[Param { name: "aPrefetchEnabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

