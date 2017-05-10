//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebVTTParserWrapper.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebVTTParserWrapper",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void loadParser (in mozIDOMWindow window); */
                    Method {
                        name: "loadParser",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindow" }],
                        ret: "nsresult",
                    },

                    /* void parse (in ACString data); */
                    Method {
                        name: "parse",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void flush (); */
                    Method {
                        name: "flush",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void watch (in nsIWebVTTListener callback); */
                    Method {
                        name: "watch",
                        abi: "C",
                        params: &[Param { name: "callback", ty: "*const nsIWebVTTListener" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocumentFragment convertCueToDOMTree (in mozIDOMWindow window, in nsISupports cue); */
                    Method {
                        name: "convertCueToDOMTree",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindow" }, Param { name: "cue", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut *const nsIDOMDocumentFragment" }],
                        ret: "nsresult",
                    },

                    /* void processCues (in mozIDOMWindow window, in nsIVariant cues, in nsISupports overlay, in nsISupports controls); */
                    Method {
                        name: "processCues",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindow" }, Param { name: "cues", ty: "*const nsIVariant" }, Param { name: "overlay", ty: "*const nsISupports" }, Param { name: "controls", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

