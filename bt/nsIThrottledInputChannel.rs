//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThrottledInputChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputChannelThrottleQueue",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in unsigned long aMeanBytesPerSecond, in unsigned long aMaxBytesPerSecond); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aMeanBytesPerSecond", ty: "libc::uint32_t" }, Param { name: "aMaxBytesPerSecond", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* unsigned long available (in unsigned long aRemaining); */
                    Method {
                        name: "available",
                        abi: "C",
                        params: &[Param { name: "aRemaining", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void recordRead (in unsigned long aBytesRead); */
                    Method {
                        name: "recordRead",
                        abi: "C",
                        params: &[Param { name: "aBytesRead", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* unsigned long long bytesProcessed (); */
                    Method {
                        name: "bytesProcessed",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* nsIAsyncInputStream wrapStream (in nsIInputStream aInputStream); */
                    Method {
                        name: "wrapStream",
                        abi: "C",
                        params: &[Param { name: "aInputStream", ty: "*const nsIInputStream" }, Param { name: "_retval", ty: "*mut *const nsIAsyncInputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIThrottledInputChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIInputChannelThrottleQueue throttleQueue; */
                    Method {
                        name: "get_throttleQueue",
                        abi: "C",
                        params: &[Param { name: "aThrottleQueue", ty: "*mut *const nsIInputChannelThrottleQueue" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_throttleQueue",
                        abi: "C",
                        params: &[Param { name: "aThrottleQueue", ty: "*const nsIInputChannelThrottleQueue" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

