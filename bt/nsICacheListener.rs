//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onCacheEntryAvailable (in nsICacheEntryDescriptor descriptor, in nsCacheAccessMode accessGranted, in nsresult status); */
                    Method {
                        name: "onCacheEntryAvailable",
                        abi: "C",
                        params: &[Param { name: "descriptor", ty: "*const nsICacheEntryDescriptor" }, Param { name: "accessGranted", ty: "nsCacheAccessMode" }, Param { name: "status", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void onCacheEntryDoomed (in nsresult status); */
                    Method {
                        name: "onCacheEntryDoomed",
                        abi: "C",
                        params: &[Param { name: "status", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

