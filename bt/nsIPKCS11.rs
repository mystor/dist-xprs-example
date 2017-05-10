//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPKCS11.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPKCS11",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void deleteModule (in AString moduleName); */
                    Method {
                        name: "deleteModule",
                        abi: "C",
                        params: &[Param { name: "moduleName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void addModule (in AString moduleName, in AString libraryFullPath, in long cryptoMechanismFlags, in long cipherFlags); */
                    Method {
                        name: "addModule",
                        abi: "C",
                        params: &[Param { name: "moduleName", ty: "*const nsAString" }, Param { name: "libraryFullPath", ty: "*const nsAString" }, Param { name: "cryptoMechanismFlags", ty: "libc::int32_t" }, Param { name: "cipherFlags", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

