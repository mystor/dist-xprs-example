//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDiskSpaceWatcher.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDiskSpaceWatcher",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute bool isDiskFull; */
                    Method {
                        name: "get_isDiskFull",
                        abi: "C",
                        params: &[Param { name: "aIsDiskFull", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long long freeSpace; */
                    Method {
                        name: "get_freeSpace",
                        abi: "C",
                        params: &[Param { name: "aFreeSpace", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

