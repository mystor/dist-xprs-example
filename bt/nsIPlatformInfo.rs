//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPlatformInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPlatformInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString platformVersion; */
                    Method {
                        name: "get_platformVersion",
                        abi: "C",
                        params: &[Param { name: "aPlatformVersion", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString platformBuildID; */
                    Method {
                        name: "get_platformBuildID",
                        abi: "C",
                        params: &[Param { name: "aPlatformBuildID", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

