//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMemoryReporter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMemoryReporterCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void callback (in ACString process, in AUTF8String path, in int32_t kind, in int32_t units, in int64_t amount, in AUTF8String description, in nsISupports data); */
                    Method {
                        name: "callback",
                        abi: "C",
                        params: &[Param { name: "process", ty: "*const nsACString" }, Param { name: "path", ty: "*const nsACString" }, Param { name: "kind", ty: "int32_t" }, Param { name: "units", ty: "int32_t" }, Param { name: "amount", ty: "int64_t" }, Param { name: "description", ty: "*const nsACString" }, Param { name: "data", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIMemoryReporter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void collectReports (in nsIMemoryReporterCallback callback, in nsISupports data, in boolean anonymize); */
                    Method {
                        name: "collectReports",
                        abi: "C",
                        params: &[Param { name: "callback", ty: "*const nsIMemoryReporterCallback" }, Param { name: "data", ty: "*const nsISupports" }, Param { name: "anonymize", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIFinishReportingCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void callback (in nsISupports data); */
                    Method {
                        name: "callback",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIMemoryReporterManager",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

