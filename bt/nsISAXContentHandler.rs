//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXContentHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISAXContentHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void startDocument (); */
                    Method {
                        name: "startDocument",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void endDocument (); */
                    Method {
                        name: "endDocument",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void startElement (in AString uri, in AString localName, in AString qName, in nsISAXAttributes attributes); */
                    Method {
                        name: "startElement",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "qName", ty: "*const nsAString" }, Param { name: "attributes", ty: "*const nsISAXAttributes" }],
                        ret: "nsresult",
                    },

                    /* void endElement (in AString uri, in AString localName, in AString qName); */
                    Method {
                        name: "endElement",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "qName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void characters (in AString value); */
                    Method {
                        name: "characters",
                        abi: "C",
                        params: &[Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void processingInstruction (in AString target, in AString data); */
                    Method {
                        name: "processingInstruction",
                        abi: "C",
                        params: &[Param { name: "target", ty: "*const nsAString" }, Param { name: "data", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void ignorableWhitespace (in AString whitespace); */
                    Method {
                        name: "ignorableWhitespace",
                        abi: "C",
                        params: &[Param { name: "whitespace", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void startPrefixMapping (in AString prefix, in AString uri); */
                    Method {
                        name: "startPrefixMapping",
                        abi: "C",
                        params: &[Param { name: "prefix", ty: "*const nsAString" }, Param { name: "uri", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void endPrefixMapping (in AString prefix); */
                    Method {
                        name: "endPrefixMapping",
                        abi: "C",
                        params: &[Param { name: "prefix", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

