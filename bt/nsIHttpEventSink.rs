//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpEventSink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpEventSink",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void onRedirect (in nsIHttpChannel httpChannel, in nsIChannel newChannel); */
                    Method {
                        name: "onRedirect",
                        abi: "C",
                        params: &[Param { name: "httpChannel", ty: "*const nsIHttpChannel" }, Param { name: "newChannel", ty: "*const nsIChannel" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

