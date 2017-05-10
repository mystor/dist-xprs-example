//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITreeColumns.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITreeColumn",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMElement element; */
                    Method {
                        name: "get_element",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsITreeColumns columns; */
                    Method {
                        name: "get_columns",
                        abi: "C",
                        params: &[Param { name: "aColumns", ty: "*mut *const nsITreeColumns" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long x; */
                    Method {
                        name: "get_x",
                        abi: "C",
                        params: &[Param { name: "aX", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long width; */
                    Method {
                        name: "get_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void getIdConst ([shared] out wstring idConst); */
                    Method {
                        name: "getIdConst",
                        abi: "C",
                        params: &[Param { name: "idConst", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] readonly attribute nsIAtom atom; */
                    Method {
                        name: "get_atom",
                        abi: "C",
                        params: &[Param { name: "aAtom", ty: "*mut *const nsIAtom" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long index; */
                    Method {
                        name: "get_index",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean primary; */
                    Method {
                        name: "get_primary",
                        abi: "C",
                        params: &[Param { name: "aPrimary", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean cycler; */
                    Method {
                        name: "get_cycler",
                        abi: "C",
                        params: &[Param { name: "aCycler", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean editable; */
                    Method {
                        name: "get_editable",
                        abi: "C",
                        params: &[Param { name: "aEditable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean selectable; */
                    Method {
                        name: "get_selectable",
                        abi: "C",
                        params: &[Param { name: "aSelectable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute short type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* nsITreeColumn getNext (); */
                    Method {
                        name: "getNext",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* nsITreeColumn getPrevious (); */
                    Method {
                        name: "getPrevious",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* void invalidate (); */
                    Method {
                        name: "invalidate",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsITreeColumns",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsITreeBoxObject tree; */
                    Method {
                        name: "get_tree",
                        abi: "C",
                        params: &[Param { name: "aTree", ty: "*mut *const nsITreeBoxObject" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long count; */
                    Method {
                        name: "get_count",
                        abi: "C",
                        params: &[Param { name: "aCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsITreeColumn getFirstColumn (); */
                    Method {
                        name: "getFirstColumn",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* nsITreeColumn getLastColumn (); */
                    Method {
                        name: "getLastColumn",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* nsITreeColumn getPrimaryColumn (); */
                    Method {
                        name: "getPrimaryColumn",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* nsITreeColumn getSortedColumn (); */
                    Method {
                        name: "getSortedColumn",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* nsITreeColumn getKeyColumn (); */
                    Method {
                        name: "getKeyColumn",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* nsITreeColumn getColumnFor (in nsIDOMElement element); */
                    Method {
                        name: "getColumnFor",
                        abi: "C",
                        params: &[Param { name: "element", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* nsITreeColumn getNamedColumn (in AString id); */
                    Method {
                        name: "getNamedColumn",
                        abi: "C",
                        params: &[Param { name: "id", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* nsITreeColumn getColumnAt (in long index); */
                    Method {
                        name: "getColumnAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* void invalidateColumns (); */
                    Method {
                        name: "invalidateColumns",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void restoreNaturalOrder (); */
                    Method {
                        name: "restoreNaturalOrder",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

