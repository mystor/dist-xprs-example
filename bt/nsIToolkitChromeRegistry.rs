//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIToolkitChromeRegistry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIToolkitChromeRegistry",
            base: Some("nsIXULChromeRegistry"),
            methods: Some(&[
                    /* void checkForOSAccessibility (); */
                    Method {
                        name: "checkForOSAccessibility",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsIUTF8StringEnumerator getLocalesForPackage (in AUTF8String aPackage); */
                    Method {
                        name: "getLocalesForPackage",
                        abi: "C",
                        params: &[Param { name: "aPackage", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIUTF8StringEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

