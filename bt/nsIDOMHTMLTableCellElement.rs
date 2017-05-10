//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLTableCellElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLTableCellElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long cellIndex; */
                    Method {
                        name: "get_cellIndex",
                        abi: "C",
                        params: &[Param { name: "aCellIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString abbr; */
                    Method {
                        name: "get_abbr",
                        abi: "C",
                        params: &[Param { name: "aAbbr", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_abbr",
                        abi: "C",
                        params: &[Param { name: "aAbbr", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString align; */
                    Method {
                        name: "get_align",
                        abi: "C",
                        params: &[Param { name: "aAlign", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_align",
                        abi: "C",
                        params: &[Param { name: "aAlign", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString axis; */
                    Method {
                        name: "get_axis",
                        abi: "C",
                        params: &[Param { name: "aAxis", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_axis",
                        abi: "C",
                        params: &[Param { name: "aAxis", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString bgColor; */
                    Method {
                        name: "get_bgColor",
                        abi: "C",
                        params: &[Param { name: "aBgColor", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_bgColor",
                        abi: "C",
                        params: &[Param { name: "aBgColor", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString ch; */
                    Method {
                        name: "get_ch",
                        abi: "C",
                        params: &[Param { name: "aCh", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_ch",
                        abi: "C",
                        params: &[Param { name: "aCh", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString chOff; */
                    Method {
                        name: "get_chOff",
                        abi: "C",
                        params: &[Param { name: "aChOff", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_chOff",
                        abi: "C",
                        params: &[Param { name: "aChOff", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute long colSpan; */
                    Method {
                        name: "get_colSpan",
                        abi: "C",
                        params: &[Param { name: "aColSpan", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_colSpan",
                        abi: "C",
                        params: &[Param { name: "aColSpan", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString headers; */
                    Method {
                        name: "get_headers",
                        abi: "C",
                        params: &[Param { name: "aHeaders", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_headers",
                        abi: "C",
                        params: &[Param { name: "aHeaders", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString height; */
                    Method {
                        name: "get_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean noWrap; */
                    Method {
                        name: "get_noWrap",
                        abi: "C",
                        params: &[Param { name: "aNoWrap", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_noWrap",
                        abi: "C",
                        params: &[Param { name: "aNoWrap", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute long rowSpan; */
                    Method {
                        name: "get_rowSpan",
                        abi: "C",
                        params: &[Param { name: "aRowSpan", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_rowSpan",
                        abi: "C",
                        params: &[Param { name: "aRowSpan", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString scope; */
                    Method {
                        name: "get_scope",
                        abi: "C",
                        params: &[Param { name: "aScope", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_scope",
                        abi: "C",
                        params: &[Param { name: "aScope", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString vAlign; */
                    Method {
                        name: "get_vAlign",
                        abi: "C",
                        params: &[Param { name: "aVAlign", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_vAlign",
                        abi: "C",
                        params: &[Param { name: "aVAlign", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString width; */
                    Method {
                        name: "get_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

