//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMMutationEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMMutationEvent",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMNode relatedNode; */
                    Method {
                        name: "get_relatedNode",
                        abi: "C",
                        params: &[Param { name: "aRelatedNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString prevValue; */
                    Method {
                        name: "get_prevValue",
                        abi: "C",
                        params: &[Param { name: "aPrevValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString newValue; */
                    Method {
                        name: "get_newValue",
                        abi: "C",
                        params: &[Param { name: "aNewValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString attrName; */
                    Method {
                        name: "get_attrName",
                        abi: "C",
                        params: &[Param { name: "aAttrName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned short attrChange; */
                    Method {
                        name: "get_attrChange",
                        abi: "C",
                        params: &[Param { name: "aAttrChange", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void initMutationEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in nsIDOMNode relatedNodeArg, in DOMString prevValueArg, in DOMString newValueArg, in DOMString attrNameArg, in unsigned short attrChangeArg); */
                    Method {
                        name: "initMutationEvent",
                        abi: "C",
                        params: &[Param { name: "typeArg", ty: "*const nsAString" }, Param { name: "canBubbleArg", ty: "bool" }, Param { name: "cancelableArg", ty: "bool" }, Param { name: "relatedNodeArg", ty: "*const nsIDOMNode" }, Param { name: "prevValueArg", ty: "*const nsAString" }, Param { name: "newValueArg", ty: "*const nsAString" }, Param { name: "attrNameArg", ty: "*const nsAString" }, Param { name: "attrChangeArg", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

