//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICertTree.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICertTreeItem",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIX509Cert cert; */
                    Method {
                        name: "get_cert",
                        abi: "C",
                        params: &[Param { name: "aCert", ty: "*mut *const nsIX509Cert" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString hostPort; */
                    Method {
                        name: "get_hostPort",
                        abi: "C",
                        params: &[Param { name: "aHostPort", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICertTree",
            base: Some("nsITreeView"),
            methods: Some(&[
                    /* void loadCerts (in unsigned long type); */
                    Method {
                        name: "loadCerts",
                        abi: "C",
                        params: &[Param { name: "type_", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void loadCertsFromCache (in nsIX509CertList cache, in unsigned long type); */
                    Method {
                        name: "loadCertsFromCache",
                        abi: "C",
                        params: &[Param { name: "cache", ty: "*const nsIX509CertList" }, Param { name: "type_", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIX509Cert getCert (in unsigned long index); */
                    Method {
                        name: "getCert",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIX509Cert" }],
                        ret: "nsresult",
                    },

                    /* nsICertTreeItem getTreeItem (in unsigned long index); */
                    Method {
                        name: "getTreeItem",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsICertTreeItem" }],
                        ret: "nsresult",
                    },

                    /* void deleteEntryObject (in unsigned long index); */
                    Method {
                        name: "deleteEntryObject",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

