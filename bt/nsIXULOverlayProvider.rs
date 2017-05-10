//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULOverlayProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULOverlayProvider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISimpleEnumerator getXULOverlays (in nsIURI aURI); */
                    Method {
                        name: "getXULOverlays",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator getStyleOverlays (in nsIURI aURI); */
                    Method {
                        name: "getStyleOverlays",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

