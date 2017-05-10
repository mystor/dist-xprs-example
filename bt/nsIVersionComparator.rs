//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIVersionComparator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIVersionComparator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* long compare (in ACString A, in ACString B); */
                    Method {
                        name: "compare",
                        abi: "C",
                        params: &[Param { name: "A", ty: "*const nsACString" }, Param { name: "B", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

