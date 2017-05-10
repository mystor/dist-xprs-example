//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebProgress.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebProgress",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addProgressListener (in nsIWebProgressListener aListener, in unsigned long aNotifyMask); */
                    Method {
                        name: "addProgressListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIWebProgressListener" }, Param { name: "aNotifyMask", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void removeProgressListener (in nsIWebProgressListener aListener); */
                    Method {
                        name: "removeProgressListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIWebProgressListener" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute mozIDOMWindowProxy DOMWindow; */
                    Method {
                        name: "get_DOMWindow",
                        abi: "C",
                        params: &[Param { name: "aDOMWindow", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint64_t DOMWindowID; */
                    Method {
                        name: "get_DOMWindowID",
                        abi: "C",
                        params: &[Param { name: "aDOMWindowID", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isTopLevel; */
                    Method {
                        name: "get_isTopLevel",
                        abi: "C",
                        params: &[Param { name: "aIsTopLevel", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isLoadingDocument; */
                    Method {
                        name: "get_isLoadingDocument",
                        abi: "C",
                        params: &[Param { name: "aIsLoadingDocument", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long loadType; */
                    Method {
                        name: "get_loadType",
                        abi: "C",
                        params: &[Param { name: "aLoadType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

