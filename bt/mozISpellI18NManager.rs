//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozISpellI18NManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozISpellI18NManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* mozISpellI18NUtil getUtil (in wstring language); */
                    Method {
                        name: "getUtil",
                        abi: "C",
                        params: &[Param { name: "language", ty: "*const libc::int16_t" }, Param { name: "_retval", ty: "*mut *const mozISpellI18NUtil" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

