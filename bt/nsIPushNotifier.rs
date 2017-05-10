//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPushNotifier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPushNotifier",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIPushData",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIPushMessage",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "get_principal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPushData data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut *const nsIPushData" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

