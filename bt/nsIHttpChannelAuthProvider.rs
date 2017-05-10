//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpChannelAuthProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpChannelAuthProvider",
            base: Some("nsICancelable"),
            methods: Some(&[
                    /* [must_use] void init (in nsIHttpAuthenticableChannel channel); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "channel", ty: "*const nsIHttpAuthenticableChannel" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void processAuthentication (in unsigned long httpStatus, in boolean sslConnectFailed); */
                    Method {
                        name: "processAuthentication",
                        abi: "C",
                        params: &[Param { name: "httpStatus", ty: "libc::uint32_t" }, Param { name: "sslConnectFailed", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void addAuthorizationHeaders (in boolean dontUseCachedWWWCreds); */
                    Method {
                        name: "addAuthorizationHeaders",
                        abi: "C",
                        params: &[Param { name: "dontUseCachedWWWCreds", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void checkForSuperfluousAuth (); */
                    Method {
                        name: "checkForSuperfluousAuth",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* [must_use] void disconnect (in nsresult status); */
                    Method {
                        name: "disconnect",
                        abi: "C",
                        params: &[Param { name: "status", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

