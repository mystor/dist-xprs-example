//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIColorPicker.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIColorPickerShownCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void update (in AString color); */
                    Method {
                        name: "update",
                        abi: "C",
                        params: &[Param { name: "color", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void done (in AString color); */
                    Method {
                        name: "done",
                        abi: "C",
                        params: &[Param { name: "color", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIColorPicker",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in mozIDOMWindowProxy parent, in AString title, in AString initialColor); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "title", ty: "*const nsAString" }, Param { name: "initialColor", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void open (in nsIColorPickerShownCallback aColorPickerShownCallback); */
                    Method {
                        name: "open",
                        abi: "C",
                        params: &[Param { name: "aColorPickerShownCallback", ty: "*const nsIColorPickerShownCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

