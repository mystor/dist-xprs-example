//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLImageElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLImageElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString alt; */
                    Method {
                        name: "get_alt",
                        abi: "C",
                        params: &[Param { name: "aAlt", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_alt",
                        abi: "C",
                        params: &[Param { name: "aAlt", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString src; */
                    Method {
                        name: "get_src",
                        abi: "C",
                        params: &[Param { name: "aSrc", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_src",
                        abi: "C",
                        params: &[Param { name: "aSrc", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString srcset; */
                    Method {
                        name: "get_srcset",
                        abi: "C",
                        params: &[Param { name: "aSrcset", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_srcset",
                        abi: "C",
                        params: &[Param { name: "aSrcset", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString sizes; */
                    Method {
                        name: "get_sizes",
                        abi: "C",
                        params: &[Param { name: "aSizes", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sizes",
                        abi: "C",
                        params: &[Param { name: "aSizes", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString useMap; */
                    Method {
                        name: "get_useMap",
                        abi: "C",
                        params: &[Param { name: "aUseMap", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_useMap",
                        abi: "C",
                        params: &[Param { name: "aUseMap", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean isMap; */
                    Method {
                        name: "get_isMap",
                        abi: "C",
                        params: &[Param { name: "aIsMap", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_isMap",
                        abi: "C",
                        params: &[Param { name: "aIsMap", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long width; */
                    Method {
                        name: "get_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long height; */
                    Method {
                        name: "get_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long naturalWidth; */
                    Method {
                        name: "get_naturalWidth",
                        abi: "C",
                        params: &[Param { name: "aNaturalWidth", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long naturalHeight; */
                    Method {
                        name: "get_naturalHeight",
                        abi: "C",
                        params: &[Param { name: "aNaturalHeight", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean complete; */
                    Method {
                        name: "get_complete",
                        abi: "C",
                        params: &[Param { name: "aComplete", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }],
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

                    /* attribute DOMString border; */
                    Method {
                        name: "get_border",
                        abi: "C",
                        params: &[Param { name: "aBorder", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_border",
                        abi: "C",
                        params: &[Param { name: "aBorder", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute long hspace; */
                    Method {
                        name: "get_hspace",
                        abi: "C",
                        params: &[Param { name: "aHspace", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_hspace",
                        abi: "C",
                        params: &[Param { name: "aHspace", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString longDesc; */
                    Method {
                        name: "get_longDesc",
                        abi: "C",
                        params: &[Param { name: "aLongDesc", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_longDesc",
                        abi: "C",
                        params: &[Param { name: "aLongDesc", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute long vspace; */
                    Method {
                        name: "get_vspace",
                        abi: "C",
                        params: &[Param { name: "aVspace", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_vspace",
                        abi: "C",
                        params: &[Param { name: "aVspace", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString lowsrc; */
                    Method {
                        name: "get_lowsrc",
                        abi: "C",
                        params: &[Param { name: "aLowsrc", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_lowsrc",
                        abi: "C",
                        params: &[Param { name: "aLowsrc", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString currentSrc; */
                    Method {
                        name: "get_currentSrc",
                        abi: "C",
                        params: &[Param { name: "aCurrentSrc", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long x; */
                    Method {
                        name: "get_x",
                        abi: "C",
                        params: &[Param { name: "aX", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long y; */
                    Method {
                        name: "get_y",
                        abi: "C",
                        params: &[Param { name: "aY", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

