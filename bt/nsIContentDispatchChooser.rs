//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentDispatchChooser.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentDispatchChooser",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void ask (in nsIHandlerInfo aHandler, in nsIInterfaceRequestor aWindowContext, in nsIURI aURI, in unsigned long aReason); */
                    Method {
                        name: "ask",
                        abi: "C",
                        params: &[Param { name: "aHandler", ty: "*const nsIHandlerInfo" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aReason", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

