//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFilePicker.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFilePickerShownCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void done (in short aResult); */
                    Method {
                        name: "done",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIFilePicker",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in mozIDOMWindowProxy parent, in AString title, in short mode); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "title", ty: "*const nsAString" }, Param { name: "mode", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void appendFilters (in long filterMask); */
                    Method {
                        name: "appendFilters",
                        abi: "C",
                        params: &[Param { name: "filterMask", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void appendFilter (in AString title, in AString filter); */
                    Method {
                        name: "appendFilter",
                        abi: "C",
                        params: &[Param { name: "title", ty: "*const nsAString" }, Param { name: "filter", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString defaultString; */
                    Method {
                        name: "get_defaultString",
                        abi: "C",
                        params: &[Param { name: "aDefaultString", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defaultString",
                        abi: "C",
                        params: &[Param { name: "aDefaultString", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString defaultExtension; */
                    Method {
                        name: "get_defaultExtension",
                        abi: "C",
                        params: &[Param { name: "aDefaultExtension", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defaultExtension",
                        abi: "C",
                        params: &[Param { name: "aDefaultExtension", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute long filterIndex; */
                    Method {
                        name: "get_filterIndex",
                        abi: "C",
                        params: &[Param { name: "aFilterIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_filterIndex",
                        abi: "C",
                        params: &[Param { name: "aFilterIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIFile displayDirectory; */
                    Method {
                        name: "get_displayDirectory",
                        abi: "C",
                        params: &[Param { name: "aDisplayDirectory", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_displayDirectory",
                        abi: "C",
                        params: &[Param { name: "aDisplayDirectory", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* attribute AString displaySpecialDirectory; */
                    Method {
                        name: "get_displaySpecialDirectory",
                        abi: "C",
                        params: &[Param { name: "aDisplaySpecialDirectory", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_displaySpecialDirectory",
                        abi: "C",
                        params: &[Param { name: "aDisplaySpecialDirectory", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile file; */
                    Method {
                        name: "get_file",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI fileURL; */
                    Method {
                        name: "get_fileURL",
                        abi: "C",
                        params: &[Param { name: "aFileURL", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator files; */
                    Method {
                        name: "get_files",
                        abi: "C",
                        params: &[Param { name: "aFiles", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISupports domFileOrDirectory; */
                    Method {
                        name: "get_domFileOrDirectory",
                        abi: "C",
                        params: &[Param { name: "aDomFileOrDirectory", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator domFileOrDirectoryEnumerator; */
                    Method {
                        name: "get_domFileOrDirectoryEnumerator",
                        abi: "C",
                        params: &[Param { name: "aDomFileOrDirectoryEnumerator", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean addToRecentDocs; */
                    Method {
                        name: "get_addToRecentDocs",
                        abi: "C",
                        params: &[Param { name: "aAddToRecentDocs", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_addToRecentDocs",
                        abi: "C",
                        params: &[Param { name: "aAddToRecentDocs", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [deprecated] short show (); */
                    Method {
                        name: "show",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void open (in nsIFilePickerShownCallback aFilePickerShownCallback); */
                    Method {
                        name: "open",
                        abi: "C",
                        params: &[Param { name: "aFilePickerShownCallback", ty: "*const nsIFilePickerShownCallback" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute short mode; */
                    Method {
                        name: "get_mode",
                        abi: "C",
                        params: &[Param { name: "aMode", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AString okButtonLabel; */
                    Method {
                        name: "get_okButtonLabel",
                        abi: "C",
                        params: &[Param { name: "aOkButtonLabel", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_okButtonLabel",
                        abi: "C",
                        params: &[Param { name: "aOkButtonLabel", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

