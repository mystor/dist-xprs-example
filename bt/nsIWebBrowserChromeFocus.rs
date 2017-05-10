//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserChromeFocus.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserChromeFocus",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void focusNextElement (in bool aForDocumentNavigation); */
                    Method {
                        name: "focusNextElement",
                        abi: "C",
                        params: &[Param { name: "aForDocumentNavigation", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void focusPrevElement (in bool aForDocumentNavigation); */
                    Method {
                        name: "focusPrevElement",
                        abi: "C",
                        params: &[Param { name: "aForDocumentNavigation", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

