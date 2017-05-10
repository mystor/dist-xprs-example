//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMacDockSupport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMacDockSupport",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIStandaloneNativeMenu dockMenu; */
                    Method {
                        name: "get_dockMenu",
                        abi: "C",
                        params: &[Param { name: "aDockMenu", ty: "*mut *const nsIStandaloneNativeMenu" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_dockMenu",
                        abi: "C",
                        params: &[Param { name: "aDockMenu", ty: "*const nsIStandaloneNativeMenu" }],
                        ret: "nsresult",
                    },

                    /* void activateApplication (in boolean aIgnoreOtherApplications); */
                    Method {
                        name: "activateApplication",
                        abi: "C",
                        params: &[Param { name: "aIgnoreOtherApplications", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute AString badgeText; */
                    Method {
                        name: "get_badgeText",
                        abi: "C",
                        params: &[Param { name: "aBadgeText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_badgeText",
                        abi: "C",
                        params: &[Param { name: "aBadgeText", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

