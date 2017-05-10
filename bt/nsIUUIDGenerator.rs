//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUUIDGenerator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUUIDGenerator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIDPtr generateUUID (); */
                    Method {
                        name: "generateUUID",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsID" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void generateUUIDInPlace (in nsNonConstIDPtr id); */
                    Method {
                        name: "generateUUIDInPlace",
                        abi: "C",
                        params: &[Param { name: "id", ty: "*const nsID" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

