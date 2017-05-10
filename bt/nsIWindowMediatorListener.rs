//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowMediatorListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowMediatorListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onWindowTitleChange (in nsIXULWindow window, in wstring newTitle); */
                    Method {
                        name: "onWindowTitleChange",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const nsIXULWindow" }, Param { name: "newTitle", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void onOpenWindow (in nsIXULWindow window); */
                    Method {
                        name: "onOpenWindow",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    /* void onCloseWindow (in nsIXULWindow window); */
                    Method {
                        name: "onCloseWindow",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const nsIXULWindow" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

