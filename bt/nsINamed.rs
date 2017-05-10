//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINamed.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINamed",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void setName (in string aName); */
                    Method {
                        name: "setName",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

