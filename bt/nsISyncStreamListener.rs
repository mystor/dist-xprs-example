//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISyncStreamListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISyncStreamListener",
            base: Some("nsIStreamListener"),
            methods: Some(&[
                    /* readonly attribute nsIInputStream inputStream; */
                    Method {
                        name: "get_inputStream",
                        abi: "C",
                        params: &[Param { name: "aInputStream", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

