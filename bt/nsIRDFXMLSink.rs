//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFXMLSink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFXMLSinkObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onBeginLoad (in nsIRDFXMLSink aSink); */
                    Method {
                        name: "onBeginLoad",
                        abi: "C",
                        params: &[Param { name: "aSink", ty: "*const nsIRDFXMLSink" }],
                        ret: "nsresult",
                    },

                    /* void onInterrupt (in nsIRDFXMLSink aSink); */
                    Method {
                        name: "onInterrupt",
                        abi: "C",
                        params: &[Param { name: "aSink", ty: "*const nsIRDFXMLSink" }],
                        ret: "nsresult",
                    },

                    /* void onResume (in nsIRDFXMLSink aSink); */
                    Method {
                        name: "onResume",
                        abi: "C",
                        params: &[Param { name: "aSink", ty: "*const nsIRDFXMLSink" }],
                        ret: "nsresult",
                    },

                    /* void onEndLoad (in nsIRDFXMLSink aSink); */
                    Method {
                        name: "onEndLoad",
                        abi: "C",
                        params: &[Param { name: "aSink", ty: "*const nsIRDFXMLSink" }],
                        ret: "nsresult",
                    },

                    /* void onError (in nsIRDFXMLSink aSink, in nsresult aStatus, in wstring aErrorMsg); */
                    Method {
                        name: "onError",
                        abi: "C",
                        params: &[Param { name: "aSink", ty: "*const nsIRDFXMLSink" }, Param { name: "aStatus", ty: "nsresult" }, Param { name: "aErrorMsg", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIRDFXMLSink",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

