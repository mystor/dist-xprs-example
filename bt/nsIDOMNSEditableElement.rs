//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNSEditableElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMNSEditableElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [noscript] readonly attribute nsIEditor editor; */
                    Method {
                        name: "get_editor",
                        abi: "C",
                        params: &[Param { name: "aEditor", ty: "*mut *const nsIEditor" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void setUserInput (in DOMString input); */
                    Method {
                        name: "setUserInput",
                        abi: "C",
                        params: &[Param { name: "input", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

