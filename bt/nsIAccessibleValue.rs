//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleValue.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleValue",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute double maximumValue; */
                    Method {
                        name: "get_maximumValue",
                        abi: "C",
                        params: &[Param { name: "aMaximumValue", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double minimumValue; */
                    Method {
                        name: "get_minimumValue",
                        abi: "C",
                        params: &[Param { name: "aMinimumValue", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* attribute double currentValue; */
                    Method {
                        name: "get_currentValue",
                        abi: "C",
                        params: &[Param { name: "aCurrentValue", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_currentValue",
                        abi: "C",
                        params: &[Param { name: "aCurrentValue", ty: "libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute double minimumIncrement; */
                    Method {
                        name: "get_minimumIncrement",
                        abi: "C",
                        params: &[Param { name: "aMinimumIncrement", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

