//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISerializationHelper.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISerializationHelper",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* ACString serializeToString (in nsISerializable serializable); */
                    Method {
                        name: "serializeToString",
                        abi: "C",
                        params: &[Param { name: "serializable", ty: "*const nsISerializable" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* nsISupports deserializeObject (in ACString input); */
                    Method {
                        name: "deserializeObject",
                        abi: "C",
                        params: &[Param { name: "input", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

