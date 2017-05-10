//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIOUtil.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIOUtil",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean inputStreamIsBuffered (in nsIInputStream aStream); */
                    Method {
                        name: "inputStreamIsBuffered",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean outputStreamIsBuffered (in nsIOutputStream aStream); */
                    Method {
                        name: "outputStreamIsBuffered",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIOutputStream" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

