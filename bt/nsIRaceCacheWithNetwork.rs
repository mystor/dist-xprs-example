//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRaceCacheWithNetwork.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRaceCacheWithNetwork",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void test_triggerNetwork (in long timeout); */
                    Method {
                        name: "test_triggerNetwork",
                        abi: "C",
                        params: &[Param { name: "timeout", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void test_delayCacheEntryOpeningBy (in long timeout); */
                    Method {
                        name: "test_delayCacheEntryOpeningBy",
                        abi: "C",
                        params: &[Param { name: "timeout", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void test_triggerDelayedOpenCacheEntry (); */
                    Method {
                        name: "test_triggerDelayedOpenCacheEntry",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

