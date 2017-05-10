//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibilityService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibilityService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIAccessible getApplicationAccessible (); */
                    Method {
                        name: "getApplicationAccessible",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessible getAccessibleFor (in nsIDOMNode aNode); */
                    Method {
                        name: "getAccessibleFor",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIDOMNode" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* AString getStringRole (in unsigned long aRole); */
                    Method {
                        name: "getStringRole",
                        abi: "C",
                        params: &[Param { name: "aRole", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsISupports getStringStates (in unsigned long aStates, in unsigned long aExtraStates); */
                    Method {
                        name: "getStringStates",
                        abi: "C",
                        params: &[Param { name: "aStates", ty: "libc::uint32_t" }, Param { name: "aExtraStates", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* AString getStringEventType (in unsigned long aEventType); */
                    Method {
                        name: "getStringEventType",
                        abi: "C",
                        params: &[Param { name: "aEventType", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getStringRelationType (in unsigned long aRelationType); */
                    Method {
                        name: "getStringRelationType",
                        abi: "C",
                        params: &[Param { name: "aRelationType", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessible getAccessibleFromCache (in nsIDOMNode aNode); */
                    Method {
                        name: "getAccessibleFromCache",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIDOMNode" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessiblePivot createAccessiblePivot (in nsIAccessible aRoot); */
                    Method {
                        name: "createAccessiblePivot",
                        abi: "C",
                        params: &[Param { name: "aRoot", ty: "*const nsIAccessible" }, Param { name: "_retval", ty: "*mut *const nsIAccessiblePivot" }],
                        ret: "nsresult",
                    },

                    /* void setLogging (in ACString aModules); */
                    Method {
                        name: "setLogging",
                        abi: "C",
                        params: &[Param { name: "aModules", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean isLogged (in AString aModule); */
                    Method {
                        name: "isLogged",
                        abi: "C",
                        params: &[Param { name: "aModule", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAccessibleRetrieval",
            base: Some("nsIAccessibilityService"),
            methods: Some(&[
                    ]),
        },


        ]; D}

