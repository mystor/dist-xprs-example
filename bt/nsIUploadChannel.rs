//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUploadChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUploadChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength); */
                    Method {
                        name: "setUploadStream",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aContentType", ty: "*const nsACString" }, Param { name: "aContentLength", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIInputStream uploadStream; */
                    Method {
                        name: "get_uploadStream",
                        abi: "C",
                        params: &[Param { name: "aUploadStream", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

