//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLIFrameElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLIFrameElement",
            base: Some("nsISupports"),
            methods: Some(&[
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

                    /* attribute DOMString frameBorder; */
                    Method {
                        name: "get_frameBorder",
                        abi: "C",
                        params: &[Param { name: "aFrameBorder", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_frameBorder",
                        abi: "C",
                        params: &[Param { name: "aFrameBorder", ty: "*const nsAString" }],
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

                    /* attribute DOMString marginHeight; */
                    Method {
                        name: "get_marginHeight",
                        abi: "C",
                        params: &[Param { name: "aMarginHeight", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_marginHeight",
                        abi: "C",
                        params: &[Param { name: "aMarginHeight", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString marginWidth; */
                    Method {
                        name: "get_marginWidth",
                        abi: "C",
                        params: &[Param { name: "aMarginWidth", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_marginWidth",
                        abi: "C",
                        params: &[Param { name: "aMarginWidth", ty: "*const nsAString" }],
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

                    /* attribute DOMString scrolling; */
                    Method {
                        name: "get_scrolling",
                        abi: "C",
                        params: &[Param { name: "aScrolling", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_scrolling",
                        abi: "C",
                        params: &[Param { name: "aScrolling", ty: "*const nsAString" }],
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

                    /* attribute DOMString srcdoc; */
                    Method {
                        name: "get_srcdoc",
                        abi: "C",
                        params: &[Param { name: "aSrcdoc", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_srcdoc",
                        abi: "C",
                        params: &[Param { name: "aSrcdoc", ty: "*const nsAString" }],
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

                    /* readonly attribute nsIDOMDocument contentDocument; */
                    Method {
                        name: "get_contentDocument",
                        abi: "C",
                        params: &[Param { name: "aContentDocument", ty: "*mut *const nsIDOMDocument" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean allowFullscreen; */
                    Method {
                        name: "get_allowFullscreen",
                        abi: "C",
                        params: &[Param { name: "aAllowFullscreen", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_allowFullscreen",
                        abi: "C",
                        params: &[Param { name: "aAllowFullscreen", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

