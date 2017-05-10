//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMSerializer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMSerializer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AString serializeToString (in nsIDOMNode root); */
                    Method {
                        name: "serializeToString",
                        abi: "C",
                        params: &[Param { name: "root", ty: "*const nsIDOMNode" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void serializeToStream (in nsIDOMNode root, in nsIOutputStream stream, in AUTF8String charset); */
                    Method {
                        name: "serializeToStream",
                        abi: "C",
                        params: &[Param { name: "root", ty: "*const nsIDOMNode" }, Param { name: "stream", ty: "*const nsIOutputStream" }, Param { name: "charset", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

