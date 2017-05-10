//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUnicharStreamLoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUnicharStreamLoaderObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* ACString onDetermineCharset (in nsIUnicharStreamLoader aLoader, in nsISupports aContext, in ACString aSegment); */
                    Method {
                        name: "onDetermineCharset",
                        abi: "C",
                        params: &[Param { name: "aLoader", ty: "*const nsIUnicharStreamLoader" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aSegment", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onStreamComplete (in nsIUnicharStreamLoader aLoader, in nsISupports aContext, in nsresult aStatus, in AString aBuffer); */
                    Method {
                        name: "onStreamComplete",
                        abi: "C",
                        params: &[Param { name: "aLoader", ty: "*const nsIUnicharStreamLoader" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aStatus", ty: "nsresult" }, Param { name: "aBuffer", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUnicharStreamLoader",
            base: Some("nsIStreamListener"),
            methods: Some(&[
                    /* void init (in nsIUnicharStreamLoaderObserver aObserver); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIUnicharStreamLoaderObserver" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIChannel channel; */
                    Method {
                        name: "get_channel",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString charset; */
                    Method {
                        name: "get_charset",
                        abi: "C",
                        params: &[Param { name: "aCharset", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString rawBuffer; */
                    Method {
                        name: "get_rawBuffer",
                        abi: "C",
                        params: &[Param { name: "aRawBuffer", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

