//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISliderListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISliderListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void valueChanged (in AString which, in long newValue, in boolean userChanged); */
                    Method {
                        name: "valueChanged",
                        abi: "C",
                        params: &[Param { name: "which", ty: "*const nsAString" }, Param { name: "newValue", ty: "libc::int32_t" }, Param { name: "userChanged", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void dragStateChanged (in boolean isDragging); */
                    Method {
                        name: "dragStateChanged",
                        abi: "C",
                        params: &[Param { name: "isDragging", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

