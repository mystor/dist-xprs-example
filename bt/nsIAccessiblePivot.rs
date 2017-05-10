//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessiblePivot.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessiblePivot",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIAccessiblePivotObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onPivotChanged (in nsIAccessiblePivot aPivot, in nsIAccessible aOldAccessible, in long aOldStart, in long aOldEnd, in PivotMoveReason aReason, in boolean aIsFromUserInput); */
                    Method {
                        name: "onPivotChanged",
                        abi: "C",
                        params: &[Param { name: "aPivot", ty: "*const nsIAccessiblePivot" }, Param { name: "aOldAccessible", ty: "*const nsIAccessible" }, Param { name: "aOldStart", ty: "libc::int32_t" }, Param { name: "aOldEnd", ty: "libc::int32_t" }, Param { name: "aReason", ty: "PivotMoveReason" }, Param { name: "aIsFromUserInput", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAccessibleTraversalRule",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}

