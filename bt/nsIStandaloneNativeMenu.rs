//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStandaloneNativeMenu.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStandaloneNativeMenu",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in nsIDOMElement aDOMElement); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aDOMElement", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* boolean menuWillOpen (); */
                    Method {
                        name: "menuWillOpen",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] readonly attribute voidPtr nativeMenu; */
                    Method {
                        name: "get_nativeMenu",
                        abi: "C",
                        params: &[Param { name: "aNativeMenu", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void activateNativeMenuItemAt (in AString anIndexString); */
                    Method {
                        name: "activateNativeMenuItemAt",
                        abi: "C",
                        params: &[Param { name: "anIndexString", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void forceUpdateNativeMenuAt (in AString anIndexString); */
                    Method {
                        name: "forceUpdateNativeMenuAt",
                        abi: "C",
                        params: &[Param { name: "anIndexString", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

