//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDataTransfer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMDataTransfer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString dropEffect; */
                    Method {
                        name: "get_dropEffect",
                        abi: "C",
                        params: &[Param { name: "aDropEffect", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_dropEffect",
                        abi: "C",
                        params: &[Param { name: "aDropEffect", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString effectAllowed; */
                    Method {
                        name: "get_effectAllowed",
                        abi: "C",
                        params: &[Param { name: "aEffectAllowed", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_effectAllowed",
                        abi: "C",
                        params: &[Param { name: "aEffectAllowed", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMFileList files; */
                    Method {
                        name: "get_files",
                        abi: "C",
                        params: &[Param { name: "aFiles", ty: "*mut *const nsIDOMFileList" }],
                        ret: "nsresult",
                    },

                    /* void setDragImage (in nsIDOMElement image, in long x, in long y); */
                    Method {
                        name: "setDragImage",
                        abi: "C",
                        params: &[Param { name: "image", ty: "*const nsIDOMElement" }, Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void addElement (in nsIDOMElement element); */
                    Method {
                        name: "addElement",
                        abi: "C",
                        params: &[Param { name: "element", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long mozItemCount; */
                    Method {
                        name: "get_mozItemCount",
                        abi: "C",
                        params: &[Param { name: "aMozItemCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString mozCursor; */
                    Method {
                        name: "get_mozCursor",
                        abi: "C",
                        params: &[Param { name: "aMozCursor", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_mozCursor",
                        abi: "C",
                        params: &[Param { name: "aMozCursor", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean mozUserCancelled; */
                    Method {
                        name: "get_mozUserCancelled",
                        abi: "C",
                        params: &[Param { name: "aMozUserCancelled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode mozSourceNode; */
                    Method {
                        name: "get_mozSourceNode",
                        abi: "C",
                        params: &[Param { name: "aMozSourceNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* [noscript] attribute unsigned long dropEffectInt; */
                    Method {
                        name: "get_dropEffectInt",
                        abi: "C",
                        params: &[Param { name: "aDropEffectInt", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_dropEffectInt",
                        abi: "C",
                        params: &[Param { name: "aDropEffectInt", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] attribute unsigned long effectAllowedInt; */
                    Method {
                        name: "get_effectAllowedInt",
                        abi: "C",
                        params: &[Param { name: "aEffectAllowedInt", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_effectAllowedInt",
                        abi: "C",
                        params: &[Param { name: "aEffectAllowedInt", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

