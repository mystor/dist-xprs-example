//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICertOverrideService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICertOverrideService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void rememberValidityOverride (in ACString aHostName, in int32_t aPort, in nsIX509Cert aCert, in uint32_t aOverrideBits, in boolean aTemporary); */
                    Method {
                        name: "rememberValidityOverride",
                        abi: "C",
                        params: &[Param { name: "aHostName", ty: "*const nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aCert", ty: "*const nsIX509Cert" }, Param { name: "aOverrideBits", ty: "uint32_t" }, Param { name: "aTemporary", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void rememberTemporaryValidityOverrideUsingFingerprint (in ACString aHostName, in int32_t aPort, in ACString aCertFingerprint, in uint32_t aOverrideBits); */
                    Method {
                        name: "rememberTemporaryValidityOverrideUsingFingerprint",
                        abi: "C",
                        params: &[Param { name: "aHostName", ty: "*const nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aCertFingerprint", ty: "*const nsACString" }, Param { name: "aOverrideBits", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean hasMatchingOverride (in ACString aHostName, in int32_t aPort, in nsIX509Cert aCert, out uint32_t aOverrideBits, out boolean aIsTemporary); */
                    Method {
                        name: "hasMatchingOverride",
                        abi: "C",
                        params: &[Param { name: "aHostName", ty: "*const nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aCert", ty: "*const nsIX509Cert" }, Param { name: "aOverrideBits", ty: "*mut uint32_t" }, Param { name: "aIsTemporary", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean getValidityOverride (in ACString aHostName, in int32_t aPort, out ACString aHashAlg, out ACString aFingerprint, out uint32_t aOverrideBits, out boolean aIsTemporary); */
                    Method {
                        name: "getValidityOverride",
                        abi: "C",
                        params: &[Param { name: "aHostName", ty: "*const nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aHashAlg", ty: "*mut nsACString" }, Param { name: "aFingerprint", ty: "*mut nsACString" }, Param { name: "aOverrideBits", ty: "*mut uint32_t" }, Param { name: "aIsTemporary", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void clearValidityOverride (in ACString aHostName, in int32_t aPort); */
                    Method {
                        name: "clearValidityOverride",
                        abi: "C",
                        params: &[Param { name: "aHostName", ty: "*const nsACString" }, Param { name: "aPort", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    /* uint32_t isCertUsedForOverrides (in nsIX509Cert aCert, in boolean aCheckTemporaries, in boolean aCheckPermanents); */
                    Method {
                        name: "isCertUsedForOverrides",
                        abi: "C",
                        params: &[Param { name: "aCert", ty: "*const nsIX509Cert" }, Param { name: "aCheckTemporaries", ty: "bool" }, Param { name: "aCheckPermanents", ty: "bool" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

