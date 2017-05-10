//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrinterEnumerator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrinterEnumerator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute wstring defaultPrinterName; */
                    Method {
                        name: "get_defaultPrinterName",
                        abi: "C",
                        params: &[Param { name: "aDefaultPrinterName", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void initPrintSettingsFromPrinter (in wstring aPrinterName, in nsIPrintSettings aPrintSettings); */
                    Method {
                        name: "initPrintSettingsFromPrinter",
                        abi: "C",
                        params: &[Param { name: "aPrinterName", ty: "*const libc::int16_t" }, Param { name: "aPrintSettings", ty: "*const nsIPrintSettings" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIStringEnumerator printerNameList; */
                    Method {
                        name: "get_printerNameList",
                        abi: "C",
                        params: &[Param { name: "aPrinterNameList", ty: "*mut *const nsIStringEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

