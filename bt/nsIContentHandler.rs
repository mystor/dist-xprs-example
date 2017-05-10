//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleContent (in string aContentType, in nsIInterfaceRequestor aWindowContext, in nsIRequest aRequest); */
                    Method {
                        name: "handleContent",
                        abi: "C",
                        params: &[Param { name: "aContentType", ty: "*const libc::c_char" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "aRequest", ty: "*const nsIRequest" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

