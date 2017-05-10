//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompleteSimpleResult.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompleteSimpleResult",
            base: Some("nsIAutoCompleteResult"),
            methods: Some(&[
                    /* void setSearchString (in AString aSearchString); */
                    Method {
                        name: "setSearchString",
                        abi: "C",
                        params: &[Param { name: "aSearchString", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setErrorDescription (in AString aErrorDescription); */
                    Method {
                        name: "setErrorDescription",
                        abi: "C",
                        params: &[Param { name: "aErrorDescription", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setDefaultIndex (in long aDefaultIndex); */
                    Method {
                        name: "setDefaultIndex",
                        abi: "C",
                        params: &[Param { name: "aDefaultIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setSearchResult (in unsigned short aSearchResult); */
                    Method {
                        name: "setSearchResult",
                        abi: "C",
                        params: &[Param { name: "aSearchResult", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void insertMatchAt (in long aIndex, in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
                    Method {
                        name: "insertMatchAt",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "aValue", ty: "*const nsAString" }, Param { name: "aComment", ty: "*const nsAString" }, Param { name: "aImage", ty: "*const nsAString" }, Param { name: "aStyle", ty: "*const nsAString" }, Param { name: "aFinalCompleteValue", ty: "*const nsAString" }, Param { name: "aLabel", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void appendMatch (in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
                    Method {
                        name: "appendMatch",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsAString" }, Param { name: "aComment", ty: "*const nsAString" }, Param { name: "aImage", ty: "*const nsAString" }, Param { name: "aStyle", ty: "*const nsAString" }, Param { name: "aFinalCompleteValue", ty: "*const nsAString" }, Param { name: "aLabel", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIAutoCompleteSimpleResultListener getListener (); */
                    Method {
                        name: "getListener",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIAutoCompleteSimpleResultListener" }],
                        ret: "nsresult",
                    },

                    /* void setListener (in nsIAutoCompleteSimpleResultListener aListener); */
                    Method {
                        name: "setListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIAutoCompleteSimpleResultListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAutoCompleteSimpleResultListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onValueRemoved (in nsIAutoCompleteSimpleResult aResult, in AString aValue, in boolean aRemoveFromDb); */
                    Method {
                        name: "onValueRemoved",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*const nsIAutoCompleteSimpleResult" }, Param { name: "aValue", ty: "*const nsAString" }, Param { name: "aRemoveFromDb", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

