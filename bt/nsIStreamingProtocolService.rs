//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamingProtocolService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamingProtocolControllerService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIStreamingProtocolController create (in nsIChannel channel); */
                    Method {
                        name: "create",
                        abi: "C",
                        params: &[Param { name: "channel", ty: "*const nsIChannel" }, Param { name: "_retval", ty: "*mut *const nsIStreamingProtocolController" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

