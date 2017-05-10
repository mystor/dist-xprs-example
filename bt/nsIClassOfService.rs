//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClassOfService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClassOfService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute unsigned long classFlags; */
                    Method {
                        name: "get_classFlags",
                        abi: "C",
                        params: &[Param { name: "aClassFlags", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_classFlags",
                        abi: "C",
                        params: &[Param { name: "aClassFlags", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void clearClassFlags (in unsigned long flags); */
                    Method {
                        name: "clearClassFlags",
                        abi: "C",
                        params: &[Param { name: "flags", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void addClassFlags (in unsigned long flags); */
                    Method {
                        name: "addClassFlags",
                        abi: "C",
                        params: &[Param { name: "flags", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

