//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIControllerCommand.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIControllerCommand",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandContext); */
                    Method {
                        name: "isCommandEnabled",
                        abi: "C",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommandContext", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void getCommandStateParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
                    Method {
                        name: "getCommandStateParams",
                        abi: "C",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aParams", ty: "*const nsICommandParams" }, Param { name: "aCommandContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void doCommand (in string aCommandName, in nsISupports aCommandContext); */
                    Method {
                        name: "doCommand",
                        abi: "C",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommandContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void doCommandParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
                    Method {
                        name: "doCommandParams",
                        abi: "C",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aParams", ty: "*const nsICommandParams" }, Param { name: "aCommandContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

