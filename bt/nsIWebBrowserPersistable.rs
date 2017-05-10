//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserPersistable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserPersistable",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void startPersistence (in unsigned long long aOuterWindowID, in nsIWebBrowserPersistDocumentReceiver aRecv); */
                    Method {
                        name: "startPersistence",
                        abi: "C",
                        params: &[Param { name: "aOuterWindowID", ty: "libc::uint64_t" }, Param { name: "aRecv", ty: "*const nsIWebBrowserPersistDocumentReceiver" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

