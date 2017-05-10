//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocShellLoadInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocShellLoadInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIURI referrer; */
                    Method {
                        name: "get_referrer",
                        abi: "C",
                        params: &[Param { name: "aReferrer", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_referrer",
                        abi: "C",
                        params: &[Param { name: "aReferrer", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIURI originalURI; */
                    Method {
                        name: "get_originalURI",
                        abi: "C",
                        params: &[Param { name: "aOriginalURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_originalURI",
                        abi: "C",
                        params: &[Param { name: "aOriginalURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean loadReplace; */
                    Method {
                        name: "get_loadReplace",
                        abi: "C",
                        params: &[Param { name: "aLoadReplace", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_loadReplace",
                        abi: "C",
                        params: &[Param { name: "aLoadReplace", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIPrincipal triggeringPrincipal; */
                    Method {
                        name: "get_triggeringPrincipal",
                        abi: "C",
                        params: &[Param { name: "aTriggeringPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_triggeringPrincipal",
                        abi: "C",
                        params: &[Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean inheritPrincipal; */
                    Method {
                        name: "get_inheritPrincipal",
                        abi: "C",
                        params: &[Param { name: "aInheritPrincipal", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_inheritPrincipal",
                        abi: "C",
                        params: &[Param { name: "aInheritPrincipal", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean principalIsExplicit; */
                    Method {
                        name: "get_principalIsExplicit",
                        abi: "C",
                        params: &[Param { name: "aPrincipalIsExplicit", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_principalIsExplicit",
                        abi: "C",
                        params: &[Param { name: "aPrincipalIsExplicit", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute nsDocShellInfoLoadType loadType; */
                    Method {
                        name: "get_loadType",
                        abi: "C",
                        params: &[Param { name: "aLoadType", ty: "*mut nsDocShellInfoLoadType" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_loadType",
                        abi: "C",
                        params: &[Param { name: "aLoadType", ty: "nsDocShellInfoLoadType" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISHEntry SHEntry; */
                    Method {
                        name: "get_SHEntry",
                        abi: "C",
                        params: &[Param { name: "aSHEntry", ty: "*mut *const nsISHEntry" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_SHEntry",
                        abi: "C",
                        params: &[Param { name: "aSHEntry", ty: "*const nsISHEntry" }],
                        ret: "nsresult",
                    },

                    /* attribute wstring target; */
                    Method {
                        name: "get_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIInputStream postDataStream; */
                    Method {
                        name: "get_postDataStream",
                        abi: "C",
                        params: &[Param { name: "aPostDataStream", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_postDataStream",
                        abi: "C",
                        params: &[Param { name: "aPostDataStream", ty: "*const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIInputStream headersStream; */
                    Method {
                        name: "get_headersStream",
                        abi: "C",
                        params: &[Param { name: "aHeadersStream", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_headersStream",
                        abi: "C",
                        params: &[Param { name: "aHeadersStream", ty: "*const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean sendReferrer; */
                    Method {
                        name: "get_sendReferrer",
                        abi: "C",
                        params: &[Param { name: "aSendReferrer", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sendReferrer",
                        abi: "C",
                        params: &[Param { name: "aSendReferrer", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute nsDocShellInfoReferrerPolicy referrerPolicy; */
                    Method {
                        name: "get_referrerPolicy",
                        abi: "C",
                        params: &[Param { name: "aReferrerPolicy", ty: "*mut nsDocShellInfoReferrerPolicy" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_referrerPolicy",
                        abi: "C",
                        params: &[Param { name: "aReferrerPolicy", ty: "nsDocShellInfoReferrerPolicy" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isSrcdocLoad; */
                    Method {
                        name: "get_isSrcdocLoad",
                        abi: "C",
                        params: &[Param { name: "aIsSrcdocLoad", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute AString srcdocData; */
                    Method {
                        name: "get_srcdocData",
                        abi: "C",
                        params: &[Param { name: "aSrcdocData", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_srcdocData",
                        abi: "C",
                        params: &[Param { name: "aSrcdocData", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIDocShell sourceDocShell; */
                    Method {
                        name: "get_sourceDocShell",
                        abi: "C",
                        params: &[Param { name: "aSourceDocShell", ty: "*mut *const nsIDocShell" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sourceDocShell",
                        abi: "C",
                        params: &[Param { name: "aSourceDocShell", ty: "*const nsIDocShell" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIURI baseURI; */
                    Method {
                        name: "get_baseURI",
                        abi: "C",
                        params: &[Param { name: "aBaseURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_baseURI",
                        abi: "C",
                        params: &[Param { name: "aBaseURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

