//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClipboardDragDropHookList.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClipboardDragDropHookList",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addClipboardDragDropHooks (in nsIClipboardDragDropHooks aHooks); */
                    Method {
                        name: "addClipboardDragDropHooks",
                        abi: "C",
                        params: &[Param { name: "aHooks", ty: "*const nsIClipboardDragDropHooks" }],
                        ret: "nsresult",
                    },

                    /* void removeClipboardDragDropHooks (in nsIClipboardDragDropHooks aHooks); */
                    Method {
                        name: "removeClipboardDragDropHooks",
                        abi: "C",
                        params: &[Param { name: "aHooks", ty: "*const nsIClipboardDragDropHooks" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator getHookEnumerator (); */
                    Method {
                        name: "getHookEnumerator",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

