//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecurityEventSink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISecurityEventSink",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onSecurityChange (in nsISupports i_Context, in unsigned long state); */
                    Method {
                        name: "onSecurityChange",
                        abi: "C",
                        params: &[Param { name: "i_Context", ty: "*const nsISupports" }, Param { name: "state", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

