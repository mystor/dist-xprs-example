//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIArrayExtensions.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIArrayExtensions",
            base: Some("nsIArray"),
            methods: Some(&[
                    /* uint32_t Count (); */
                    Method {
                        name: "Count",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsISupports GetElementAt (in uint32_t index); */
                    Method {
                        name: "GetElementAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

