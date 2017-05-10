//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheStorage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheStorage",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void asyncOpenURI (in nsIURI aURI, in ACString aIdExtension, in uint32_t aFlags, in nsICacheEntryOpenCallback aCallback); */
                    Method {
                        name: "asyncOpenURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdExtension", ty: "*const nsACString" }, Param { name: "aFlags", ty: "uint32_t" }, Param { name: "aCallback", ty: "*const nsICacheEntryOpenCallback" }],
                        ret: "nsresult",
                    },

                    /* nsICacheEntry openTruncate (in nsIURI aURI, in ACString aIdExtension); */
                    Method {
                        name: "openTruncate",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdExtension", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsICacheEntry" }],
                        ret: "nsresult",
                    },

                    /* boolean exists (in nsIURI aURI, in ACString aIdExtension); */
                    Method {
                        name: "exists",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdExtension", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void getCacheIndexEntryAttrs (in nsIURI aURI, in ACString aIdExtension, out bool aHasAltData, out uint32_t aSizeInKB); */
                    Method {
                        name: "getCacheIndexEntryAttrs",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdExtension", ty: "*const nsACString" }, Param { name: "aHasAltData", ty: "*mut bool" }, Param { name: "aSizeInKB", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void asyncDoomURI (in nsIURI aURI, in ACString aIdExtension, in nsICacheEntryDoomCallback aCallback); */
                    Method {
                        name: "asyncDoomURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIdExtension", ty: "*const nsACString" }, Param { name: "aCallback", ty: "*const nsICacheEntryDoomCallback" }],
                        ret: "nsresult",
                    },

                    /* void asyncEvictStorage (in nsICacheEntryDoomCallback aCallback); */
                    Method {
                        name: "asyncEvictStorage",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const nsICacheEntryDoomCallback" }],
                        ret: "nsresult",
                    },

                    /* void asyncVisitStorage (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
                    Method {
                        name: "asyncVisitStorage",
                        abi: "C",
                        params: &[Param { name: "aVisitor", ty: "*const nsICacheStorageVisitor" }, Param { name: "aVisitEntries", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

