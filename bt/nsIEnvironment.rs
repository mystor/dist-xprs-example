//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEnvironment.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEnvironment",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void set (in AString aName, in AString aValue); */
                    Method {
                        name: "set",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString get (in AString aName); */
                    Method {
                        name: "get",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* boolean exists (in AString aName); */
                    Method {
                        name: "exists",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

