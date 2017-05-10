//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowWatcher.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowWatcher",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* mozIDOMWindowProxy openWindow (in mozIDOMWindowProxy aParent, in string aUrl, in string aName, in string aFeatures, in nsISupports aArguments); */
                    Method {
                        name: "openWindow",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "aUrl", ty: "*const libc::c_char" }, Param { name: "aName", ty: "*const libc::c_char" }, Param { name: "aFeatures", ty: "*const libc::c_char" }, Param { name: "aArguments", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void registerNotification (in nsIObserver aObserver); */
                    Method {
                        name: "registerNotification",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    /* void unregisterNotification (in nsIObserver aObserver); */
                    Method {
                        name: "unregisterNotification",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator getWindowEnumerator (); */
                    Method {
                        name: "getWindowEnumerator",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsIPrompt getNewPrompter (in mozIDOMWindowProxy aParent); */
                    Method {
                        name: "getNewPrompter",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut *const nsIPrompt" }],
                        ret: "nsresult",
                    },

                    /* nsIAuthPrompt getNewAuthPrompter (in mozIDOMWindowProxy aParent); */
                    Method {
                        name: "getNewAuthPrompter",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut *const nsIAuthPrompt" }],
                        ret: "nsresult",
                    },

                    /* void setWindowCreator (in nsIWindowCreator creator); */
                    Method {
                        name: "setWindowCreator",
                        abi: "C",
                        params: &[Param { name: "creator", ty: "*const nsIWindowCreator" }],
                        ret: "nsresult",
                    },

                    /* boolean hasWindowCreator (); */
                    Method {
                        name: "hasWindowCreator",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIWebBrowserChrome getChromeForWindow (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "getChromeForWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut *const nsIWebBrowserChrome" }],
                        ret: "nsresult",
                    },

                    /* mozIDOMWindowProxy getWindowByName (in AString aTargetName, in mozIDOMWindowProxy aCurrentWindow); */
                    Method {
                        name: "getWindowByName",
                        abi: "C",
                        params: &[Param { name: "aTargetName", ty: "*const nsAString" }, Param { name: "aCurrentWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* attribute mozIDOMWindowProxy activeWindow; */
                    Method {
                        name: "get_activeWindow",
                        abi: "C",
                        params: &[Param { name: "aActiveWindow", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_activeWindow",
                        abi: "C",
                        params: &[Param { name: "aActiveWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

