//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserChrome2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserChrome2",
            base: Some("nsIWebBrowserChrome"),
            methods: Some(&[
                    /* void setStatusWithContext (in unsigned long statusType, in AString statusText, in nsISupports statusContext); */
                    Method {
                        name: "setStatusWithContext",
                        abi: "C",
                        params: &[Param { name: "statusType", ty: "libc::uint32_t" }, Param { name: "statusText", ty: "*const nsAString" }, Param { name: "statusContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

