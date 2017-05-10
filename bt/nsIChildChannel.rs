//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIChildChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIChildChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void connectParent (in uint32_t registrarId); */
                    Method {
                        name: "connectParent",
                        abi: "C",
                        params: &[Param { name: "registrarId", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void completeRedirectSetup (in nsIStreamListener aListener, in nsISupports aContext); */
                    Method {
                        name: "completeRedirectSetup",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIStreamListener" }, Param { name: "aContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

