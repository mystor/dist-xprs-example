//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProfileMigrator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProfileStartup",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIFile directory; */
                    Method {
                        name: "get_directory",
                        abi: "C",
                        params: &[Param { name: "aDirectory", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void doStartup (); */
                    Method {
                        name: "doStartup",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIProfileMigrator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void migrate (in nsIProfileStartup aStartup, in ACString aKey, [optional] in ACString aProfileName); */
                    Method {
                        name: "migrate",
                        abi: "C",
                        params: &[Param { name: "aStartup", ty: "*const nsIProfileStartup" }, Param { name: "aKey", ty: "*const nsACString" }, Param { name: "aProfileName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

