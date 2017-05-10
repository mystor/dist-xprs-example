//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIConsoleMessage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIConsoleMessage",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute uint32_t logLevel; */
                    Method {
                        name: "get_logLevel",
                        abi: "C",
                        params: &[Param { name: "aLogLevel", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long timeStamp; */
                    Method {
                        name: "get_timeStamp",
                        abi: "C",
                        params: &[Param { name: "aTimeStamp", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* [binaryname(MessageMoz)] readonly attribute wstring message; */
                    Method {
                        name: "get_MessageMoz",
                        abi: "C",
                        params: &[Param { name: "aMessage", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

