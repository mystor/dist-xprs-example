//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUploadChannel2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUploadChannel2",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void explicitSetUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength, in ACString aMethod, in boolean aStreamHasHeaders); */
                    Method {
                        name: "explicitSetUploadStream",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aContentType", ty: "*const nsACString" }, Param { name: "aContentLength", ty: "libc::int64_t" }, Param { name: "aMethod", ty: "*const nsACString" }, Param { name: "aStreamHasHeaders", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean uploadStreamHasHeaders; */
                    Method {
                        name: "get_uploadStreamHasHeaders",
                        abi: "C",
                        params: &[Param { name: "aUploadStreamHasHeaders", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void ensureUploadStreamIsCloneable (in nsIRunnable aCallback); */
                    Method {
                        name: "ensureUploadStreamIsCloneable",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const nsIRunnable" }],
                        ret: "nsresult",
                    },

                    /* [noscript] nsIInputStream cloneUploadStream (); */
                    Method {
                        name: "cloneUploadStream",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

