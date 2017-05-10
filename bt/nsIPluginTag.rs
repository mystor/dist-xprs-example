//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPluginTag.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPluginTag",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIFakePluginTag",
            base: Some("nsIPluginTag"),
            methods: Some(&[
                    /* readonly attribute nsIURI handlerURI; */
                    Method {
                        name: "get_handlerURI",
                        abi: "C",
                        params: &[Param { name: "aHandlerURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

