//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIApplicationCache.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIApplicationCacheNamespace",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in unsigned long itemType, in ACString namespaceSpec, in ACString data); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "itemType", ty: "libc::uint32_t" }, Param { name: "namespaceSpec", ty: "*const nsACString" }, Param { name: "data", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long itemType; */
                    Method {
                        name: "get_itemType",
                        abi: "C",
                        params: &[Param { name: "aItemType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString namespaceSpec; */
                    Method {
                        name: "get_namespaceSpec",
                        abi: "C",
                        params: &[Param { name: "aNamespaceSpec", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIApplicationCache",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

