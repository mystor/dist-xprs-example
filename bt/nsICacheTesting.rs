//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheTesting.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheTesting",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void suspendCacheIOThread (in uint32_t aLevel); */
                    Method {
                        name: "suspendCacheIOThread",
                        abi: "C",
                        params: &[Param { name: "aLevel", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void resumeCacheIOThread (); */
                    Method {
                        name: "resumeCacheIOThread",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void flush (in nsIObserver aObserver); */
                    Method {
                        name: "flush",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

