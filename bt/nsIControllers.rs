//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIControllers.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIControllers",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIController getControllerForCommand (in string command); */
                    Method {
                        name: "getControllerForCommand",
                        abi: "C",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIController" }],
                        ret: "nsresult",
                    },

                    /* void insertControllerAt (in unsigned long index, in nsIController controller); */
                    Method {
                        name: "insertControllerAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "controller", ty: "*const nsIController" }],
                        ret: "nsresult",
                    },

                    /* nsIController removeControllerAt (in unsigned long index); */
                    Method {
                        name: "removeControllerAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIController" }],
                        ret: "nsresult",
                    },

                    /* nsIController getControllerAt (in unsigned long index); */
                    Method {
                        name: "getControllerAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIController" }],
                        ret: "nsresult",
                    },

                    /* void appendController (in nsIController controller); */
                    Method {
                        name: "appendController",
                        abi: "C",
                        params: &[Param { name: "controller", ty: "*const nsIController" }],
                        ret: "nsresult",
                    },

                    /* void removeController (in nsIController controller); */
                    Method {
                        name: "removeController",
                        abi: "C",
                        params: &[Param { name: "controller", ty: "*const nsIController" }],
                        ret: "nsresult",
                    },

                    /* unsigned long getControllerId (in nsIController controller); */
                    Method {
                        name: "getControllerId",
                        abi: "C",
                        params: &[Param { name: "controller", ty: "*const nsIController" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIController getControllerById (in unsigned long controllerID); */
                    Method {
                        name: "getControllerById",
                        abi: "C",
                        params: &[Param { name: "controllerID", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIController" }],
                        ret: "nsresult",
                    },

                    /* unsigned long getControllerCount (); */
                    Method {
                        name: "getControllerCount",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

