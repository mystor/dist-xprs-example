//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInputStreamPump.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputStreamPump",
            base: Some("nsIRequest"),
            methods: Some(&[
                    /* void init (in nsIInputStream aStream, in long long aStreamPos, in long long aStreamLen, in unsigned long aSegmentSize, in unsigned long aSegmentCount, in boolean aCloseWhenDone); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aStreamPos", ty: "libc::int64_t" }, Param { name: "aStreamLen", ty: "libc::int64_t" }, Param { name: "aSegmentSize", ty: "libc::uint32_t" }, Param { name: "aSegmentCount", ty: "libc::uint32_t" }, Param { name: "aCloseWhenDone", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void asyncRead (in nsIStreamListener aListener, in nsISupports aListenerContext); */
                    Method {
                        name: "asyncRead",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIStreamListener" }, Param { name: "aListenerContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

