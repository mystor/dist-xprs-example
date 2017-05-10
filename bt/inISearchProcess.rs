//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/inISearchProcess.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "inISearchProcess",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean isActive; */
                    Method {
                        name: "get_isActive",
                        abi: "C",
                        params: &[Param { name: "aIsActive", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long resultCount; */
                    Method {
                        name: "get_resultCount",
                        abi: "C",
                        params: &[Param { name: "aResultCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean holdResults; */
                    Method {
                        name: "get_holdResults",
                        abi: "C",
                        params: &[Param { name: "aHoldResults", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_holdResults",
                        abi: "C",
                        params: &[Param { name: "aHoldResults", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void searchSync (); */
                    Method {
                        name: "searchSync",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void searchAsync (in inISearchObserver aObserver); */
                    Method {
                        name: "searchAsync",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const inISearchObserver" }],
                        ret: "nsresult",
                    },

                    /* void searchStop (); */
                    Method {
                        name: "searchStop",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean searchStep (); */
                    Method {
                        name: "searchStep",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AString getStringResultAt (in long aIndex); */
                    Method {
                        name: "getStringResultAt",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* long getIntResultAt (in long aIndex); */
                    Method {
                        name: "getIntResultAt",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* unsigned long getUIntResultAt (in long aIndex); */
                    Method {
                        name: "getUIntResultAt",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

