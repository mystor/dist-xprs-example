//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocumentStateListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocumentStateListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void NotifyDocumentCreated (); */
                    Method {
                        name: "NotifyDocumentCreated",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void NotifyDocumentWillBeDestroyed (); */
                    Method {
                        name: "NotifyDocumentWillBeDestroyed",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void NotifyDocumentStateChanged (in boolean nowDirty); */
                    Method {
                        name: "NotifyDocumentStateChanged",
                        abi: "C",
                        params: &[Param { name: "nowDirty", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

