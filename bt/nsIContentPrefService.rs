//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentPrefService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentPrefObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onContentPrefSet (in AString aGroup, in AString aName, in nsIVariant aValue, [optional] in boolean aIsPrivate); */
                    Method {
                        name: "onContentPrefSet",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const nsAString" }, Param { name: "aName", ty: "*const nsAString" }, Param { name: "aValue", ty: "*const nsIVariant" }, Param { name: "aIsPrivate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void onContentPrefRemoved (in AString aGroup, in AString aName, [optional] in boolean aIsPrivate); */
                    Method {
                        name: "onContentPrefRemoved",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const nsAString" }, Param { name: "aName", ty: "*const nsAString" }, Param { name: "aIsPrivate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIContentPrefCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onResult (in nsIVariant aResult); */
                    Method {
                        name: "onResult",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIContentPrefService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIVariant getPref (in nsIVariant aGroup, in AString aName, in nsILoadContext aPrivacyContext, [optional] in nsIContentPrefCallback aCallback); */
                    Method {
                        name: "getPref",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const nsIVariant" }, Param { name: "aName", ty: "*const nsAString" }, Param { name: "aPrivacyContext", ty: "*const nsILoadContext" }, Param { name: "aCallback", ty: "*const nsIContentPrefCallback" }, Param { name: "_retval", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* void setPref (in nsIVariant aGroup, in AString aName, in nsIVariant aValue, in nsILoadContext aPrivacyContext); */
                    Method {
                        name: "setPref",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const nsIVariant" }, Param { name: "aName", ty: "*const nsAString" }, Param { name: "aValue", ty: "*const nsIVariant" }, Param { name: "aPrivacyContext", ty: "*const nsILoadContext" }],
                        ret: "nsresult",
                    },

                    /* boolean hasPref (in nsIVariant aGroup, in AString aName, in nsILoadContext aContext); */
                    Method {
                        name: "hasPref",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const nsIVariant" }, Param { name: "aName", ty: "*const nsAString" }, Param { name: "aContext", ty: "*const nsILoadContext" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean hasCachedPref (in nsIVariant aGroup, in AString aName, in nsILoadContext aContext); */
                    Method {
                        name: "hasCachedPref",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const nsIVariant" }, Param { name: "aName", ty: "*const nsAString" }, Param { name: "aContext", ty: "*const nsILoadContext" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void removePref (in nsIVariant aGroup, in AString aName, in nsILoadContext aContext); */
                    Method {
                        name: "removePref",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const nsIVariant" }, Param { name: "aName", ty: "*const nsAString" }, Param { name: "aContext", ty: "*const nsILoadContext" }],
                        ret: "nsresult",
                    },

                    /* void removeGroupedPrefs (in nsILoadContext aContext); */
                    Method {
                        name: "removeGroupedPrefs",
                        abi: "C",
                        params: &[Param { name: "aContext", ty: "*const nsILoadContext" }],
                        ret: "nsresult",
                    },

                    /* void removePrefsByName (in AString aName, in nsILoadContext aContext); */
                    Method {
                        name: "removePrefsByName",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aContext", ty: "*const nsILoadContext" }],
                        ret: "nsresult",
                    },

                    /* nsIPropertyBag2 getPrefs (in nsIVariant aGroup, in nsILoadContext aContext); */
                    Method {
                        name: "getPrefs",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const nsIVariant" }, Param { name: "aContext", ty: "*const nsILoadContext" }, Param { name: "_retval", ty: "*mut *const nsIPropertyBag2" }],
                        ret: "nsresult",
                    },

                    /* nsIPropertyBag2 getPrefsByName (in AString aName, in nsILoadContext aContext); */
                    Method {
                        name: "getPrefsByName",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aContext", ty: "*const nsILoadContext" }, Param { name: "_retval", ty: "*mut *const nsIPropertyBag2" }],
                        ret: "nsresult",
                    },

                    /* void addObserver (in AString aName, in nsIContentPrefObserver aObserver); */
                    Method {
                        name: "addObserver",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aObserver", ty: "*const nsIContentPrefObserver" }],
                        ret: "nsresult",
                    },

                    /* void removeObserver (in AString aName, in nsIContentPrefObserver aObserver); */
                    Method {
                        name: "removeObserver",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aObserver", ty: "*const nsIContentPrefObserver" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIContentURIGrouper grouper; */
                    Method {
                        name: "get_grouper",
                        abi: "C",
                        params: &[Param { name: "aGrouper", ty: "*mut *const nsIContentURIGrouper" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute mozIStorageConnection DBConnection; */
                    Method {
                        name: "get_DBConnection",
                        abi: "C",
                        params: &[Param { name: "aDBConnection", ty: "*mut *const mozIStorageConnection" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

