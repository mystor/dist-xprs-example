//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIGeckoMediaPluginChromeService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIGeckoMediaPluginChromeService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addPluginDirectory (in AString directory); */
                    Method {
                        name: "addPluginDirectory",
                        abi: "C",
                        params: &[Param { name: "directory", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void removePluginDirectory (in AString directory); */
                    Method {
                        name: "removePluginDirectory",
                        abi: "C",
                        params: &[Param { name: "directory", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void removeAndDeletePluginDirectory (in AString directory, [optional] in bool defer); */
                    Method {
                        name: "removeAndDeletePluginDirectory",
                        abi: "C",
                        params: &[Param { name: "directory", ty: "*const nsAString" }, Param { name: "defer", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void forgetThisSite (in AString site, in DOMString aPattern); */
                    Method {
                        name: "forgetThisSite",
                        abi: "C",
                        params: &[Param { name: "site", ty: "*const nsAString" }, Param { name: "aPattern", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* bool isPersistentStorageAllowed (in ACString nodeId); */
                    Method {
                        name: "isPersistentStorageAllowed",
                        abi: "C",
                        params: &[Param { name: "nodeId", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIFile getStorageDir (); */
                    Method {
                        name: "getStorageDir",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

