//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILocalCertService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILocalCertService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getOrCreateCert (in ACString nickname, in nsILocalCertGetCallback cb); */
                    Method {
                        name: "getOrCreateCert",
                        abi: "C",
                        params: &[Param { name: "nickname", ty: "*const nsACString" }, Param { name: "cb", ty: "*const nsILocalCertGetCallback" }],
                        ret: "nsresult",
                    },

                    /* void removeCert (in ACString nickname, in nsILocalCertCallback cb); */
                    Method {
                        name: "removeCert",
                        abi: "C",
                        params: &[Param { name: "nickname", ty: "*const nsACString" }, Param { name: "cb", ty: "*const nsILocalCertCallback" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean loginPromptRequired; */
                    Method {
                        name: "get_loginPromptRequired",
                        abi: "C",
                        params: &[Param { name: "aLoginPromptRequired", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsILocalCertGetCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleCert (in nsIX509Cert cert, in nsresult result); */
                    Method {
                        name: "handleCert",
                        abi: "C",
                        params: &[Param { name: "cert", ty: "*const nsIX509Cert" }, Param { name: "result", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsILocalCertCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleResult (in nsresult result); */
                    Method {
                        name: "handleResult",
                        abi: "C",
                        params: &[Param { name: "result", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

