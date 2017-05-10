//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICycleCollectorListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICycleCollectorHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void noteRefCountedObject (in ACString aAddress, in unsigned long aRefCount, in ACString aObjectDescription); */
                    Method {
                        name: "noteRefCountedObject",
                        abi: "C",
                        params: &[Param { name: "aAddress", ty: "*const nsACString" }, Param { name: "aRefCount", ty: "libc::uint32_t" }, Param { name: "aObjectDescription", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void noteGCedObject (in ACString aAddress, in boolean aMarked, in ACString aObjectDescription, in ACString aCompartmentAddress); */
                    Method {
                        name: "noteGCedObject",
                        abi: "C",
                        params: &[Param { name: "aAddress", ty: "*const nsACString" }, Param { name: "aMarked", ty: "bool" }, Param { name: "aObjectDescription", ty: "*const nsACString" }, Param { name: "aCompartmentAddress", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void noteEdge (in ACString aFromAddress, in ACString aToAddress, in ACString aEdgeName); */
                    Method {
                        name: "noteEdge",
                        abi: "C",
                        params: &[Param { name: "aFromAddress", ty: "*const nsACString" }, Param { name: "aToAddress", ty: "*const nsACString" }, Param { name: "aEdgeName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void describeRoot (in ACString aAddress, in unsigned long aKnownEdges); */
                    Method {
                        name: "describeRoot",
                        abi: "C",
                        params: &[Param { name: "aAddress", ty: "*const nsACString" }, Param { name: "aKnownEdges", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void describeGarbage (in ACString aAddress); */
                    Method {
                        name: "describeGarbage",
                        abi: "C",
                        params: &[Param { name: "aAddress", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICycleCollectorLogSink",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsICycleCollectorListener",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

