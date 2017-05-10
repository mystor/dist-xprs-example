//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownloadManagerUI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDownloadManagerUI",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void show ([optional] in nsIInterfaceRequestor aWindowContext, [optional] in nsIDownload aDownload, [optional] in short aReason, [optional] in boolean aUsePrivateUI); */
                    Method {
                        name: "show",
                        abi: "C",
                        params: &[Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "aDownload", ty: "*const nsIDownload" }, Param { name: "aReason", ty: "libc::int16_t" }, Param { name: "aUsePrivateUI", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean visible; */
                    Method {
                        name: "get_visible",
                        abi: "C",
                        params: &[Param { name: "aVisible", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void getAttention (); */
                    Method {
                        name: "getAttention",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

