//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBFCacheEntry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBFCacheEntry",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void RemoveFromBFCacheSync (); */
                    Method {
                        name: "RemoveFromBFCacheSync",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void RemoveFromBFCacheAsync (); */
                    Method {
                        name: "RemoveFromBFCacheAsync",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long long ID; */
                    Method {
                        name: "get_ID",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

