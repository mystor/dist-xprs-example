//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHashable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHashable",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean equals (in nsIHashable aOther); */
                    Method {
                        name: "equals",
                        abi: "C",
                        params: &[Param { name: "aOther", ty: "*const nsIHashable" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long hashCode; */
                    Method {
                        name: "get_hashCode",
                        abi: "C",
                        params: &[Param { name: "aHashCode", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

