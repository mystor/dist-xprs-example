//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXMutableAttributes.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISAXMutableAttributes",
            base: Some("nsISAXAttributes"),
            methods: Some(&[
                    /* void addAttribute (in AString uri, in AString localName, in AString qName, in AString type, in AString value); */
                    Method {
                        name: "addAttribute",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "qName", ty: "*const nsAString" }, Param { name: "type_", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void clear (); */
                    Method {
                        name: "clear",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void removeAttribute (in unsigned long index); */
                    Method {
                        name: "removeAttribute",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void setAttributes (in nsISAXAttributes attributes); */
                    Method {
                        name: "setAttributes",
                        abi: "C",
                        params: &[Param { name: "attributes", ty: "*const nsISAXAttributes" }],
                        ret: "nsresult",
                    },

                    /* void setAttribute (in unsigned long index, in AString uri, in AString localName, in AString qName, in AString type, in AString value); */
                    Method {
                        name: "setAttribute",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "uri", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "qName", ty: "*const nsAString" }, Param { name: "type_", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setLocalName (in unsigned long index, in AString localName); */
                    Method {
                        name: "setLocalName",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "localName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setQName (in unsigned long index, in AString qName); */
                    Method {
                        name: "setQName",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "qName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setType (in unsigned long index, in AString type); */
                    Method {
                        name: "setType",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "type_", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setURI (in unsigned long index, in AString uri); */
                    Method {
                        name: "setURI",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "uri", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setValue (in unsigned long index, in AString value); */
                    Method {
                        name: "setValue",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

