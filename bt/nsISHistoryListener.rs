//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISHistoryListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISHistoryListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void OnHistoryNewEntry (in nsIURI aNewURI, in long aOldIndex); */
                    Method {
                        name: "OnHistoryNewEntry",
                        abi: "C",
                        params: &[Param { name: "aNewURI", ty: "*const nsIURI" }, Param { name: "aOldIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean OnHistoryGoBack (in nsIURI aBackURI); */
                    Method {
                        name: "OnHistoryGoBack",
                        abi: "C",
                        params: &[Param { name: "aBackURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean OnHistoryGoForward (in nsIURI aForwardURI); */
                    Method {
                        name: "OnHistoryGoForward",
                        abi: "C",
                        params: &[Param { name: "aForwardURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean OnHistoryReload (in nsIURI aReloadURI, in unsigned long aReloadFlags); */
                    Method {
                        name: "OnHistoryReload",
                        abi: "C",
                        params: &[Param { name: "aReloadURI", ty: "*const nsIURI" }, Param { name: "aReloadFlags", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean OnHistoryGotoIndex (in long aIndex, in nsIURI aGotoURI); */
                    Method {
                        name: "OnHistoryGotoIndex",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "aGotoURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean OnHistoryPurge (in long aNumEntries); */
                    Method {
                        name: "OnHistoryPurge",
                        abi: "C",
                        params: &[Param { name: "aNumEntries", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void OnHistoryReplaceEntry (in long aIndex); */
                    Method {
                        name: "OnHistoryReplaceEntry",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void OnLengthChanged (in long aCount); */
                    Method {
                        name: "OnLengthChanged",
                        abi: "C",
                        params: &[Param { name: "aCount", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void OnIndexChanged (in long aIndex); */
                    Method {
                        name: "OnIndexChanged",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

