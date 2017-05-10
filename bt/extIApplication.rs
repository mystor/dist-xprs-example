//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/extIApplication.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "extIConsole",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void log (in AString aMsg); */
                    Method {
                        name: "log",
                        abi: "C",
                        params: &[Param { name: "aMsg", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void open (); */
                    Method {
                        name: "open",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "extIEventItem",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIVariant data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* void preventDefault (); */
                    Method {
                        name: "preventDefault",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "extIEventListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleEvent (in extIEventItem aEvent); */
                    Method {
                        name: "handleEvent",
                        abi: "C",
                        params: &[Param { name: "aEvent", ty: "*const extIEventItem" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "extIEvents",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addListener (in AString aEvent, in extIEventListener aListener); */
                    Method {
                        name: "addListener",
                        abi: "C",
                        params: &[Param { name: "aEvent", ty: "*const nsAString" }, Param { name: "aListener", ty: "*const extIEventListener" }],
                        ret: "nsresult",
                    },

                    /* void removeListener (in AString aEvent, in extIEventListener aListener); */
                    Method {
                        name: "removeListener",
                        abi: "C",
                        params: &[Param { name: "aEvent", ty: "*const nsAString" }, Param { name: "aListener", ty: "*const extIEventListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "extIPreferenceBranch",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString root; */
                    Method {
                        name: "get_root",
                        abi: "C",
                        params: &[Param { name: "aRoot", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIVariant all; */
                    Method {
                        name: "get_all",
                        abi: "C",
                        params: &[Param { name: "aAll", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute extIEvents events; */
                    Method {
                        name: "get_events",
                        abi: "C",
                        params: &[Param { name: "aEvents", ty: "*mut *const extIEvents" }],
                        ret: "nsresult",
                    },

                    /* boolean has (in AString aName); */
                    Method {
                        name: "has",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* extIPreference get (in AString aName); */
                    Method {
                        name: "get",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const extIPreference" }],
                        ret: "nsresult",
                    },

                    /* nsIVariant getValue (in AString aName, in nsIVariant aDefaultValue); */
                    Method {
                        name: "getValue",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aDefaultValue", ty: "*const nsIVariant" }, Param { name: "_retval", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* void setValue (in AString aName, in nsIVariant aValue); */
                    Method {
                        name: "setValue",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aValue", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* void reset (); */
                    Method {
                        name: "reset",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "extIPreference",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIVariant value; */
                    Method {
                        name: "get_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean locked; */
                    Method {
                        name: "get_locked",
                        abi: "C",
                        params: &[Param { name: "aLocked", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_locked",
                        abi: "C",
                        params: &[Param { name: "aLocked", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean modified; */
                    Method {
                        name: "get_modified",
                        abi: "C",
                        params: &[Param { name: "aModified", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute extIPreferenceBranch branch; */
                    Method {
                        name: "get_branch",
                        abi: "C",
                        params: &[Param { name: "aBranch", ty: "*mut *const extIPreferenceBranch" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute extIEvents events; */
                    Method {
                        name: "get_events",
                        abi: "C",
                        params: &[Param { name: "aEvents", ty: "*mut *const extIEvents" }],
                        ret: "nsresult",
                    },

                    /* void reset (); */
                    Method {
                        name: "reset",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "extIExtension",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean enabled; */
                    Method {
                        name: "get_enabled",
                        abi: "C",
                        params: &[Param { name: "aEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString version; */
                    Method {
                        name: "get_version",
                        abi: "C",
                        params: &[Param { name: "aVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean firstRun; */
                    Method {
                        name: "get_firstRun",
                        abi: "C",
                        params: &[Param { name: "aFirstRun", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute extIPreferenceBranch prefs; */
                    Method {
                        name: "get_prefs",
                        abi: "C",
                        params: &[Param { name: "aPrefs", ty: "*mut *const extIPreferenceBranch" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute extISessionStorage storage; */
                    Method {
                        name: "get_storage",
                        abi: "C",
                        params: &[Param { name: "aStorage", ty: "*mut *const extISessionStorage" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute extIEvents events; */
                    Method {
                        name: "get_events",
                        abi: "C",
                        params: &[Param { name: "aEvents", ty: "*mut *const extIEvents" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "extIExtensions",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIVariant all; */
                    Method {
                        name: "get_all",
                        abi: "C",
                        params: &[Param { name: "aAll", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* boolean has (in AString aId); */
                    Method {
                        name: "has",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* extIExtension get (in AString aId); */
                    Method {
                        name: "get",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const extIExtension" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "extIExtensionsCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void callback (in nsIVariant extensions); */
                    Method {
                        name: "callback",
                        abi: "C",
                        params: &[Param { name: "extensions", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "extISessionStorage",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute extIEvents events; */
                    Method {
                        name: "get_events",
                        abi: "C",
                        params: &[Param { name: "aEvents", ty: "*mut *const extIEvents" }],
                        ret: "nsresult",
                    },

                    /* boolean has (in AString aName); */
                    Method {
                        name: "has",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void set (in AString aName, in nsIVariant aValue); */
                    Method {
                        name: "set",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aValue", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* nsIVariant get (in AString aName, in nsIVariant aDefaultValue); */
                    Method {
                        name: "get",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aDefaultValue", ty: "*const nsIVariant" }, Param { name: "_retval", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "extIApplication",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString version; */
                    Method {
                        name: "get_version",
                        abi: "C",
                        params: &[Param { name: "aVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute extIConsole console; */
                    Method {
                        name: "get_console",
                        abi: "C",
                        params: &[Param { name: "aConsole", ty: "*mut *const extIConsole" }],
                        ret: "nsresult",
                    },

                    /* void getExtensions (in extIExtensionsCallback aCallback); */
                    Method {
                        name: "getExtensions",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const extIExtensionsCallback" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute extIPreferenceBranch prefs; */
                    Method {
                        name: "get_prefs",
                        abi: "C",
                        params: &[Param { name: "aPrefs", ty: "*mut *const extIPreferenceBranch" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute extISessionStorage storage; */
                    Method {
                        name: "get_storage",
                        abi: "C",
                        params: &[Param { name: "aStorage", ty: "*mut *const extISessionStorage" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute extIEvents events; */
                    Method {
                        name: "get_events",
                        abi: "C",
                        params: &[Param { name: "aEvents", ty: "*mut *const extIEvents" }],
                        ret: "nsresult",
                    },

                    /* boolean quit (); */
                    Method {
                        name: "quit",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean restart (); */
                    Method {
                        name: "restart",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

