//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWritablePropertyBag.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWritablePropertyBag",
            base: Some("nsIPropertyBag"),
            methods: Some(&[
                    /* void setProperty (in AString name, in nsIVariant value); */
                    Method {
                        name: "setProperty",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* void deleteProperty (in AString name); */
                    Method {
                        name: "deleteProperty",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

