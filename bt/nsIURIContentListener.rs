//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIContentListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURIContentListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean onStartURIOpen (in nsIURI aURI); */
                    Method {
                        name: "onStartURIOpen",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean doContent (in ACString aContentType, in boolean aIsContentPreferred, in nsIRequest aRequest, out nsIStreamListener aContentHandler); */
                    Method {
                        name: "doContent",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*const nsACString" }, Param { name: "aIsContentPreferred", ty: "bool" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContentHandler", ty: "*mut *const nsIStreamListener" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isPreferred (in string aContentType, out string aDesiredContentType); */
                    Method {
                        name: "isPreferred",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*const libc::c_char" }, Param { name: "aDesiredContentType", ty: "*mut *const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean canHandleContent (in string aContentType, in boolean aIsContentPreferred, out string aDesiredContentType); */
                    Method {
                        name: "canHandleContent",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*const libc::c_char" }, Param { name: "aIsContentPreferred", ty: "bool" }, Param { name: "aDesiredContentType", ty: "*mut *const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISupports loadCookie; */
                    Method {
                        name: "get_loadCookie",
                        abi: "C",
                        params: &[Param { name: "aLoadCookie", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_loadCookie",
                        abi: "C",
                        params: &[Param { name: "aLoadCookie", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIURIContentListener parentContentListener; */
                    Method {
                        name: "get_parentContentListener",
                        abi: "C",
                        params: &[Param { name: "aParentContentListener", ty: "*mut *const nsIURIContentListener" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_parentContentListener",
                        abi: "C",
                        params: &[Param { name: "aParentContentListener", ty: "*const nsIURIContentListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

