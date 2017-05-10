//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIControllerContext.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIControllerContext",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in nsIControllerCommandTable aCommandTable); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aCommandTable", ty: "*const nsIControllerCommandTable" }],
                        ret: "nsresult",
                    },

                    /* void setCommandContext (in nsISupports aCommandContext); */
                    Method {
                        name: "setCommandContext",
                        abi: "C",
                        params: &[Param { name: "aCommandContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

