//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleHideEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleHideEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Some(&[
                    /* readonly attribute nsIAccessible targetParent; */
                    Method {
                        name: "get_targetParent",
                        abi: "C",
                        params: &[Param { name: "aTargetParent", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessible targetNextSibling; */
                    Method {
                        name: "get_targetNextSibling",
                        abi: "C",
                        params: &[Param { name: "aTargetNextSibling", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessible targetPrevSibling; */
                    Method {
                        name: "get_targetPrevSibling",
                        abi: "C",
                        params: &[Param { name: "aTargetPrevSibling", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

