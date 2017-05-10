//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/amIAddonManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "amIAddonManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean mapURIToAddonID (in nsIURI aURI, out AUTF8String aID); */
                    Method {
                        name: "mapURIToAddonID",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aID", ty: "*mut nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

