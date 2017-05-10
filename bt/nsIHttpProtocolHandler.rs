//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpProtocolHandler",
            base: Some("nsIProxiedProtocolHandler"),
            methods: Some(&[
                    /* [must_use] readonly attribute ACString userAgent; */
                    Method {
                        name: "get_userAgent",
                        abi: "C",
                        params: &[Param { name: "aUserAgent", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString appName; */
                    Method {
                        name: "get_appName",
                        abi: "C",
                        params: &[Param { name: "aAppName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString appVersion; */
                    Method {
                        name: "get_appVersion",
                        abi: "C",
                        params: &[Param { name: "aAppVersion", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString platform; */
                    Method {
                        name: "get_platform",
                        abi: "C",
                        params: &[Param { name: "aPlatform", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString oscpu; */
                    Method {
                        name: "get_oscpu",
                        abi: "C",
                        params: &[Param { name: "aOscpu", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute ACString misc; */
                    Method {
                        name: "get_misc",
                        abi: "C",
                        params: &[Param { name: "aMisc", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

