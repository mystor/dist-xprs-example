//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISSLStatus.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISSLStatus",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIX509Cert serverCert; */
                    Method {
                        name: "get_serverCert",
                        abi: "C",
                        params: &[Param { name: "aServerCert", ty: "*mut *const nsIX509Cert" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString cipherName; */
                    Method {
                        name: "get_cipherName",
                        abi: "C",
                        params: &[Param { name: "aCipherName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long keyLength; */
                    Method {
                        name: "get_keyLength",
                        abi: "C",
                        params: &[Param { name: "aKeyLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long secretKeyLength; */
                    Method {
                        name: "get_secretKeyLength",
                        abi: "C",
                        params: &[Param { name: "aSecretKeyLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned short protocolVersion; */
                    Method {
                        name: "get_protocolVersion",
                        abi: "C",
                        params: &[Param { name: "aProtocolVersion", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned short certificateTransparencyStatus; */
                    Method {
                        name: "get_certificateTransparencyStatus",
                        abi: "C",
                        params: &[Param { name: "aCertificateTransparencyStatus", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isDomainMismatch; */
                    Method {
                        name: "get_isDomainMismatch",
                        abi: "C",
                        params: &[Param { name: "aIsDomainMismatch", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isNotValidAtThisTime; */
                    Method {
                        name: "get_isNotValidAtThisTime",
                        abi: "C",
                        params: &[Param { name: "aIsNotValidAtThisTime", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isUntrusted; */
                    Method {
                        name: "get_isUntrusted",
                        abi: "C",
                        params: &[Param { name: "aIsUntrusted", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isExtendedValidation; */
                    Method {
                        name: "get_isExtendedValidation",
                        abi: "C",
                        params: &[Param { name: "aIsExtendedValidation", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

