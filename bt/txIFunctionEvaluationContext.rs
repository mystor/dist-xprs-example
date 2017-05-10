//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/txIFunctionEvaluationContext.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "txIFunctionEvaluationContext",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute uint32_t position; */
                    Method {
                        name: "get_position",
                        abi: "C",
                        params: &[Param { name: "aPosition", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t size; */
                    Method {
                        name: "get_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode contextNode; */
                    Method {
                        name: "get_contextNode",
                        abi: "C",
                        params: &[Param { name: "aContextNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISupports state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

