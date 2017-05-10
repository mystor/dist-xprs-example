//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITokenPasswordDialogs.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITokenPasswordDialogs",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean setPassword (in nsIInterfaceRequestor ctx, in AString tokenName); */
                    Method {
                        name: "setPassword",
                        abi: "C",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "tokenName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

