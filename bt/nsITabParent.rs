//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITabParent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITabParent",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getChildProcessOffset (out int32_t aCssX, out int32_t aCssY); */
                    Method {
                        name: "getChildProcessOffset",
                        abi: "C",
                        params: &[Param { name: "aCssX", ty: "*mut int32_t" }, Param { name: "aCssY", ty: "*mut int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean useAsyncPanZoom; */
                    Method {
                        name: "get_useAsyncPanZoom",
                        abi: "C",
                        params: &[Param { name: "aUseAsyncPanZoom", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean docShellIsActive; */
                    Method {
                        name: "get_docShellIsActive",
                        abi: "C",
                        params: &[Param { name: "aDocShellIsActive", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_docShellIsActive",
                        abi: "C",
                        params: &[Param { name: "aDocShellIsActive", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [infallible] readonly attribute boolean isPrerendered; */
                    Method {
                        name: "get_isPrerendered",
                        abi: "C",
                        params: &[Param { name: "aIsPrerendered", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void preserveLayers (in boolean aPreserveLayers); */
                    Method {
                        name: "preserveLayers",
                        abi: "C",
                        params: &[Param { name: "aPreserveLayers", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void suppressDisplayport (in bool aEnabled); */
                    Method {
                        name: "suppressDisplayport",
                        abi: "C",
                        params: &[Param { name: "aEnabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint64_t tabId; */
                    Method {
                        name: "get_tabId",
                        abi: "C",
                        params: &[Param { name: "aTabId", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute int32_t osPid; */
                    Method {
                        name: "get_osPid",
                        abi: "C",
                        params: &[Param { name: "aOsPid", ty: "*mut int32_t" }],
                        ret: "nsresult",
                    },

                    /* void navigateByKey (in bool aForward, in bool aForDocumentNavigation); */
                    Method {
                        name: "navigateByKey",
                        abi: "C",
                        params: &[Param { name: "aForward", ty: "bool" }, Param { name: "aForDocumentNavigation", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean hasContentOpener; */
                    Method {
                        name: "get_hasContentOpener",
                        abi: "C",
                        params: &[Param { name: "aHasContentOpener", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean hasPresented; */
                    Method {
                        name: "get_hasPresented",
                        abi: "C",
                        params: &[Param { name: "aHasPresented", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void transmitPermissionsForPrincipal (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "transmitPermissionsForPrincipal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean hasBeforeUnload; */
                    Method {
                        name: "get_hasBeforeUnload",
                        abi: "C",
                        params: &[Param { name: "aHasBeforeUnload", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

