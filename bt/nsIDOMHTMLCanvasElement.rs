//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLCanvasElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLCanvasElement",
            base: Some("nsISupports"),
            methods: Some(&[
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

                    /* attribute boolean mozOpaque; */
                    Method {
                        name: "get_mozOpaque",
                        abi: "C",
                        params: &[Param { name: "aMozOpaque", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_mozOpaque",
                        abi: "C",
                        params: &[Param { name: "aMozOpaque", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

