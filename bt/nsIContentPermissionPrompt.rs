//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentPermissionPrompt.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentPermissionType",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString access; */
                    Method {
                        name: "get_access",
                        abi: "C",
                        params: &[Param { name: "aAccess", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray options; */
                    Method {
                        name: "get_options",
                        abi: "C",
                        params: &[Param { name: "aOptions", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIContentPermissionRequestCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notifyVisibility (in boolean isVisible); */
                    Method {
                        name: "notifyVisibility",
                        abi: "C",
                        params: &[Param { name: "isVisible", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIContentPermissionRequester",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getVisibility (in nsIContentPermissionRequestCallback callback); */
                    Method {
                        name: "getVisibility",
                        abi: "C",
                        params: &[Param { name: "callback", ty: "*const nsIContentPermissionRequestCallback" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIContentPermissionRequestCallback onVisibilityChange; */
                    Method {
                        name: "get_onVisibilityChange",
                        abi: "C",
                        params: &[Param { name: "aOnVisibilityChange", ty: "*mut *const nsIContentPermissionRequestCallback" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_onVisibilityChange",
                        abi: "C",
                        params: &[Param { name: "aOnVisibilityChange", ty: "*const nsIContentPermissionRequestCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIContentPermissionRequest",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIContentPermissionPrompt",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void prompt (in nsIContentPermissionRequest request); */
                    Method {
                        name: "prompt",
                        abi: "C",
                        params: &[Param { name: "request", ty: "*const nsIContentPermissionRequest" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

