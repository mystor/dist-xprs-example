//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlClassifierStreamUpdater.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierStreamUpdater",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean downloadUpdates (in ACString aRequestTables, in ACString aRequestPayload, in boolean aIsPostRequest, in ACString aUpdateUrl, in nsIUrlClassifierCallback aSuccessCallback, in nsIUrlClassifierCallback aUpdateErrorCallback, in nsIUrlClassifierCallback aDownloadErrorCallback); */
                    Method {
                        name: "downloadUpdates",
                        abi: "C",
                        params: &[Param { name: "aRequestTables", ty: "*const nsACString" }, Param { name: "aRequestPayload", ty: "*const nsACString" }, Param { name: "aIsPostRequest", ty: "bool" }, Param { name: "aUpdateUrl", ty: "*const nsACString" }, Param { name: "aSuccessCallback", ty: "*const nsIUrlClassifierCallback" }, Param { name: "aUpdateErrorCallback", ty: "*const nsIUrlClassifierCallback" }, Param { name: "aDownloadErrorCallback", ty: "*const nsIUrlClassifierCallback" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

