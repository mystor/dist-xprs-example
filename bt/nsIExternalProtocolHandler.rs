//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExternalProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIExternalProtocolHandler",
            base: Some("nsIProtocolHandler"),
            methods: Some(&[
                    /* boolean externalAppExistsForScheme (in ACString scheme); */
                    Method {
                        name: "externalAppExistsForScheme",
                        abi: "C",
                        params: &[Param { name: "scheme", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

