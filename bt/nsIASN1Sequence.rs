//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIASN1Sequence.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIASN1Sequence",
            base: Some("nsIASN1Object"),
            methods: Some(&[
                    /* attribute nsIMutableArray ASN1Objects; */
                    Method {
                        name: "get_ASN1Objects",
                        abi: "C",
                        params: &[Param { name: "aASN1Objects", ty: "*mut *const nsIMutableArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_ASN1Objects",
                        abi: "C",
                        params: &[Param { name: "aASN1Objects", ty: "*const nsIMutableArray" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean isValidContainer; */
                    Method {
                        name: "get_isValidContainer",
                        abi: "C",
                        params: &[Param { name: "aIsValidContainer", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_isValidContainer",
                        abi: "C",
                        params: &[Param { name: "aIsValidContainer", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean isExpanded; */
                    Method {
                        name: "get_isExpanded",
                        abi: "C",
                        params: &[Param { name: "aIsExpanded", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_isExpanded",
                        abi: "C",
                        params: &[Param { name: "aIsExpanded", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

