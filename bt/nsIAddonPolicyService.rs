//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAddonPolicyService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAddonPolicyService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString baseCSP; */
                    Method {
                        name: "get_baseCSP",
                        abi: "C",
                        params: &[Param { name: "aBaseCSP", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString defaultCSP; */
                    Method {
                        name: "get_defaultCSP",
                        abi: "C",
                        params: &[Param { name: "aDefaultCSP", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getAddonCSP (in AString aAddonId); */
                    Method {
                        name: "getAddonCSP",
                        abi: "C",
                        params: &[Param { name: "aAddonId", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* ACString getGeneratedBackgroundPageUrl (in ACString aAddonId); */
                    Method {
                        name: "getGeneratedBackgroundPageUrl",
                        abi: "C",
                        params: &[Param { name: "aAddonId", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean addonHasPermission (in AString aAddonId, in AString aPerm); */
                    Method {
                        name: "addonHasPermission",
                        abi: "C",
                        params: &[Param { name: "aAddonId", ty: "*const nsAString" }, Param { name: "aPerm", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean addonMayLoadURI (in AString aAddonId, in nsIURI aURI, [optional] in boolean aExplicit); */
                    Method {
                        name: "addonMayLoadURI",
                        abi: "C",
                        params: &[Param { name: "aAddonId", ty: "*const nsAString" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aExplicit", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean extensionURILoadableByAnyone (in nsIURI aURI); */
                    Method {
                        name: "extensionURILoadableByAnyone",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AString extensionURIToAddonId (in nsIURI aURI); */
                    Method {
                        name: "extensionURIToAddonId",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAddonContentPolicy",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AString validateAddonCSP (in AString aPolicyString); */
                    Method {
                        name: "validateAddonCSP",
                        abi: "C",
                        params: &[Param { name: "aPolicyString", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

