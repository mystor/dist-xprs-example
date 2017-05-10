//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserPersistDocument.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserPersistURIMap",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long numMappedURIs; */
                    Method {
                        name: "get_numMappedURIs",
                        abi: "C",
                        params: &[Param { name: "aNumMappedURIs", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void getURIMapping (in unsigned long aIndex, out AUTF8String aMapFrom, out AUTF8String aMapTo); */
                    Method {
                        name: "getURIMapping",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::uint32_t" }, Param { name: "aMapFrom", ty: "*mut nsACString" }, Param { name: "aMapTo", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String targetBaseURI; */
                    Method {
                        name: "get_targetBaseURI",
                        abi: "C",
                        params: &[Param { name: "aTargetBaseURI", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWebBrowserPersistDocument",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean isPrivate; */
                    Method {
                        name: "get_isPrivate",
                        abi: "C",
                        params: &[Param { name: "aIsPrivate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String documentURI; */
                    Method {
                        name: "get_documentURI",
                        abi: "C",
                        params: &[Param { name: "aDocumentURI", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String baseURI; */
                    Method {
                        name: "get_baseURI",
                        abi: "C",
                        params: &[Param { name: "aBaseURI", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString contentType; */
                    Method {
                        name: "get_contentType",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString characterSet; */
                    Method {
                        name: "get_characterSet",
                        abi: "C",
                        params: &[Param { name: "aCharacterSet", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString title; */
                    Method {
                        name: "get_title",
                        abi: "C",
                        params: &[Param { name: "aTitle", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString referrer; */
                    Method {
                        name: "get_referrer",
                        abi: "C",
                        params: &[Param { name: "aReferrer", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString contentDisposition; */
                    Method {
                        name: "get_contentDisposition",
                        abi: "C",
                        params: &[Param { name: "aContentDisposition", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIInputStream postData; */
                    Method {
                        name: "get_postData",
                        abi: "C",
                        params: &[Param { name: "aPostData", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long cacheKey; */
                    Method {
                        name: "get_cacheKey",
                        abi: "C",
                        params: &[Param { name: "aCacheKey", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long persistFlags; */
                    Method {
                        name: "get_persistFlags",
                        abi: "C",
                        params: &[Param { name: "aPersistFlags", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_persistFlags",
                        abi: "C",
                        params: &[Param { name: "aPersistFlags", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void readResources (in nsIWebBrowserPersistResourceVisitor aVisitor); */
                    Method {
                        name: "readResources",
                        abi: "C",
                        params: &[Param { name: "aVisitor", ty: "*const nsIWebBrowserPersistResourceVisitor" }],
                        ret: "nsresult",
                    },

                    /* void writeContent (in nsIOutputStream aStream, in nsIWebBrowserPersistURIMap aURIMap, in ACString aRequestedContentType, in unsigned long aEncoderFlags, in unsigned long aWrapColumn, in nsIWebBrowserPersistWriteCompletion aCompletion); */
                    Method {
                        name: "writeContent",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIOutputStream" }, Param { name: "aURIMap", ty: "*const nsIWebBrowserPersistURIMap" }, Param { name: "aRequestedContentType", ty: "*const nsACString" }, Param { name: "aEncoderFlags", ty: "libc::uint32_t" }, Param { name: "aWrapColumn", ty: "libc::uint32_t" }, Param { name: "aCompletion", ty: "*const nsIWebBrowserPersistWriteCompletion" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWebBrowserPersistResourceVisitor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void visitResource (in nsIWebBrowserPersistDocument aDocument, in AUTF8String aURI); */
                    Method {
                        name: "visitResource",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }, Param { name: "aURI", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void visitDocument (in nsIWebBrowserPersistDocument aDocument, in nsIWebBrowserPersistDocument aSubDocument); */
                    Method {
                        name: "visitDocument",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }, Param { name: "aSubDocument", ty: "*const nsIWebBrowserPersistDocument" }],
                        ret: "nsresult",
                    },

                    /* void endVisit (in nsIWebBrowserPersistDocument aDocument, in nsresult aStatus); */
                    Method {
                        name: "endVisit",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }, Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWebBrowserPersistWriteCompletion",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onFinish (in nsIWebBrowserPersistDocument aDocument, in nsIOutputStream aStream, in ACString aContentType, in nsresult aStatus); */
                    Method {
                        name: "onFinish",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }, Param { name: "aStream", ty: "*const nsIOutputStream" }, Param { name: "aContentType", ty: "*const nsACString" }, Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWebBrowserPersistDocumentReceiver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onDocumentReady (in nsIWebBrowserPersistDocument aDocument); */
                    Method {
                        name: "onDocumentReady",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }],
                        ret: "nsresult",
                    },

                    /* void onError (in nsresult aFailure); */
                    Method {
                        name: "onError",
                        abi: "C",
                        params: &[Param { name: "aFailure", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

