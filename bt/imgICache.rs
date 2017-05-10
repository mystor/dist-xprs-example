//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgICache.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgICache",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void clearCache (in boolean chrome); */
                    Method {
                        name: "clearCache",
                        abi: "C",
                        params: &[Param { name: "chrome", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void removeEntry (in nsIURI uri, [optional] in nsIDOMDocument doc); */
                    Method {
                        name: "removeEntry",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "doc", ty: "*const nsIDOMDocument" }],
                        ret: "nsresult",
                    },

                    /* [must_use] nsIProperties findEntryProperties (in nsIURI uri, [optional] in nsIDOMDocument doc); */
                    Method {
                        name: "findEntryProperties",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "doc", ty: "*const nsIDOMDocument" }, Param { name: "_retval", ty: "*mut *const nsIProperties" }],
                        ret: "nsresult",
                    },

                    /* void respectPrivacyNotifications (); */
                    Method {
                        name: "respectPrivacyNotifications",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* [noscript,notxpcom] void clearCacheForControlledDocument (in nsIDocument doc); */
                    Method {
                        name: "clearCacheForControlledDocument",
                        abi: "C",
                        params: &[Param { name: "doc", ty: "*const nsIDocument" }],
                        ret: "libc::c_void",
                    },

                    ]),
        },


        ]; D}

