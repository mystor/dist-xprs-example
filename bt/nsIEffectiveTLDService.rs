//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEffectiveTLDService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEffectiveTLDService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* ACString getPublicSuffix (in nsIURI aURI); */
                    Method {
                        name: "getPublicSuffix",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString getBaseDomain (in nsIURI aURI, [optional] in uint32_t aAdditionalParts); */
                    Method {
                        name: "getBaseDomain",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aAdditionalParts", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString getPublicSuffixFromHost (in AUTF8String aHost); */
                    Method {
                        name: "getPublicSuffixFromHost",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString getBaseDomainFromHost (in AUTF8String aHost, [optional] in uint32_t aAdditionalParts); */
                    Method {
                        name: "getBaseDomainFromHost",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }, Param { name: "aAdditionalParts", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString getNextSubDomain (in AUTF8String aHost); */
                    Method {
                        name: "getNextSubDomain",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

