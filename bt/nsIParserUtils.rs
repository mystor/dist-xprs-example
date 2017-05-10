//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIParserUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIParserUtils",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AString sanitize (in AString src, in unsigned long flags); */
                    Method {
                        name: "sanitize",
                        abi: "C",
                        params: &[Param { name: "src", ty: "*const nsAString" }, Param { name: "flags", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString convertToPlainText (in AString src, in unsigned long flags, in unsigned long wrapCol); */
                    Method {
                        name: "convertToPlainText",
                        abi: "C",
                        params: &[Param { name: "src", ty: "*const nsAString" }, Param { name: "flags", ty: "libc::uint32_t" }, Param { name: "wrapCol", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocumentFragment parseFragment (in AString fragment, in unsigned long flags, in boolean isXML, in nsIURI baseURI, in nsIDOMElement element); */
                    Method {
                        name: "parseFragment",
                        abi: "C",
                        params: &[Param { name: "fragment", ty: "*const nsAString" }, Param { name: "flags", ty: "libc::uint32_t" }, Param { name: "isXML", ty: "bool" }, Param { name: "baseURI", ty: "*const nsIURI" }, Param { name: "element", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut *const nsIDOMDocumentFragment" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

