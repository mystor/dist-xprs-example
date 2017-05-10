//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandLineHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICommandLineHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handle (in nsICommandLine aCommandLine); */
                    Method {
                        name: "handle",
                        abi: "C",
                        params: &[Param { name: "aCommandLine", ty: "*const nsICommandLine" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String helpInfo; */
                    Method {
                        name: "get_helpInfo",
                        abi: "C",
                        params: &[Param { name: "aHelpInfo", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

