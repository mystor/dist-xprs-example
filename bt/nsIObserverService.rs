//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIObserverService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIObserverService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addObserver (in nsIObserver anObserver, in string aTopic, [optional] in boolean ownsWeak); */
                    Method {
                        name: "addObserver",
                        abi: "C",
                        params: &[Param { name: "anObserver", ty: "*const nsIObserver" }, Param { name: "aTopic", ty: "*const libc::c_char" }, Param { name: "ownsWeak", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void removeObserver (in nsIObserver anObserver, in string aTopic); */
                    Method {
                        name: "removeObserver",
                        abi: "C",
                        params: &[Param { name: "anObserver", ty: "*const nsIObserver" }, Param { name: "aTopic", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void notifyObservers (in nsISupports aSubject, in string aTopic, [optional] in wstring someData); */
                    Method {
                        name: "notifyObservers",
                        abi: "C",
                        params: &[Param { name: "aSubject", ty: "*const nsISupports" }, Param { name: "aTopic", ty: "*const libc::c_char" }, Param { name: "someData", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator enumerateObservers (in string aTopic); */
                    Method {
                        name: "enumerateObservers",
                        abi: "C",
                        params: &[Param { name: "aTopic", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

