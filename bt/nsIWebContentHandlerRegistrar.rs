//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebContentHandlerRegistrar.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebContentHandlerRegistrar",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void registerContentHandler (in DOMString mimeType, in DOMString uri, in DOMString title, in nsISupports windowOrBrowser); */
                    Method {
                        name: "registerContentHandler",
                        abi: "C",
                        params: &[Param { name: "mimeType", ty: "*const nsAString" }, Param { name: "uri", ty: "*const nsAString" }, Param { name: "title", ty: "*const nsAString" }, Param { name: "windowOrBrowser", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void registerProtocolHandler (in DOMString protocol, in DOMString uri, in DOMString title, in nsISupports windowOrBrowser); */
                    Method {
                        name: "registerProtocolHandler",
                        abi: "C",
                        params: &[Param { name: "protocol", ty: "*const nsAString" }, Param { name: "uri", ty: "*const nsAString" }, Param { name: "title", ty: "*const nsAString" }, Param { name: "windowOrBrowser", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

