//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentSignatureVerifier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentSignatureVerifier",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean verifyContentSignature (in ACString aData, in ACString aContentSignatureHeader, in ACString aCertificateChain, in ACString aName); */
                    Method {
                        name: "verifyContentSignature",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsACString" }, Param { name: "aContentSignatureHeader", ty: "*const nsACString" }, Param { name: "aCertificateChain", ty: "*const nsACString" }, Param { name: "aName", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void createContext (in ACString aData, in ACString aContentSignatureHeader, in ACString aCertificateChain, in ACString aName); */
                    Method {
                        name: "createContext",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsACString" }, Param { name: "aContentSignatureHeader", ty: "*const nsACString" }, Param { name: "aCertificateChain", ty: "*const nsACString" }, Param { name: "aName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void createContextWithoutCertChain (in nsIContentSignatureReceiverCallback aCallback, in ACString aContentSignatureHeader, in ACString aName); */
                    Method {
                        name: "createContextWithoutCertChain",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const nsIContentSignatureReceiverCallback" }, Param { name: "aContentSignatureHeader", ty: "*const nsACString" }, Param { name: "aName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void update (in ACString aData); */
                    Method {
                        name: "update",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean end (); */
                    Method {
                        name: "end",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIContentSignatureReceiverCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void contextCreated (in boolean successful); */
                    Method {
                        name: "contextCreated",
                        abi: "C",
                        params: &[Param { name: "successful", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

