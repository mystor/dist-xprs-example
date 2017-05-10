//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIWithPrincipal.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURIWithPrincipal",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "get_principal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI principalUri; */
                    Method {
                        name: "get_principalUri",
                        abi: "C",
                        params: &[Param { name: "aPrincipalUri", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

