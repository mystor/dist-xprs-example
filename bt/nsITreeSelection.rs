//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITreeSelection.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITreeSelection",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsITreeBoxObject tree; */
                    Method {
                        name: "get_tree",
                        abi: "C",
                        params: &[Param { name: "aTree", ty: "*mut *const nsITreeBoxObject" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_tree",
                        abi: "C",
                        params: &[Param { name: "aTree", ty: "*const nsITreeBoxObject" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean single; */
                    Method {
                        name: "get_single",
                        abi: "C",
                        params: &[Param { name: "aSingle", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long count; */
                    Method {
                        name: "get_count",
                        abi: "C",
                        params: &[Param { name: "aCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean isSelected (in long index); */
                    Method {
                        name: "isSelected",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void select (in long index); */
                    Method {
                        name: "select",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void timedSelect (in long index, in long delay); */
                    Method {
                        name: "timedSelect",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "delay", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void toggleSelect (in long index); */
                    Method {
                        name: "toggleSelect",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void rangedSelect (in long startIndex, in long endIndex, in boolean augment); */
                    Method {
                        name: "rangedSelect",
                        abi: "C",
                        params: &[Param { name: "startIndex", ty: "libc::int32_t" }, Param { name: "endIndex", ty: "libc::int32_t" }, Param { name: "augment", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void clearRange (in long startIndex, in long endIndex); */
                    Method {
                        name: "clearRange",
                        abi: "C",
                        params: &[Param { name: "startIndex", ty: "libc::int32_t" }, Param { name: "endIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void clearSelection (); */
                    Method {
                        name: "clearSelection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void invertSelection (); */
                    Method {
                        name: "invertSelection",
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

                    /* long getRangeCount (); */
                    Method {
                        name: "getRangeCount",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void getRangeAt (in long i, out long min, out long max); */
                    Method {
                        name: "getRangeAt",
                        abi: "C",
                        params: &[Param { name: "i", ty: "libc::int32_t" }, Param { name: "min", ty: "*mut libc::int32_t" }, Param { name: "max", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void invalidateSelection (); */
                    Method {
                        name: "invalidateSelection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void adjustSelection (in long index, in long count); */
                    Method {
                        name: "adjustSelection",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "count", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean selectEventsSuppressed; */
                    Method {
                        name: "get_selectEventsSuppressed",
                        abi: "C",
                        params: &[Param { name: "aSelectEventsSuppressed", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selectEventsSuppressed",
                        abi: "C",
                        params: &[Param { name: "aSelectEventsSuppressed", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute long currentIndex; */
                    Method {
                        name: "get_currentIndex",
                        abi: "C",
                        params: &[Param { name: "aCurrentIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_currentIndex",
                        abi: "C",
                        params: &[Param { name: "aCurrentIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsITreeColumn currentColumn; */
                    Method {
                        name: "get_currentColumn",
                        abi: "C",
                        params: &[Param { name: "aCurrentColumn", ty: "*mut *const nsITreeColumn" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_currentColumn",
                        abi: "C",
                        params: &[Param { name: "aCurrentColumn", ty: "*const nsITreeColumn" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long shiftSelectPivot; */
                    Method {
                        name: "get_shiftSelectPivot",
                        abi: "C",
                        params: &[Param { name: "aShiftSelectPivot", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINativeTreeSelection",
            base: Some("nsITreeSelection"),
            methods: Some(&[
                    /* [noscript] void ensureNative (); */
                    Method {
                        name: "ensureNative",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

