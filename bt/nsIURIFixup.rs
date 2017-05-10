//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIFixup.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURIFixupInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsISupports consumer; */
                    Method {
                        name: "get_consumer",
                        abi: "C",
                        params: &[Param { name: "aConsumer", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_consumer",
                        abi: "C",
                        params: &[Param { name: "aConsumer", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI preferredURI; */
                    Method {
                        name: "get_preferredURI",
                        abi: "C",
                        params: &[Param { name: "aPreferredURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI fixedURI; */
                    Method {
                        name: "get_fixedURI",
                        abi: "C",
                        params: &[Param { name: "aFixedURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString keywordProviderName; */
                    Method {
                        name: "get_keywordProviderName",
                        abi: "C",
                        params: &[Param { name: "aKeywordProviderName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString keywordAsSent; */
                    Method {
                        name: "get_keywordAsSent",
                        abi: "C",
                        params: &[Param { name: "aKeywordAsSent", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean fixupChangedProtocol; */
                    Method {
                        name: "get_fixupChangedProtocol",
                        abi: "C",
                        params: &[Param { name: "aFixupChangedProtocol", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean fixupCreatedAlternateURI; */
                    Method {
                        name: "get_fixupCreatedAlternateURI",
                        abi: "C",
                        params: &[Param { name: "aFixupCreatedAlternateURI", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String originalInput; */
                    Method {
                        name: "get_originalInput",
                        abi: "C",
                        params: &[Param { name: "aOriginalInput", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIURIFixup",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIURI createExposableURI (in nsIURI aURI); */
                    Method {
                        name: "createExposableURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* nsIURI createFixupURI (in AUTF8String aURIText, in unsigned long aFixupFlags, [optional] out nsIInputStream aPostData); */
                    Method {
                        name: "createFixupURI",
                        abi: "C",
                        params: &[Param { name: "aURIText", ty: "*const nsACString" }, Param { name: "aFixupFlags", ty: "libc::uint32_t" }, Param { name: "aPostData", ty: "*mut *const nsIInputStream" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* nsIURIFixupInfo getFixupURIInfo (in AUTF8String aURIText, in unsigned long aFixupFlags, [optional] out nsIInputStream aPostData); */
                    Method {
                        name: "getFixupURIInfo",
                        abi: "C",
                        params: &[Param { name: "aURIText", ty: "*const nsACString" }, Param { name: "aFixupFlags", ty: "libc::uint32_t" }, Param { name: "aPostData", ty: "*mut *const nsIInputStream" }, Param { name: "_retval", ty: "*mut *const nsIURIFixupInfo" }],
                        ret: "nsresult",
                    },

                    /* nsIURIFixupInfo keywordToURI (in AUTF8String aKeyword, [optional] out nsIInputStream aPostData); */
                    Method {
                        name: "keywordToURI",
                        abi: "C",
                        params: &[Param { name: "aKeyword", ty: "*const nsACString" }, Param { name: "aPostData", ty: "*mut *const nsIInputStream" }, Param { name: "_retval", ty: "*mut *const nsIURIFixupInfo" }],
                        ret: "nsresult",
                    },

                    /* bool isDomainWhitelisted (in AUTF8String aDomain, in uint32_t aDotPos); */
                    Method {
                        name: "isDomainWhitelisted",
                        abi: "C",
                        params: &[Param { name: "aDomain", ty: "*const nsACString" }, Param { name: "aDotPos", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

