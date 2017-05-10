//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheEntryOpenCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheEntryOpenCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* unsigned long onCacheEntryCheck (in nsICacheEntry aEntry, in nsIApplicationCache aApplicationCache); */
                    Method {
                        name: "onCacheEntryCheck",
                        abi: "C",
                        params: &[Param { name: "aEntry", ty: "*const nsICacheEntry" }, Param { name: "aApplicationCache", ty: "*const nsIApplicationCache" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void onCacheEntryAvailable (in nsICacheEntry aEntry, in boolean aNew, in nsIApplicationCache aApplicationCache, in nsresult aResult); */
                    Method {
                        name: "onCacheEntryAvailable",
                        abi: "C",
                        params: &[Param { name: "aEntry", ty: "*const nsICacheEntry" }, Param { name: "aNew", ty: "bool" }, Param { name: "aApplicationCache", ty: "*const nsIApplicationCache" }, Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

