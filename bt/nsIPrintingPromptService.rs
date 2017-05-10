//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintingPromptService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrintingPromptService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void showPrintDialog (in mozIDOMWindowProxy parent, in nsIWebBrowserPrint webBrowserPrint, in nsIPrintSettings printSettings); */
                    Method {
                        name: "showPrintDialog",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "webBrowserPrint", ty: "*const nsIWebBrowserPrint" }, Param { name: "printSettings", ty: "*const nsIPrintSettings" }],
                        ret: "nsresult",
                    },

                    /* void showProgress (in mozIDOMWindowProxy parent, in nsIWebBrowserPrint webBrowserPrint, in nsIPrintSettings printSettings, in nsIObserver openDialogObserver, in boolean isForPrinting, out nsIWebProgressListener webProgressListener, out nsIPrintProgressParams printProgressParams, out boolean notifyOnOpen); */
                    Method {
                        name: "showProgress",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "webBrowserPrint", ty: "*const nsIWebBrowserPrint" }, Param { name: "printSettings", ty: "*const nsIPrintSettings" }, Param { name: "openDialogObserver", ty: "*const nsIObserver" }, Param { name: "isForPrinting", ty: "bool" }, Param { name: "webProgressListener", ty: "*mut *const nsIWebProgressListener" }, Param { name: "printProgressParams", ty: "*mut *const nsIPrintProgressParams" }, Param { name: "notifyOnOpen", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void showPageSetup (in mozIDOMWindowProxy parent, in nsIPrintSettings printSettings, in nsIObserver aObs); */
                    Method {
                        name: "showPageSetup",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "printSettings", ty: "*const nsIPrintSettings" }, Param { name: "aObs", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    /* void showPrinterProperties (in mozIDOMWindowProxy parent, in wstring printerName, in nsIPrintSettings printSettings); */
                    Method {
                        name: "showPrinterProperties",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "printerName", ty: "*const libc::int16_t" }, Param { name: "printSettings", ty: "*const nsIPrintSettings" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

