//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMemoryInfoDumper.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFinishDumpingCallback",
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
            name: "nsIDumpGCAndCCLogsCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onDump (in nsIFile aGCLog, in nsIFile aCCLog, in bool aIsParent); */
                    Method {
                        name: "onDump",
                        abi: "C",
                        params: &[Param { name: "aGCLog", ty: "*const nsIFile" }, Param { name: "aCCLog", ty: "*const nsIFile" }, Param { name: "aIsParent", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void onFinish (); */
                    Method {
                        name: "onFinish",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIMemoryInfoDumper",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void dumpMemoryReportsToNamedFile (in AString aFilename, in nsIFinishDumpingCallback aFinishDumping, in nsISupports aFinishDumpingData, in boolean aAnonymize); */
                    Method {
                        name: "dumpMemoryReportsToNamedFile",
                        abi: "C",
                        params: &[Param { name: "aFilename", ty: "*const nsAString" }, Param { name: "aFinishDumping", ty: "*const nsIFinishDumpingCallback" }, Param { name: "aFinishDumpingData", ty: "*const nsISupports" }, Param { name: "aAnonymize", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void dumpMemoryInfoToTempDir (in AString aIdentifier, in boolean aAnonymize, in boolean aMinimizeMemoryUsage); */
                    Method {
                        name: "dumpMemoryInfoToTempDir",
                        abi: "C",
                        params: &[Param { name: "aIdentifier", ty: "*const nsAString" }, Param { name: "aAnonymize", ty: "bool" }, Param { name: "aMinimizeMemoryUsage", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void dumpGCAndCCLogsToFile (in AString aIdentifier, in bool aDumpAllTraces, in bool aDumpChildProcesses, in nsIDumpGCAndCCLogsCallback aCallback); */
                    Method {
                        name: "dumpGCAndCCLogsToFile",
                        abi: "C",
                        params: &[Param { name: "aIdentifier", ty: "*const nsAString" }, Param { name: "aDumpAllTraces", ty: "bool" }, Param { name: "aDumpChildProcesses", ty: "bool" }, Param { name: "aCallback", ty: "*const nsIDumpGCAndCCLogsCallback" }],
                        ret: "nsresult",
                    },

                    /* void dumpGCAndCCLogsToSink (in bool aDumpAllTraces, in nsICycleCollectorLogSink aSink); */
                    Method {
                        name: "dumpGCAndCCLogsToSink",
                        abi: "C",
                        params: &[Param { name: "aDumpAllTraces", ty: "bool" }, Param { name: "aSink", ty: "*const nsICycleCollectorLogSink" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

