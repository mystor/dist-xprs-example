//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/amIAddonPathService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "amIAddonPathService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AString findAddonId (in AString path); */
                    Method {
                        name: "findAddonId",
                        abi: "C",
                        params: &[Param { name: "path", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void insertPath (in AString path, in AString addonId); */
                    Method {
                        name: "insertPath",
                        abi: "C",
                        params: &[Param { name: "path", ty: "*const nsAString" }, Param { name: "addonId", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString mapURIToAddonId (in nsIURI aURI); */
                    Method {
                        name: "mapURIToAddonId",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

