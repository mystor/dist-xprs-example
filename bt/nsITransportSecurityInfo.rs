//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransportSecurityInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITransportSecurityInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long securityState; */
                    Method {
                        name: "get_securityState",
                        abi: "C",
                        params: &[Param { name: "aSecurityState", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute wstring errorMessage; */
                    Method {
                        name: "get_errorMessage",
                        abi: "C",
                        params: &[Param { name: "aErrorMessage", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long errorCode; */
                    Method {
                        name: "get_errorCode",
                        abi: "C",
                        params: &[Param { name: "aErrorCode", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIX509CertList failedCertChain; */
                    Method {
                        name: "get_failedCertChain",
                        abi: "C",
                        params: &[Param { name: "aFailedCertChain", ty: "*mut *const nsIX509CertList" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

