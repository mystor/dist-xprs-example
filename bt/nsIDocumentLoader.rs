//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocumentLoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocumentLoader",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void stop (); */
                    Method {
                        name: "stop",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISupports container; */
                    Method {
                        name: "get_container",
                        abi: "C",
                        params: &[Param { name: "aContainer", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsILoadGroup loadGroup; */
                    Method {
                        name: "get_loadGroup",
                        abi: "C",
                        params: &[Param { name: "aLoadGroup", ty: "*mut *const nsILoadGroup" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIChannel documentChannel; */
                    Method {
                        name: "get_documentChannel",
                        abi: "C",
                        params: &[Param { name: "aDocumentChannel", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

