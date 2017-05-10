//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClipboardDragDropHooks.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClipboardDragDropHooks",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean allowStartDrag (in nsIDOMEvent event); */
                    Method {
                        name: "allowStartDrag",
                        abi: "C",
                        params: &[Param { name: "event", ty: "*const nsIDOMEvent" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean allowDrop (in nsIDOMEvent event, in nsIDragSession session); */
                    Method {
                        name: "allowDrop",
                        abi: "C",
                        params: &[Param { name: "event", ty: "*const nsIDOMEvent" }, Param { name: "session", ty: "*const nsIDragSession" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean onCopyOrDrag (in nsIDOMEvent aEvent, in nsITransferable trans); */
                    Method {
                        name: "onCopyOrDrag",
                        abi: "C",
                        params: &[Param { name: "aEvent", ty: "*const nsIDOMEvent" }, Param { name: "trans", ty: "*const nsITransferable" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean onPasteOrDrop (in nsIDOMEvent event, in nsITransferable trans); */
                    Method {
                        name: "onPasteOrDrop",
                        abi: "C",
                        params: &[Param { name: "event", ty: "*const nsIDOMEvent" }, Param { name: "trans", ty: "*const nsITransferable" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

