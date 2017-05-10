//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserDOMWindow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIOpenURIInFrameParams",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIBrowserDOMWindow",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* mozIDOMWindowProxy openURI (in nsIURI aURI, in mozIDOMWindowProxy aOpener, in short aWhere, in long aFlags); */
                    Method {
                        name: "openURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aOpener", ty: "*const mozIDOMWindowProxy" }, Param { name: "aWhere", ty: "libc::int16_t" }, Param { name: "aFlags", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* nsIFrameLoaderOwner openURIInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in unsigned long long aNextTabParentId); */
                    Method {
                        name: "openURIInFrame",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "params", ty: "*const nsIOpenURIInFrameParams" }, Param { name: "aWhere", ty: "libc::int16_t" }, Param { name: "aFlags", ty: "libc::int32_t" }, Param { name: "aNextTabParentId", ty: "libc::uint64_t" }, Param { name: "_retval", ty: "*mut *const nsIFrameLoaderOwner" }],
                        ret: "nsresult",
                    },

                    /* boolean isTabContentWindow (in nsIDOMWindow aWindow); */
                    Method {
                        name: "isTabContentWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIDOMWindow" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean canClose (); */
                    Method {
                        name: "canClose",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

