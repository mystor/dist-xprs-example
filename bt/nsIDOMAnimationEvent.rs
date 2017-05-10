//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMAnimationEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMAnimationEvent",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString animationName; */
                    Method {
                        name: "get_animationName",
                        abi: "C",
                        params: &[Param { name: "aAnimationName", ty: "*mut nsAString" }],
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

