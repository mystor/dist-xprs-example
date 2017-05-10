//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINotificationStorage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINotificationStorageCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handle (in DOMString id, in DOMString title, in DOMString dir, in DOMString lang, in DOMString body, in DOMString tag, in DOMString icon, in DOMString data, in DOMString behavior, in DOMString serviceWorkerRegistrationScope); */
                    Method {
                        name: "handle",
                        abi: "C",
                        params: &[Param { name: "id", ty: "*const nsAString" }, Param { name: "title", ty: "*const nsAString" }, Param { name: "dir", ty: "*const nsAString" }, Param { name: "lang", ty: "*const nsAString" }, Param { name: "body", ty: "*const nsAString" }, Param { name: "tag", ty: "*const nsAString" }, Param { name: "icon", ty: "*const nsAString" }, Param { name: "data", ty: "*const nsAString" }, Param { name: "behavior", ty: "*const nsAString" }, Param { name: "serviceWorkerRegistrationScope", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void done (); */
                    Method {
                        name: "done",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINotificationStorage",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void put (in DOMString origin, in DOMString id, in DOMString title, in DOMString dir, in DOMString lang, in DOMString body, in DOMString tag, in DOMString icon, in DOMString alertName, in DOMString data, in DOMString behavior, in DOMString serviceWorkerRegistrationScope); */
                    Method {
                        name: "put",
                        abi: "C",
                        params: &[Param { name: "origin", ty: "*const nsAString" }, Param { name: "id", ty: "*const nsAString" }, Param { name: "title", ty: "*const nsAString" }, Param { name: "dir", ty: "*const nsAString" }, Param { name: "lang", ty: "*const nsAString" }, Param { name: "body", ty: "*const nsAString" }, Param { name: "tag", ty: "*const nsAString" }, Param { name: "icon", ty: "*const nsAString" }, Param { name: "alertName", ty: "*const nsAString" }, Param { name: "data", ty: "*const nsAString" }, Param { name: "behavior", ty: "*const nsAString" }, Param { name: "serviceWorkerRegistrationScope", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void get (in DOMString origin, in DOMString tag, in nsINotificationStorageCallback aCallback); */
                    Method {
                        name: "get",
                        abi: "C",
                        params: &[Param { name: "origin", ty: "*const nsAString" }, Param { name: "tag", ty: "*const nsAString" }, Param { name: "aCallback", ty: "*const nsINotificationStorageCallback" }],
                        ret: "nsresult",
                    },

                    /* void getByID (in DOMString origin, in DOMString id, in nsINotificationStorageCallback aCallback); */
                    Method {
                        name: "getByID",
                        abi: "C",
                        params: &[Param { name: "origin", ty: "*const nsAString" }, Param { name: "id", ty: "*const nsAString" }, Param { name: "aCallback", ty: "*const nsINotificationStorageCallback" }],
                        ret: "nsresult",
                    },

                    /* void delete (in DOMString origin, in DOMString id); */
                    Method {
                        name: "delete",
                        abi: "C",
                        params: &[Param { name: "origin", ty: "*const nsAString" }, Param { name: "id", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* boolean canPut (in DOMString origin); */
                    Method {
                        name: "canPut",
                        abi: "C",
                        params: &[Param { name: "origin", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

