//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsICacheSession createSession (in string clientID, in nsCacheStoragePolicy storagePolicy, in boolean streamBased); */
                    Method {
                        name: "createSession",
                        abi: "C",
                        params: &[Param { name: "clientID", ty: "*const libc::c_char" }, Param { name: "storagePolicy", ty: "nsCacheStoragePolicy" }, Param { name: "streamBased", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsICacheSession" }],
                        ret: "nsresult",
                    },

                    /* void visitEntries (in nsICacheVisitor visitor); */
                    Method {
                        name: "visitEntries",
                        abi: "C",
                        params: &[Param { name: "visitor", ty: "*const nsICacheVisitor" }],
                        ret: "nsresult",
                    },

                    /* void evictEntries (in nsCacheStoragePolicy storagePolicy); */
                    Method {
                        name: "evictEntries",
                        abi: "C",
                        params: &[Param { name: "storagePolicy", ty: "nsCacheStoragePolicy" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIEventTarget cacheIOTarget; */
                    Method {
                        name: "get_cacheIOTarget",
                        abi: "C",
                        params: &[Param { name: "aCacheIOTarget", ty: "*mut *const nsIEventTarget" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICacheServiceInternal",
            base: Some("nsICacheService"),
            methods: Some(&[
                    /* readonly attribute double lockHeldTime; */
                    Method {
                        name: "get_lockHeldTime",
                        abi: "C",
                        params: &[Param { name: "aLockHeldTime", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

