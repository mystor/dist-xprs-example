//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetUtil.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINetUtil",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AUTF8String parseRequestContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset); */
                    Method {
                        name: "parseRequestContentType",
                        abi: "C",
                        params: &[Param { name: "aTypeHeader", ty: "*const nsACString" }, Param { name: "aCharset", ty: "*mut nsACString" }, Param { name: "aHadCharset", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String parseResponseContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset); */
                    Method {
                        name: "parseResponseContentType",
                        abi: "C",
                        params: &[Param { name: "aTypeHeader", ty: "*const nsACString" }, Param { name: "aCharset", ty: "*mut nsACString" }, Param { name: "aHadCharset", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean protocolHasFlags (in nsIURI aURI, in unsigned long aFlag); */
                    Method {
                        name: "protocolHasFlags",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aFlag", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean URIChainHasFlags (in nsIURI aURI, in unsigned long aFlags); */
                    Method {
                        name: "URIChainHasFlags",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIURI toImmutableURI (in nsIURI aURI); */
                    Method {
                        name: "toImmutableURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* nsIURI newSimpleNestedURI (in nsIURI aURI); */
                    Method {
                        name: "newSimpleNestedURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* ACString escapeString (in ACString aString, in unsigned long aEscapeType); */
                    Method {
                        name: "escapeString",
                        abi: "C",
                        params: &[Param { name: "aString", ty: "*const nsACString" }, Param { name: "aEscapeType", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString escapeURL (in ACString aStr, in unsigned long aFlags); */
                    Method {
                        name: "escapeURL",
                        abi: "C",
                        params: &[Param { name: "aStr", ty: "*const nsACString" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString unescapeString (in AUTF8String aStr, in unsigned long aFlags); */
                    Method {
                        name: "unescapeString",
                        abi: "C",
                        params: &[Param { name: "aStr", ty: "*const nsACString" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean extractCharsetFromContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out long aCharsetStart, out long aCharsetEnd); */
                    Method {
                        name: "extractCharsetFromContentType",
                        abi: "C",
                        params: &[Param { name: "aTypeHeader", ty: "*const nsACString" }, Param { name: "aCharset", ty: "*mut nsACString" }, Param { name: "aCharsetStart", ty: "*mut libc::int32_t" }, Param { name: "aCharsetEnd", ty: "*mut libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* unsigned long parseAttributePolicyString (in AString aPolicyString); */
                    Method {
                        name: "parseAttributePolicyString",
                        abi: "C",
                        params: &[Param { name: "aPolicyString", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

