//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamConverterService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamConverterService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean canConvert (in string aFromType, in string aToType); */
                    Method {
                        name: "canConvert",
                        abi: "C",
                        params: &[Param { name: "aFromType", ty: "*const libc::c_char" }, Param { name: "aToType", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aContext); */
                    Method {
                        name: "convert",
                        abi: "C",
                        params: &[Param { name: "aFromStream", ty: "*const nsIInputStream" }, Param { name: "aFromType", ty: "*const libc::c_char" }, Param { name: "aToType", ty: "*const libc::c_char" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* nsIStreamListener asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aContext); */
                    Method {
                        name: "asyncConvertData",
                        abi: "C",
                        params: &[Param { name: "aFromType", ty: "*const libc::c_char" }, Param { name: "aToType", ty: "*const libc::c_char" }, Param { name: "aListener", ty: "*const nsIStreamListener" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut *const nsIStreamListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

