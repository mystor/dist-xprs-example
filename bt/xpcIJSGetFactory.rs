//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpcIJSGetFactory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "xpcIJSGetFactory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIFactory get (in nsCIDRef aCID); */
                    Method {
                        name: "get",
                        abi: "C",
                        params: &[Param { name: "aCID", ty: "*const nsCID" }, Param { name: "_retval", ty: "*mut *const nsIFactory" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

