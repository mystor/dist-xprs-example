//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowMediator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowMediator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISimpleEnumerator getEnumerator (in wstring aWindowType); */
                    Method {
                        name: "getEnumerator",
                        abi: "C",
                        params: &[Param { name: "aWindowType", ty: "*const libc::int16_t" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator getXULWindowEnumerator (in wstring aWindowType); */
                    Method {
                        name: "getXULWindowEnumerator",
                        abi: "C",
                        params: &[Param { name: "aWindowType", ty: "*const libc::int16_t" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator getZOrderDOMWindowEnumerator (in wstring aWindowType, in boolean aFrontToBack); */
                    Method {
                        name: "getZOrderDOMWindowEnumerator",
                        abi: "C",
                        params: &[Param { name: "aWindowType", ty: "*const libc::int16_t" }, Param { name: "aFrontToBack", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator getZOrderXULWindowEnumerator (in wstring aWindowType, in boolean aFrontToBack); */
                    Method {
                        name: "getZOrderXULWindowEnumerator",
                        abi: "C",
                        params: &[Param { name: "aWindowType", ty: "*const libc::int16_t" }, Param { name: "aFrontToBack", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* mozIDOMWindowProxy getMostRecentWindow (in wstring aWindowType); */
                    Method {
                        name: "getMostRecentWindow",
                        abi: "C",
                        params: &[Param { name: "aWindowType", ty: "*const libc::int16_t" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* mozIDOMWindowProxy getOuterWindowWithId (in unsigned long long aOuterWindowID); */
                    Method {
                        name: "getOuterWindowWithId",
                        abi: "C",
                        params: &[Param { name: "aOuterWindowID", ty: "libc::uint64_t" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* mozIDOMWindow getCurrentInnerWindowWithId (in unsigned long long aInnerWindowID); */
                    Method {
                        name: "getCurrentInnerWindowWithId",
                        abi: "C",
                        params: &[Param { name: "aInnerWindowID", ty: "libc::uint64_t" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindow" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void registerWindow (in nsIXULWindow aWindow); */
                    Method {
                        name: "registerWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void unregisterWindow (in nsIXULWindow aWindow); */
                    Method {
                        name: "unregisterWindow",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void updateWindowTimeStamp (in nsIXULWindow aWindow); */
                    Method {
                        name: "updateWindowTimeStamp",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void updateWindowTitle (in nsIXULWindow aWindow, in wstring inTitle); */
                    Method {
                        name: "updateWindowTitle",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIXULWindow" }, Param { name: "inTitle", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] boolean calculateZPosition (in nsIXULWindow inWindow, in unsigned long inPosition, in nsIWidget inBelow, out unsigned long outPosition, out nsIWidget outBelow); */
                    Method {
                        name: "calculateZPosition",
                        abi: "C",
                        params: &[Param { name: "inWindow", ty: "*const nsIXULWindow" }, Param { name: "inPosition", ty: "libc::uint32_t" }, Param { name: "inBelow", ty: "*const nsIWidget" }, Param { name: "outPosition", ty: "*mut libc::uint32_t" }, Param { name: "outBelow", ty: "*mut *const nsIWidget" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void setZPosition (in nsIXULWindow inWindow, in unsigned long inPosition, in nsIXULWindow inBelow); */
                    Method {
                        name: "setZPosition",
                        abi: "C",
                        params: &[Param { name: "inWindow", ty: "*const nsIXULWindow" }, Param { name: "inPosition", ty: "libc::uint32_t" }, Param { name: "inBelow", ty: "*const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    /* [noscript] uint32_t getZLevel (in nsIXULWindow aWindow); */
                    Method {
                        name: "getZLevel",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIXULWindow" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void setZLevel (in nsIXULWindow aWindow, in uint32_t aZLevel); */
                    Method {
                        name: "setZLevel",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIXULWindow" }, Param { name: "aZLevel", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void addListener (in nsIWindowMediatorListener aListener); */
                    Method {
                        name: "addListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIWindowMediatorListener" }],
                        ret: "nsresult",
                    },

                    /* void removeListener (in nsIWindowMediatorListener aListener); */
                    Method {
                        name: "removeListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIWindowMediatorListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWindowMediator_44",
            base: Some("nsIWindowMediator"),
            methods: Some(&[
                    /* mozIDOMWindowProxy getMostRecentNonPBWindow (in wstring aWindowType); */
                    Method {
                        name: "getMostRecentNonPBWindow",
                        abi: "C",
                        params: &[Param { name: "aWindowType", ty: "*const libc::int16_t" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

