//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleApplication.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleApplication",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString appName; */
                    Method {
                        name: "get_appName",
                        abi: "C",
                        params: &[Param { name: "aAppName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString appVersion; */
                    Method {
                        name: "get_appVersion",
                        abi: "C",
                        params: &[Param { name: "aAppVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString platformName; */
                    Method {
                        name: "get_platformName",
                        abi: "C",
                        params: &[Param { name: "aPlatformName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString platformVersion; */
                    Method {
                        name: "get_platformVersion",
                        abi: "C",
                        params: &[Param { name: "aPlatformVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

