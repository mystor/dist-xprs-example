//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFormatConverter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFormatConverter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIArray getInputDataFlavors (); */
                    Method {
                        name: "getInputDataFlavors",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* nsIArray getOutputDataFlavors (); */
                    Method {
                        name: "getOutputDataFlavors",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* boolean canConvert (in string aFromDataFlavor, in string aToDataFlavor); */
                    Method {
                        name: "canConvert",
                        abi: "C",
                        params: &[Param { name: "aFromDataFlavor", ty: "*const libc::c_char" }, Param { name: "aToDataFlavor", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void convert (in string aFromDataFlavor, in nsISupports aFromData, in unsigned long aDataLen, in string aToDataFlavor, out nsISupports aToData, out unsigned long aDataToLen); */
                    Method {
                        name: "convert",
                        abi: "C",
                        params: &[Param { name: "aFromDataFlavor", ty: "*const libc::c_char" }, Param { name: "aFromData", ty: "*const nsISupports" }, Param { name: "aDataLen", ty: "libc::uint32_t" }, Param { name: "aToDataFlavor", ty: "*const libc::c_char" }, Param { name: "aToData", ty: "*mut *const nsISupports" }, Param { name: "aDataToLen", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

