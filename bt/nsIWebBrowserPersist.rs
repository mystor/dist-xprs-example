//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserPersist.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserPersist",
            base: Some("nsICancelable"),
            methods: Some(&[
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

                    /* readonly attribute unsigned long currentState; */
                    Method {
                        name: "get_currentState",
                        abi: "C",
                        params: &[Param { name: "aCurrentState", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsresult result; */
                    Method {
                        name: "get_result",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*mut nsresult" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIWebProgressListener progressListener; */
                    Method {
                        name: "get_progressListener",
                        abi: "C",
                        params: &[Param { name: "aProgressListener", ty: "*mut *const nsIWebProgressListener" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_progressListener",
                        abi: "C",
                        params: &[Param { name: "aProgressListener", ty: "*const nsIWebProgressListener" }],
                        ret: "nsresult",
                    },

                    /* void saveURI (in nsIURI aURI, in nsISupports aCacheKey, in nsIURI aReferrer, in unsigned long aReferrerPolicy, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsILoadContext aPrivacyContext); */
                    Method {
                        name: "saveURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aCacheKey", ty: "*const nsISupports" }, Param { name: "aReferrer", ty: "*const nsIURI" }, Param { name: "aReferrerPolicy", ty: "libc::uint32_t" }, Param { name: "aPostData", ty: "*const nsIInputStream" }, Param { name: "aExtraHeaders", ty: "*const libc::c_char" }, Param { name: "aFile", ty: "*const nsISupports" }, Param { name: "aPrivacyContext", ty: "*const nsILoadContext" }],
                        ret: "nsresult",
                    },

                    /* void savePrivacyAwareURI (in nsIURI aURI, in nsISupports aCacheKey, in nsIURI aReferrer, in unsigned long aReferrerPolicy, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in boolean aIsPrivate); */
                    Method {
                        name: "savePrivacyAwareURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aCacheKey", ty: "*const nsISupports" }, Param { name: "aReferrer", ty: "*const nsIURI" }, Param { name: "aReferrerPolicy", ty: "libc::uint32_t" }, Param { name: "aPostData", ty: "*const nsIInputStream" }, Param { name: "aExtraHeaders", ty: "*const libc::c_char" }, Param { name: "aFile", ty: "*const nsISupports" }, Param { name: "aIsPrivate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void saveChannel (in nsIChannel aChannel, in nsISupports aFile); */
                    Method {
                        name: "saveChannel",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aFile", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void saveDocument (in nsISupports aDocument, in nsISupports aFile, in nsISupports aDataPath, in string aOutputContentType, in unsigned long aEncodingFlags, in unsigned long aWrapColumn); */
                    Method {
                        name: "saveDocument",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*const nsISupports" }, Param { name: "aFile", ty: "*const nsISupports" }, Param { name: "aDataPath", ty: "*const nsISupports" }, Param { name: "aOutputContentType", ty: "*const libc::c_char" }, Param { name: "aEncodingFlags", ty: "libc::uint32_t" }, Param { name: "aWrapColumn", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void cancelSave (); */
                    Method {
                        name: "cancelSave",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

