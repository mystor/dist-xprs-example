//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMValidityState.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMValidityState",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean valueMissing; */
                    Method {
                        name: "get_valueMissing",
                        abi: "C",
                        params: &[Param { name: "aValueMissing", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean typeMismatch; */
                    Method {
                        name: "get_typeMismatch",
                        abi: "C",
                        params: &[Param { name: "aTypeMismatch", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean patternMismatch; */
                    Method {
                        name: "get_patternMismatch",
                        abi: "C",
                        params: &[Param { name: "aPatternMismatch", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean tooLong; */
                    Method {
                        name: "get_tooLong",
                        abi: "C",
                        params: &[Param { name: "aTooLong", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean tooShort; */
                    Method {
                        name: "get_tooShort",
                        abi: "C",
                        params: &[Param { name: "aTooShort", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean rangeUnderflow; */
                    Method {
                        name: "get_rangeUnderflow",
                        abi: "C",
                        params: &[Param { name: "aRangeUnderflow", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean rangeOverflow; */
                    Method {
                        name: "get_rangeOverflow",
                        abi: "C",
                        params: &[Param { name: "aRangeOverflow", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean stepMismatch; */
                    Method {
                        name: "get_stepMismatch",
                        abi: "C",
                        params: &[Param { name: "aStepMismatch", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean badInput; */
                    Method {
                        name: "get_badInput",
                        abi: "C",
                        params: &[Param { name: "aBadInput", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean customError; */
                    Method {
                        name: "get_customError",
                        abi: "C",
                        params: &[Param { name: "aCustomError", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean valid; */
                    Method {
                        name: "get_valid",
                        abi: "C",
                        params: &[Param { name: "aValid", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

