//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditingSession.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEditingSession",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long editorStatus; */
                    Method {
                        name: "get_editorStatus",
                        abi: "C",
                        params: &[Param { name: "aEditorStatus", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void makeWindowEditable (in mozIDOMWindowProxy window, in string aEditorType, in boolean doAfterUriLoad, in boolean aMakeWholeDocumentEditable, in boolean aInteractive); */
                    Method {
                        name: "makeWindowEditable",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "aEditorType", ty: "*const libc::c_char" }, Param { name: "doAfterUriLoad", ty: "bool" }, Param { name: "aMakeWholeDocumentEditable", ty: "bool" }, Param { name: "aInteractive", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* boolean windowIsEditable (in mozIDOMWindowProxy window); */
                    Method {
                        name: "windowIsEditable",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIEditor getEditorForWindow (in mozIDOMWindowProxy window); */
                    Method {
                        name: "getEditorForWindow",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut *const nsIEditor" }],
                        ret: "nsresult",
                    },

                    /* void setupEditorOnWindow (in mozIDOMWindowProxy window); */
                    Method {
                        name: "setupEditorOnWindow",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void tearDownEditorOnWindow (in mozIDOMWindowProxy window); */
                    Method {
                        name: "tearDownEditorOnWindow",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void setEditorOnControllers (in mozIDOMWindowProxy aWindow, in nsIEditor aEditor); */
                    Method {
                        name: "setEditorOnControllers",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "aEditor", ty: "*const nsIEditor" }],
                        ret: "nsresult",
                    },

                    /* void disableJSAndPlugins (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "disableJSAndPlugins",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void restoreJSAndPlugins (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "restoreJSAndPlugins",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void detachFromWindow (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "detachFromWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void reattachToWindow (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "reattachToWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean jsAndPluginsDisabled; */
                    Method {
                        name: "get_jsAndPluginsDisabled",
                        abi: "C",
                        params: &[Param { name: "aJsAndPluginsDisabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

