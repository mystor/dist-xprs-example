//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIToolkitProfile.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProfileLock",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIFile directory; */
                    Method {
                        name: "get_directory",
                        abi: "C",
                        params: &[Param { name: "aDirectory", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile localDirectory; */
                    Method {
                        name: "get_localDirectory",
                        abi: "C",
                        params: &[Param { name: "aLocalDirectory", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime replacedLockTime; */
                    Method {
                        name: "get_replacedLockTime",
                        abi: "C",
                        params: &[Param { name: "aReplacedLockTime", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* void unlock (); */
                    Method {
                        name: "unlock",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIToolkitProfile",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIFile rootDir; */
                    Method {
                        name: "get_rootDir",
                        abi: "C",
                        params: &[Param { name: "aRootDir", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile localDir; */
                    Method {
                        name: "get_localDir",
                        abi: "C",
                        params: &[Param { name: "aLocalDir", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void remove (in boolean removeFiles); */
                    Method {
                        name: "remove",
                        abi: "C",
                        params: &[Param { name: "removeFiles", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* nsIProfileLock lock (out nsIProfileUnlocker aUnlocker); */
                    Method {
                        name: "lock",
                        abi: "C",
                        params: &[Param { name: "aUnlocker", ty: "*mut *const nsIProfileUnlocker" }, Param { name: "_retval", ty: "*mut *const nsIProfileLock" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

