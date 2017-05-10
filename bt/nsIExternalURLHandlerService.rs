//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExternalURLHandlerService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIExternalURLHandlerService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIHandlerInfo getURLHandlerInfoFromOS (in nsIURI aURL, out boolean aFound); */
                    Method {
                        name: "getURLHandlerInfoFromOS",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsIURI" }, Param { name: "aFound", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut *const nsIHandlerInfo" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

