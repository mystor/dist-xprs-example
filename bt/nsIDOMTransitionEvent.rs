//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMTransitionEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMTransitionEvent",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString propertyName; */
                    Method {
                        name: "get_propertyName",
                        abi: "C",
                        params: &[Param { name: "aPropertyName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute float elapsedTime; */
                    Method {
                        name: "get_elapsedTime",
                        abi: "C",
                        params: &[Param { name: "aElapsedTime", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString pseudoElement; */
                    Method {
                        name: "get_pseudoElement",
                        abi: "C",
                        params: &[Param { name: "aPseudoElement", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

