//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIChannel",
            base: Some("nsIRequest"),
            methods: Some(&[
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

                    /* readonly attribute nsIURI URI; */
                    Method {
                        name: "get_URI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISupports owner; */
                    Method {
                        name: "get_owner",
                        abi: "C",
                        params: &[Param { name: "aOwner", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_owner",
                        abi: "C",
                        params: &[Param { name: "aOwner", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIInterfaceRequestor notificationCallbacks; */
                    Method {
                        name: "get_notificationCallbacks",
                        abi: "C",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*mut *const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_notificationCallbacks",
                        abi: "C",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISupports securityInfo; */
                    Method {
                        name: "get_securityInfo",
                        abi: "C",
                        params: &[Param { name: "aSecurityInfo", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* attribute ACString contentType; */
                    Method {
                        name: "get_contentType",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contentType",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute ACString contentCharset; */
                    Method {
                        name: "get_contentCharset",
                        abi: "C",
                        params: &[Param { name: "aContentCharset", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contentCharset",
                        abi: "C",
                        params: &[Param { name: "aContentCharset", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute int64_t contentLength; */
                    Method {
                        name: "get_contentLength",
                        abi: "C",
                        params: &[Param { name: "aContentLength", ty: "*mut int64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contentLength",
                        abi: "C",
                        params: &[Param { name: "aContentLength", ty: "int64_t" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream open (); */
                    Method {
                        name: "open",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream open2 (); */
                    Method {
                        name: "open2",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* void asyncOpen (in nsIStreamListener aListener, in nsISupports aContext); */
                    Method {
                        name: "asyncOpen",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIStreamListener" }, Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void asyncOpen2 (in nsIStreamListener aListener); */
                    Method {
                        name: "asyncOpen2",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIStreamListener" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long contentDisposition; */
                    Method {
                        name: "get_contentDisposition",
                        abi: "C",
                        params: &[Param { name: "aContentDisposition", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contentDisposition",
                        abi: "C",
                        params: &[Param { name: "aContentDisposition", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AString contentDispositionFilename; */
                    Method {
                        name: "get_contentDispositionFilename",
                        abi: "C",
                        params: &[Param { name: "aContentDispositionFilename", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contentDispositionFilename",
                        abi: "C",
                        params: &[Param { name: "aContentDispositionFilename", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString contentDispositionHeader; */
                    Method {
                        name: "get_contentDispositionHeader",
                        abi: "C",
                        params: &[Param { name: "aContentDispositionHeader", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsILoadInfo loadInfo; */
                    Method {
                        name: "get_loadInfo",
                        abi: "C",
                        params: &[Param { name: "aLoadInfo", ty: "*mut *const nsILoadInfo" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_loadInfo",
                        abi: "C",
                        params: &[Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool isDocument; */
                    Method {
                        name: "get_isDocument",
                        abi: "C",
                        params: &[Param { name: "aIsDocument", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

