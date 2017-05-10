//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIX509CertDB.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIOpenSignedAppFileCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void openSignedAppFileFinished (in nsresult rv, in nsIZipReader aZipReader, in nsIX509Cert aSignerCert); */
                    Method {
                        name: "openSignedAppFileFinished",
                        abi: "C",
                        params: &[Param { name: "rv", ty: "nsresult" }, Param { name: "aZipReader", ty: "*const nsIZipReader" }, Param { name: "aSignerCert", ty: "*const nsIX509Cert" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIVerifySignedDirectoryCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void verifySignedDirectoryFinished (in nsresult rv, in nsIX509Cert aSignerCert); */
                    Method {
                        name: "verifySignedDirectoryFinished",
                        abi: "C",
                        params: &[Param { name: "rv", ty: "nsresult" }, Param { name: "aSignerCert", ty: "*const nsIX509Cert" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICertVerificationCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void verifyCertFinished (in int32_t aPRErrorCode, in nsIX509CertList aVerifiedChain, in bool aHasEVPolicy); */
                    Method {
                        name: "verifyCertFinished",
                        abi: "C",
                        params: &[Param { name: "aPRErrorCode", ty: "int32_t" }, Param { name: "aVerifiedChain", ty: "*const nsIX509CertList" }, Param { name: "aHasEVPolicy", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIX509CertDB",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

