//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentViewerContainer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentViewerContainer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void embed (in nsIContentViewer aDocViewer, in string aCommand, in nsISupports aExtraInfo); */
                    Method {
                        name: "embed",
                        abi: "C",
                        params: &[Param { name: "aDocViewer", ty: "*const nsIContentViewer" }, Param { name: "aCommand", ty: "*const libc::c_char" }, Param { name: "aExtraInfo", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void setIsPrinting (in boolean aIsPrinting); */
                    Method {
                        name: "setIsPrinting",
                        abi: "C",
                        params: &[Param { name: "aIsPrinting", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

