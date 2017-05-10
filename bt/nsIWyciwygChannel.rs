//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWyciwygChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWyciwygChannel",
            base: Some("nsIChannel"),
            methods: Some(&[
                    /* void writeToCacheEntry (in AString aData); */
                    Method {
                        name: "writeToCacheEntry",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void closeCacheEntry (in nsresult reason); */
                    Method {
                        name: "closeCacheEntry",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void setSecurityInfo (in nsISupports aSecurityInfo); */
                    Method {
                        name: "setSecurityInfo",
                        abi: "C",
                        params: &[Param { name: "aSecurityInfo", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void setCharsetAndSource (in long aSource, in ACString aCharset); */
                    Method {
                        name: "setCharsetAndSource",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "libc::int32_t" }, Param { name: "aCharset", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString getCharsetAndSource (out long aSource); */
                    Method {
                        name: "getCharsetAndSource",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*mut libc::int32_t" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

