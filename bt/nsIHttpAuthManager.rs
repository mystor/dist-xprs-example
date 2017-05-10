//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpAuthManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpAuthManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void getAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, out AString aUserDomain, out AString aUserName, out AString aUserPassword, [optional] in bool aIsPrivate, [optional] in nsIPrincipal aPrincipal); */
                    Method {
                        name: "getAuthIdentity",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*const nsACString" }, Param { name: "aHost", ty: "*const nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aAuthType", ty: "*const nsACString" }, Param { name: "aRealm", ty: "*const nsACString" }, Param { name: "aPath", ty: "*const nsACString" }, Param { name: "aUserDomain", ty: "*mut nsAString" }, Param { name: "aUserName", ty: "*mut nsAString" }, Param { name: "aUserPassword", ty: "*mut nsAString" }, Param { name: "aIsPrivate", ty: "bool" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void setAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, in AString aUserDomain, in AString aUserName, in AString aUserPassword, [optional] in boolean aIsPrivate, [optional] in nsIPrincipal aPrincipal); */
                    Method {
                        name: "setAuthIdentity",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*const nsACString" }, Param { name: "aHost", ty: "*const nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aAuthType", ty: "*const nsACString" }, Param { name: "aRealm", ty: "*const nsACString" }, Param { name: "aPath", ty: "*const nsACString" }, Param { name: "aUserDomain", ty: "*const nsAString" }, Param { name: "aUserName", ty: "*const nsAString" }, Param { name: "aUserPassword", ty: "*const nsAString" }, Param { name: "aIsPrivate", ty: "bool" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void clearAll (); */
                    Method {
                        name: "clearAll",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

