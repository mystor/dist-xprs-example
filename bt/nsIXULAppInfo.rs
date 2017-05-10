//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULAppInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULAppInfo",
            base: Some("nsIPlatformInfo"),
            methods: Some(&[
                    /* readonly attribute ACString vendor; */
                    Method {
                        name: "get_vendor",
                        abi: "C",
                        params: &[Param { name: "aVendor", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString ID; */
                    Method {
                        name: "get_ID",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString version; */
                    Method {
                        name: "get_version",
                        abi: "C",
                        params: &[Param { name: "aVersion", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString appBuildID; */
                    Method {
                        name: "get_appBuildID",
                        abi: "C",
                        params: &[Param { name: "aAppBuildID", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString UAName; */
                    Method {
                        name: "get_UAName",
                        abi: "C",
                        params: &[Param { name: "aUAName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

