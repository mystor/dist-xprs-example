//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStatusReporter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStatusReporter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString process; */
                    Method {
                        name: "get_process",
                        abi: "C",
                        params: &[Param { name: "aProcess", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String description; */
                    Method {
                        name: "get_description",
                        abi: "C",
                        params: &[Param { name: "aDescription", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIStatusReporterManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISimpleEnumerator enumerateReporters (); */
                    Method {
                        name: "enumerateReporters",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* void registerReporter (in nsIStatusReporter reporter); */
                    Method {
                        name: "registerReporter",
                        abi: "C",
                        params: &[Param { name: "reporter", ty: "*const nsIStatusReporter" }],
                        ret: "nsresult",
                    },

                    /* void unregisterReporter (in nsIStatusReporter reporter); */
                    Method {
                        name: "unregisterReporter",
                        abi: "C",
                        params: &[Param { name: "reporter", ty: "*const nsIStatusReporter" }],
                        ret: "nsresult",
                    },

                    /* void init (); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void dumpReports (); */
                    Method {
                        name: "dumpReports",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

