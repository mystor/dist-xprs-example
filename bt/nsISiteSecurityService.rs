//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISiteSecurityService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISiteSecurityState",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsISiteHSTSState",
            base: Some("nsISiteSecurityState"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsISiteHPKPState",
            base: Some("nsISiteSecurityState"),
            methods: Some(&[
                    /* readonly attribute nsISimpleEnumerator sha256Keys; */
                    Method {
                        name: "get_sha256Keys",
                        abi: "C",
                        params: &[Param { name: "aSha256Keys", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISiteSecurityService",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

