//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPIWindowWatcher.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsPIWindowWatcher",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addWindow (in mozIDOMWindowProxy aWindow, in nsIWebBrowserChrome aChrome); */
                    Method {
                        name: "addWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "aChrome", ty: "*const nsIWebBrowserChrome" }],
                        ret: "nsresult",
                    },

                    /* void removeWindow (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "removeWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* mozIDOMWindowProxy openWindow2 (in mozIDOMWindowProxy aParent, in string aUrl, in string aName, in string aFeatures, in boolean aCalledFromScript, in boolean aDialog, in boolean aNavigate, in nsISupports aArgs, in boolean aIsPopupSpam, in boolean aForceNoOpener, in nsIDocShellLoadInfo aLoadInfo); */
                    Method {
                        name: "openWindow2",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "aUrl", ty: "*const libc::c_char" }, Param { name: "aName", ty: "*const libc::c_char" }, Param { name: "aFeatures", ty: "*const libc::c_char" }, Param { name: "aCalledFromScript", ty: "bool" }, Param { name: "aDialog", ty: "bool" }, Param { name: "aNavigate", ty: "bool" }, Param { name: "aArgs", ty: "*const nsISupports" }, Param { name: "aIsPopupSpam", ty: "bool" }, Param { name: "aForceNoOpener", ty: "bool" }, Param { name: "aLoadInfo", ty: "*const nsIDocShellLoadInfo" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* nsITabParent openWindowWithTabParent (in nsITabParent aOpeningTab, in ACString aFeatures, in boolean aCalledFromJS, in float aOpenerFullZoom, in unsigned long long aNextTabParentId); */
                    Method {
                        name: "openWindowWithTabParent",
                        abi: "C",
                        params: &[Param { name: "aOpeningTab", ty: "*const nsITabParent" }, Param { name: "aFeatures", ty: "*const nsACString" }, Param { name: "aCalledFromJS", ty: "bool" }, Param { name: "aOpenerFullZoom", ty: "libc::c_float" }, Param { name: "aNextTabParentId", ty: "libc::uint64_t" }, Param { name: "_retval", ty: "*mut *const nsITabParent" }],
                        ret: "nsresult",
                    },

                    /* nsIDocShellTreeItem findItemWithName (in AString aName, in nsIDocShellTreeItem aRequestor, in nsIDocShellTreeItem aOriginalRequestor); */
                    Method {
                        name: "findItemWithName",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aRequestor", ty: "*const nsIDocShellTreeItem" }, Param { name: "aOriginalRequestor", ty: "*const nsIDocShellTreeItem" }, Param { name: "_retval", ty: "*mut *const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

