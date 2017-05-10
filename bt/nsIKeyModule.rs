//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIKeyModule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIKeyObject",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIKeyObjectFactory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIKeyObject keyFromString (in short aAlgorithm, in ACString aKey); */
                    Method {
                        name: "keyFromString",
                        abi: "C",
                        params: &[Param { name: "aAlgorithm", ty: "libc::int16_t" }, Param { name: "aKey", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIKeyObject" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

