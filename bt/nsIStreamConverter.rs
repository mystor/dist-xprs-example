//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamConverter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamConverter",
            base: Some("nsIStreamListener"),
            methods: Some(&[
                    /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aCtxt); */
                    Method {
                        name: "convert",
                        abi: "C",
                        params: &[Param { name: "aFromStream", ty: "*const nsIInputStream" }, Param { name: "aFromType", ty: "*const libc::c_char" }, Param { name: "aToType", ty: "*const libc::c_char" }, Param { name: "aCtxt", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* void asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aCtxt); */
                    Method {
                        name: "asyncConvertData",
                        abi: "C",
                        params: &[Param { name: "aFromType", ty: "*const libc::c_char" }, Param { name: "aToType", ty: "*const libc::c_char" }, Param { name: "aListener", ty: "*const nsIStreamListener" }, Param { name: "aCtxt", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

