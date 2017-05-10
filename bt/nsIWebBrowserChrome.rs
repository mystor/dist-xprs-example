//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserChrome.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserChrome",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setStatus (in unsigned long statusType, in wstring status); */
                    Method {
                        name: "setStatus",
                        abi: "C",
                        params: &[Param { name: "statusType", ty: "libc::uint32_t" }, Param { name: "status", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIWebBrowser webBrowser; */
                    Method {
                        name: "get_webBrowser",
                        abi: "C",
                        params: &[Param { name: "aWebBrowser", ty: "*mut *const nsIWebBrowser" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_webBrowser",
                        abi: "C",
                        params: &[Param { name: "aWebBrowser", ty: "*const nsIWebBrowser" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long chromeFlags; */
                    Method {
                        name: "get_chromeFlags",
                        abi: "C",
                        params: &[Param { name: "aChromeFlags", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_chromeFlags",
                        abi: "C",
                        params: &[Param { name: "aChromeFlags", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void destroyBrowserWindow (); */
                    Method {
                        name: "destroyBrowserWindow",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void sizeBrowserTo (in long aCX, in long aCY); */
                    Method {
                        name: "sizeBrowserTo",
                        abi: "C",
                        params: &[Param { name: "aCX", ty: "libc::int32_t" }, Param { name: "aCY", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void showAsModal (); */
                    Method {
                        name: "showAsModal",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean isWindowModal (); */
                    Method {
                        name: "isWindowModal",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void exitModalEventLoop (in nsresult aStatus); */
                    Method {
                        name: "exitModalEventLoop",
                        abi: "C",
                        params: &[Param { name: "aStatus", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

