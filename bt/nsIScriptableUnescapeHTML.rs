//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptableUnescapeHTML.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptableUnescapeHTML",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AString unescape (in AString src); */
                    Method {
                        name: "unescape",
                        abi: "C",
                        params: &[Param { name: "src", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocumentFragment parseFragment (in AString fragment, in boolean isXML, in nsIURI baseURI, in nsIDOMElement element); */
                    Method {
                        name: "parseFragment",
                        abi: "C",
                        params: &[Param { name: "fragment", ty: "*const nsAString" }, Param { name: "isXML", ty: "bool" }, Param { name: "baseURI", ty: "*const nsIURI" }, Param { name: "element", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut *const nsIDOMDocumentFragment" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

