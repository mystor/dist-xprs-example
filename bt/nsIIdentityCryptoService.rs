//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIdentityCryptoService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIdentityCryptoService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void generateKeyPair (in AUTF8String algorithm, in nsIIdentityKeyGenCallback callback); */
                    Method {
                        name: "generateKeyPair",
                        abi: "C",
                        params: &[Param { name: "algorithm", ty: "*const nsACString" }, Param { name: "callback", ty: "*const nsIIdentityKeyGenCallback" }],
                        ret: "nsresult",
                    },

                    /* ACString base64UrlEncode (in AUTF8String toEncode); */
                    Method {
                        name: "base64UrlEncode",
                        abi: "C",
                        params: &[Param { name: "toEncode", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIIdentityKeyPair",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String keyType; */
                    Method {
                        name: "get_keyType",
                        abi: "C",
                        params: &[Param { name: "aKeyType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String hexRSAPublicKeyExponent; */
                    Method {
                        name: "get_hexRSAPublicKeyExponent",
                        abi: "C",
                        params: &[Param { name: "aHexRSAPublicKeyExponent", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String hexRSAPublicKeyModulus; */
                    Method {
                        name: "get_hexRSAPublicKeyModulus",
                        abi: "C",
                        params: &[Param { name: "aHexRSAPublicKeyModulus", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String hexDSAPrime; */
                    Method {
                        name: "get_hexDSAPrime",
                        abi: "C",
                        params: &[Param { name: "aHexDSAPrime", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String hexDSASubPrime; */
                    Method {
                        name: "get_hexDSASubPrime",
                        abi: "C",
                        params: &[Param { name: "aHexDSASubPrime", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String hexDSAGenerator; */
                    Method {
                        name: "get_hexDSAGenerator",
                        abi: "C",
                        params: &[Param { name: "aHexDSAGenerator", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String hexDSAPublicValue; */
                    Method {
                        name: "get_hexDSAPublicValue",
                        abi: "C",
                        params: &[Param { name: "aHexDSAPublicValue", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* void sign (in AUTF8String aText, in nsIIdentitySignCallback callback); */
                    Method {
                        name: "sign",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*const nsACString" }, Param { name: "callback", ty: "*const nsIIdentitySignCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIIdentityKeyGenCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void generateKeyPairFinished (in nsresult rv, in nsIIdentityKeyPair keyPair); */
                    Method {
                        name: "generateKeyPairFinished",
                        abi: "C",
                        params: &[Param { name: "rv", ty: "nsresult" }, Param { name: "keyPair", ty: "*const nsIIdentityKeyPair" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIIdentitySignCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void signFinished (in nsresult rv, in ACString base64urlSignature); */
                    Method {
                        name: "signFinished",
                        abi: "C",
                        params: &[Param { name: "rv", ty: "nsresult" }, Param { name: "base64urlSignature", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

