//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXAttributes.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISAXAttributes",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* long getIndexFromName (in AString uri, in AString localName); */
                    Method {
                        name: "getIndexFromName",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* long getIndexFromQName (in AString qName); */
                    Method {
                        name: "getIndexFromQName",
                        abi: "C",
                        params: &[Param { name: "qName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* AString getLocalName (in unsigned long index); */
                    Method {
                        name: "getLocalName",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getQName (in unsigned long index); */
                    Method {
                        name: "getQName",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getType (in unsigned long index); */
                    Method {
                        name: "getType",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getTypeFromName (in AString uri, in AString localName); */
                    Method {
                        name: "getTypeFromName",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getTypeFromQName (in AString qName); */
                    Method {
                        name: "getTypeFromQName",
                        abi: "C",
                        params: &[Param { name: "qName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getURI (in unsigned long index); */
                    Method {
                        name: "getURI",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getValue (in unsigned long index); */
                    Method {
                        name: "getValue",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getValueFromName (in AString uri, in AString localName); */
                    Method {
                        name: "getValueFromName",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsAString" }, Param { name: "localName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getValueFromQName (in AString qName); */
                    Method {
                        name: "getValueFromQName",
                        abi: "C",
                        params: &[Param { name: "qName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

