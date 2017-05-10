//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrivateBrowsingTrackingProtectionWhitelist.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrivateBrowsingTrackingProtectionWhitelist",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addToAllowList (in nsIURI uri); */
                    Method {
                        name: "addToAllowList",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void removeFromAllowList (in nsIURI uri); */
                    Method {
                        name: "removeFromAllowList",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* bool existsInAllowList (in nsIURI uri); */
                    Method {
                        name: "existsInAllowList",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

