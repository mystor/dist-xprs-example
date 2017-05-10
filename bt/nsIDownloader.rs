//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownloader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDownloader",
            base: Some("nsIStreamListener"),
            methods: Some(&[
                    /* void init (in nsIDownloadObserver observer, in nsIFile downloadLocation); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "observer", ty: "*const nsIDownloadObserver" }, Param { name: "downloadLocation", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDownloadObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onDownloadComplete (in nsIDownloader downloader, in nsIRequest request, in nsISupports ctxt, in nsresult status, in nsIFile result); */
                    Method {
                        name: "onDownloadComplete",
                        abi: "C",
                        params: &[Param { name: "downloader", ty: "*const nsIDownloader" }, Param { name: "request", ty: "*const nsIRequest" }, Param { name: "ctxt", ty: "*const nsISupports" }, Param { name: "status", ty: "nsresult" }, Param { name: "result", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

