//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPermission.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPermission",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "get_principal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t capability; */
                    Method {
                        name: "get_capability",
                        abi: "C",
                        params: &[Param { name: "aCapability", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t expireType; */
                    Method {
                        name: "get_expireType",
                        abi: "C",
                        params: &[Param { name: "aExpireType", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute int64_t expireTime; */
                    Method {
                        name: "get_expireTime",
                        abi: "C",
                        params: &[Param { name: "aExpireTime", ty: "*mut int64_t" }],
                        ret: "nsresult",
                    },

                    /* boolean matches (in nsIPrincipal principal, in boolean exactHost); */
                    Method {
                        name: "matches",
                        abi: "C",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "exactHost", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean matchesURI (in nsIURI uri, in boolean exactHost); */
                    Method {
                        name: "matchesURI",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "exactHost", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

