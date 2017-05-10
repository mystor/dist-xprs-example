//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowCreator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowCreator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIWebBrowserChrome createChromeWindow (in nsIWebBrowserChrome parent, in uint32_t chromeFlags); */
                    Method {
                        name: "createChromeWindow",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const nsIWebBrowserChrome" }, Param { name: "chromeFlags", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIWebBrowserChrome" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

