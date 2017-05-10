//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamTransportService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamTransportService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsITransport createInputTransport (in nsIInputStream aStream, in long long aStartOffset, in long long aReadLimit, in boolean aCloseWhenDone); */
                    Method {
                        name: "createInputTransport",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aStartOffset", ty: "libc::int64_t" }, Param { name: "aReadLimit", ty: "libc::int64_t" }, Param { name: "aCloseWhenDone", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsITransport" }],
                        ret: "nsresult",
                    },

                    /* nsITransport createOutputTransport (in nsIOutputStream aStream, in long long aStartOffset, in long long aWriteLimit, in boolean aCloseWhenDone); */
                    Method {
                        name: "createOutputTransport",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIOutputStream" }, Param { name: "aStartOffset", ty: "libc::int64_t" }, Param { name: "aWriteLimit", ty: "libc::int64_t" }, Param { name: "aCloseWhenDone", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsITransport" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

