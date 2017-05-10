//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMSVGElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMSVGElement",
            base: Some("nsIDOMElement"),
            methods: Some(&[
                    /* readonly attribute nsIDOMSVGElement ownerSVGElement; */
                    Method {
                        name: "get_ownerSVGElement",
                        abi: "C",
                        params: &[Param { name: "aOwnerSVGElement", ty: "*mut *const nsIDOMSVGElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMSVGElement viewportElement; */
                    Method {
                        name: "get_viewportElement",
                        abi: "C",
                        params: &[Param { name: "aViewportElement", ty: "*mut *const nsIDOMSVGElement" }],
                        ret: "nsresult",
                    },

                    /* [binaryname(SVGClassName)] readonly attribute nsISupports className; */
                    Method {
                        name: "get_SVGClassName",
                        abi: "C",
                        params: &[Param { name: "aClassName", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMCSSStyleDeclaration style; */
                    Method {
                        name: "get_style",
                        abi: "C",
                        params: &[Param { name: "aStyle", ty: "*mut *const nsIDOMCSSStyleDeclaration" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

