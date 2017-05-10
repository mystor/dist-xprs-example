//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISystemStatusBar.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISystemStatusBar",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addItem (in nsIDOMElement aDOMMenuElement); */
                    Method {
                        name: "addItem",
                        abi: "C",
                        params: &[Param { name: "aDOMMenuElement", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void removeItem (in nsIDOMElement aDOMMenuElement); */
                    Method {
                        name: "removeItem",
                        abi: "C",
                        params: &[Param { name: "aDOMMenuElement", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

