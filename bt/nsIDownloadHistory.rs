//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownloadHistory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDownloadHistory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addDownload (in nsIURI aSource, [optional] in nsIURI aReferrer, [optional] in PRTime aStartTime, [optional] in nsIURI aDestination); */
                    Method {
                        name: "addDownload",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIURI" }, Param { name: "aReferrer", ty: "*const nsIURI" }, Param { name: "aStartTime", ty: "PRTime" }, Param { name: "aDestination", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void removeAllDownloads (); */
                    Method {
                        name: "removeAllDownloads",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

