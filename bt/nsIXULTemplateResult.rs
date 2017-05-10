//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULTemplateResult.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULTemplateResult",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean isContainer; */
                    Method {
                        name: "get_isContainer",
                        abi: "C",
                        params: &[Param { name: "aIsContainer", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isEmpty; */
                    Method {
                        name: "get_isEmpty",
                        abi: "C",
                        params: &[Param { name: "aIsEmpty", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean mayProcessChildren; */
                    Method {
                        name: "get_mayProcessChildren",
                        abi: "C",
                        params: &[Param { name: "aMayProcessChildren", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIRDFResource resource; */
                    Method {
                        name: "get_resource",
                        abi: "C",
                        params: &[Param { name: "aResource", ty: "*mut *const nsIRDFResource" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getBindingFor (in nsIAtom aVar); */
                    Method {
                        name: "getBindingFor",
                        abi: "C",
                        params: &[Param { name: "aVar", ty: "*const nsIAtom" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsISupports getBindingObjectFor (in nsIAtom aVar); */
                    Method {
                        name: "getBindingObjectFor",
                        abi: "C",
                        params: &[Param { name: "aVar", ty: "*const nsIAtom" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void ruleMatched (in nsISupports aQuery, in nsIDOMNode aRuleNode); */
                    Method {
                        name: "ruleMatched",
                        abi: "C",
                        params: &[Param { name: "aQuery", ty: "*const nsISupports" }, Param { name: "aRuleNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void hasBeenRemoved (); */
                    Method {
                        name: "hasBeenRemoved",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

