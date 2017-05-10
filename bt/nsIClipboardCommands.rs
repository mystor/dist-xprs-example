//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClipboardCommands.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClipboardCommands",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean canCutSelection (); */
                    Method {
                        name: "canCutSelection",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean canCopySelection (); */
                    Method {
                        name: "canCopySelection",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean canCopyLinkLocation (); */
                    Method {
                        name: "canCopyLinkLocation",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean canCopyImageLocation (); */
                    Method {
                        name: "canCopyImageLocation",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean canCopyImageContents (); */
                    Method {
                        name: "canCopyImageContents",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean canPaste (); */
                    Method {
                        name: "canPaste",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void cutSelection (); */
                    Method {
                        name: "cutSelection",
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

                    /* void copyLinkLocation (); */
                    Method {
                        name: "copyLinkLocation",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void copyImageLocation (); */
                    Method {
                        name: "copyImageLocation",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void copyImageContents (); */
                    Method {
                        name: "copyImageContents",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void paste (); */
                    Method {
                        name: "paste",
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

                    /* void selectNone (); */
                    Method {
                        name: "selectNone",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

