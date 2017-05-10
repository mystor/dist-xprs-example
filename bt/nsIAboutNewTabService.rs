//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAboutNewTabService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAboutNewTabService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute ACString newTabURL; */
                    Method {
                        name: "get_newTabURL",
                        abi: "C",
                        params: &[Param { name: "aNewTabURL", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_newTabURL",
                        abi: "C",
                        params: &[Param { name: "aNewTabURL", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute ACString defaultURL; */
                    Method {
                        name: "get_defaultURL",
                        abi: "C",
                        params: &[Param { name: "aDefaultURL", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defaultURL",
                        abi: "C",
                        params: &[Param { name: "aDefaultURL", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool overridden; */
                    Method {
                        name: "get_overridden",
                        abi: "C",
                        params: &[Param { name: "aOverridden", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute bool activityStreamEnabled; */
                    Method {
                        name: "get_activityStreamEnabled",
                        abi: "C",
                        params: &[Param { name: "aActivityStreamEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString activityStreamURL; */
                    Method {
                        name: "get_activityStreamURL",
                        abi: "C",
                        params: &[Param { name: "aActivityStreamURL", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* void resetNewTabURL (); */
                    Method {
                        name: "resetNewTabURL",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

