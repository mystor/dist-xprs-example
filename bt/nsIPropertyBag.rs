//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPropertyBag.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPropertyBag",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsISimpleEnumerator enumerator; */
                    Method {
                        name: "get_enumerator",
                        abi: "C",
                        params: &[Param { name: "aEnumerator", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsIVariant getProperty (in AString name); */
                    Method {
                        name: "getProperty",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

