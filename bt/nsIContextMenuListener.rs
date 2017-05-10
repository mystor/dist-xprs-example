//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContextMenuListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContextMenuListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onShowContextMenu (in unsigned long aContextFlags, in nsIDOMEvent aEvent, in nsIDOMNode aNode); */
                    Method {
                        name: "onShowContextMenu",
                        abi: "C",
                        params: &[Param { name: "aContextFlags", ty: "libc::uint32_t" }, Param { name: "aEvent", ty: "*const nsIDOMEvent" }, Param { name: "aNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

