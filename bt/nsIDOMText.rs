//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMText.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMText",
            base: Some("nsIDOMCharacterData"),
            methods: Some(&[
                    /* nsIDOMText splitText (in unsigned long offset) raises (DOMException); */
                    Method {
                        name: "splitText",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMText" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString wholeText; */
                    Method {
                        name: "get_wholeText",
                        abi: "C",
                        params: &[Param { name: "aWholeText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

