//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransferable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFlavorDataProvider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getFlavorData (in nsITransferable aTransferable, in string aFlavor, out nsISupports aData, out unsigned long aDataLen); */
                    Method {
                        name: "getFlavorData",
                        abi: "C",
                        params: &[Param { name: "aTransferable", ty: "*const nsITransferable" }, Param { name: "aFlavor", ty: "*const libc::c_char" }, Param { name: "aData", ty: "*mut *const nsISupports" }, Param { name: "aDataLen", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsITransferable",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in nsILoadContext aContext); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const nsILoadContext" }],
                        ret: "nsresult",
                    },

                    /* nsIArray flavorsTransferableCanExport (); */
                    Method {
                        name: "flavorsTransferableCanExport",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* void getTransferData (in string aFlavor, out nsISupports aData, out unsigned long aDataLen); */
                    Method {
                        name: "getTransferData",
                        abi: "C",
                        params: &[Param { name: "aFlavor", ty: "*const libc::c_char" }, Param { name: "aData", ty: "*mut *const nsISupports" }, Param { name: "aDataLen", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void getAnyTransferData (out ACString aFlavor, out nsISupports aData, out unsigned long aDataLen); */
                    Method {
                        name: "getAnyTransferData",
                        abi: "C",
                        params: &[Param { name: "aFlavor", ty: "*mut nsACString" }, Param { name: "aData", ty: "*mut *const nsISupports" }, Param { name: "aDataLen", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean isLargeDataSet (); */
                    Method {
                        name: "isLargeDataSet",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIArray flavorsTransferableCanImport (); */
                    Method {
                        name: "flavorsTransferableCanImport",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* void setTransferData (in string aFlavor, in nsISupports aData, in unsigned long aDataLen); */
                    Method {
                        name: "setTransferData",
                        abi: "C",
                        params: &[Param { name: "aFlavor", ty: "*const libc::c_char" }, Param { name: "aData", ty: "*const nsISupports" }, Param { name: "aDataLen", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void addDataFlavor (in string aDataFlavor); */
                    Method {
                        name: "addDataFlavor",
                        abi: "C",
                        params: &[Param { name: "aDataFlavor", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void removeDataFlavor (in string aDataFlavor); */
                    Method {
                        name: "removeDataFlavor",
                        abi: "C",
                        params: &[Param { name: "aDataFlavor", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIFormatConverter converter; */
                    Method {
                        name: "get_converter",
                        abi: "C",
                        params: &[Param { name: "aConverter", ty: "*mut *const nsIFormatConverter" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_converter",
                        abi: "C",
                        params: &[Param { name: "aConverter", ty: "*const nsIFormatConverter" }],
                        ret: "nsresult",
                    },

                    /* [noscript] attribute boolean isPrivateData; */
                    Method {
                        name: "get_isPrivateData",
                        abi: "C",
                        params: &[Param { name: "aIsPrivateData", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_isPrivateData",
                        abi: "C",
                        params: &[Param { name: "aIsPrivateData", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] attribute nsIPrincipal requestingPrincipal; */
                    Method {
                        name: "get_requestingPrincipal",
                        abi: "C",
                        params: &[Param { name: "aRequestingPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_requestingPrincipal",
                        abi: "C",
                        params: &[Param { name: "aRequestingPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* [noscript] attribute nsContentPolicyType contentPolicyType; */
                    Method {
                        name: "get_contentPolicyType",
                        abi: "C",
                        params: &[Param { name: "aContentPolicyType", ty: "*mut nsContentPolicyType" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contentPolicyType",
                        abi: "C",
                        params: &[Param { name: "aContentPolicyType", ty: "nsContentPolicyType" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

