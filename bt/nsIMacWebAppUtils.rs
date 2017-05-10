//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMacWebAppUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITrashAppCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void trashAppFinished (in nsresult rv); */
                    Method {
                        name: "trashAppFinished",
                        abi: "C",
                        params: &[Param { name: "rv", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIMacWebAppUtils",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AString pathForAppWithIdentifier (in AString bundleIdentifier); */
                    Method {
                        name: "pathForAppWithIdentifier",
                        abi: "C",
                        params: &[Param { name: "bundleIdentifier", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void launchAppWithIdentifier (in AString bundleIdentifier); */
                    Method {
                        name: "launchAppWithIdentifier",
                        abi: "C",
                        params: &[Param { name: "bundleIdentifier", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void trashApp (in AString path, in nsITrashAppCallback callback); */
                    Method {
                        name: "trashApp",
                        abi: "C",
                        params: &[Param { name: "path", ty: "*const nsAString" }, Param { name: "callback", ty: "*const nsITrashAppCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

