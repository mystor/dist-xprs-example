//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowCreator2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowCreator2",
            base: Some("nsIWindowCreator"),
            methods: Some(&[
                    /* nsIWebBrowserChrome createChromeWindow2 (in nsIWebBrowserChrome parent, in uint32_t chromeFlags, in nsITabParent aOpeningTab, in mozIDOMWindowProxy aOpener, in unsigned long long aNextTabParentId, out boolean cancel); */
                    Method {
                        name: "createChromeWindow2",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const nsIWebBrowserChrome" }, Param { name: "chromeFlags", ty: "uint32_t" }, Param { name: "aOpeningTab", ty: "*const nsITabParent" }, Param { name: "aOpener", ty: "*const mozIDOMWindowProxy" }, Param { name: "aNextTabParentId", ty: "libc::uint64_t" }, Param { name: "cancel", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut *const nsIWebBrowserChrome" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void setScreenId (in uint32_t aScreenId); */
                    Method {
                        name: "setScreenId",
                        abi: "C",
                        params: &[Param { name: "aScreenId", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

