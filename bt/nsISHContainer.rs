//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISHContainer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISHContainer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long childCount; */
                    Method {
                        name: "get_childCount",
                        abi: "C",
                        params: &[Param { name: "aChildCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void AddChild (in nsISHEntry child, in long offset); */
                    Method {
                        name: "AddChild",
                        abi: "C",
                        params: &[Param { name: "child", ty: "*const nsISHEntry" }, Param { name: "offset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void RemoveChild (in nsISHEntry child); */
                    Method {
                        name: "RemoveChild",
                        abi: "C",
                        params: &[Param { name: "child", ty: "*const nsISHEntry" }],
                        ret: "nsresult",
                    },

                    /* nsISHEntry GetChildAt (in long index); */
                    Method {
                        name: "GetChildAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsISHEntry" }],
                        ret: "nsresult",
                    },

                    /* void ReplaceChild (in nsISHEntry aNewChild); */
                    Method {
                        name: "ReplaceChild",
                        abi: "C",
                        params: &[Param { name: "aNewChild", ty: "*const nsISHEntry" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

