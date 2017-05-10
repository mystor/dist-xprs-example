//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkInterceptController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInterceptedChannel",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsINetworkInterceptController",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* bool shouldPrepareForIntercept (in nsIURI aURI, in bool aIsNonSubresourceRequest); */
                    Method {
                        name: "shouldPrepareForIntercept",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIsNonSubresourceRequest", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void channelIntercepted (in nsIInterceptedChannel aChannel); */
                    Method {
                        name: "channelIntercepted",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIInterceptedChannel" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

