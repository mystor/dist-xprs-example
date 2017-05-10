//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentViewerEdit.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentViewerEdit",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void clearSelection (); */
                    Method {
                        name: "clearSelection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void selectAll (); */
                    Method {
                        name: "selectAll",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void copySelection (); */
                    Method {
                        name: "copySelection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean copyable; */
                    Method {
                        name: "get_copyable",
                        abi: "C",
                        params: &[Param { name: "aCopyable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void copyLinkLocation (); */
                    Method {
                        name: "copyLinkLocation",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean inLink; */
                    Method {
                        name: "get_inLink",
                        abi: "C",
                        params: &[Param { name: "aInLink", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void copyImage (in long aCopyFlags); */
                    Method {
                        name: "copyImage",
                        abi: "C",
                        params: &[Param { name: "aCopyFlags", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean inImage; */
                    Method {
                        name: "get_inImage",
                        abi: "C",
                        params: &[Param { name: "aInImage", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AString getContents (in string aMimeType, in boolean aSelectionOnly); */
                    Method {
                        name: "getContents",
                        abi: "C",
                        params: &[Param { name: "aMimeType", ty: "*const libc::c_char" }, Param { name: "aSelectionOnly", ty: "bool" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean canGetContents; */
                    Method {
                        name: "get_canGetContents",
                        abi: "C",
                        params: &[Param { name: "aCanGetContents", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void setCommandNode (in nsIDOMNode aNode); */
                    Method {
                        name: "setCommandNode",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

