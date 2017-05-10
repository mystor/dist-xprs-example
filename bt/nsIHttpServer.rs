//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpServer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpServer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void start (in long port); */
                    Method {
                        name: "start",
                        abi: "C",
                        params: &[Param { name: "port", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void stop (in nsIHttpServerStoppedCallback callback); */
                    Method {
                        name: "stop",
                        abi: "C",
                        params: &[Param { name: "callback", ty: "*const nsIHttpServerStoppedCallback" }],
                        ret: "nsresult",
                    },

                    /* void registerFile (in string path, in nsIFile file); */
                    Method {
                        name: "registerFile",
                        abi: "C",
                        params: &[Param { name: "path", ty: "*const libc::c_char" }, Param { name: "file", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void registerPathHandler (in string path, in nsIHttpRequestHandler handler); */
                    Method {
                        name: "registerPathHandler",
                        abi: "C",
                        params: &[Param { name: "path", ty: "*const libc::c_char" }, Param { name: "handler", ty: "*const nsIHttpRequestHandler" }],
                        ret: "nsresult",
                    },

                    /* void registerPrefixHandler (in string prefix, in nsIHttpRequestHandler handler); */
                    Method {
                        name: "registerPrefixHandler",
                        abi: "C",
                        params: &[Param { name: "prefix", ty: "*const libc::c_char" }, Param { name: "handler", ty: "*const nsIHttpRequestHandler" }],
                        ret: "nsresult",
                    },

                    /* void registerErrorHandler (in unsigned long code, in nsIHttpRequestHandler handler); */
                    Method {
                        name: "registerErrorHandler",
                        abi: "C",
                        params: &[Param { name: "code", ty: "libc::uint32_t" }, Param { name: "handler", ty: "*const nsIHttpRequestHandler" }],
                        ret: "nsresult",
                    },

                    /* void registerDirectory (in string path, in nsIFile dir); */
                    Method {
                        name: "registerDirectory",
                        abi: "C",
                        params: &[Param { name: "path", ty: "*const libc::c_char" }, Param { name: "dir", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void registerContentType (in string extension, in string type); */
                    Method {
                        name: "registerContentType",
                        abi: "C",
                        params: &[Param { name: "extension", ty: "*const libc::c_char" }, Param { name: "type_", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void setIndexHandler (in nsIHttpRequestHandler handler); */
                    Method {
                        name: "setIndexHandler",
                        abi: "C",
                        params: &[Param { name: "handler", ty: "*const nsIHttpRequestHandler" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIHttpServerIdentity identity; */
                    Method {
                        name: "get_identity",
                        abi: "C",
                        params: &[Param { name: "aIdentity", ty: "*mut *const nsIHttpServerIdentity" }],
                        ret: "nsresult",
                    },

                    /* AString getState (in AString path, in AString key); */
                    Method {
                        name: "getState",
                        abi: "C",
                        params: &[Param { name: "path", ty: "*const nsAString" }, Param { name: "key", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setState (in AString path, in AString key, in AString value); */
                    Method {
                        name: "setState",
                        abi: "C",
                        params: &[Param { name: "path", ty: "*const nsAString" }, Param { name: "key", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getSharedState (in AString key); */
                    Method {
                        name: "getSharedState",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setSharedState (in AString key, in AString value); */
                    Method {
                        name: "setSharedState",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsISupports getObjectState (in AString key); */
                    Method {
                        name: "getObjectState",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void setObjectState (in AString key, in nsISupports value); */
                    Method {
                        name: "setObjectState",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIHttpServerStoppedCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onStopped (); */
                    Method {
                        name: "onStopped",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIHttpServerIdentity",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute string primaryScheme; */
                    Method {
                        name: "get_primaryScheme",
                        abi: "C",
                        params: &[Param { name: "aPrimaryScheme", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string primaryHost; */
                    Method {
                        name: "get_primaryHost",
                        abi: "C",
                        params: &[Param { name: "aPrimaryHost", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long primaryPort; */
                    Method {
                        name: "get_primaryPort",
                        abi: "C",
                        params: &[Param { name: "aPrimaryPort", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void add (in string scheme, in string host, in long port); */
                    Method {
                        name: "add",
                        abi: "C",
                        params: &[Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "host", ty: "*const libc::c_char" }, Param { name: "port", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean remove (in string scheme, in string host, in long port); */
                    Method {
                        name: "remove",
                        abi: "C",
                        params: &[Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "host", ty: "*const libc::c_char" }, Param { name: "port", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean has (in string scheme, in string host, in long port); */
                    Method {
                        name: "has",
                        abi: "C",
                        params: &[Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "host", ty: "*const libc::c_char" }, Param { name: "port", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* string getScheme (in string host, in long port); */
                    Method {
                        name: "getScheme",
                        abi: "C",
                        params: &[Param { name: "host", ty: "*const libc::c_char" }, Param { name: "port", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void setPrimary (in string scheme, in string host, in long port); */
                    Method {
                        name: "setPrimary",
                        abi: "C",
                        params: &[Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "host", ty: "*const libc::c_char" }, Param { name: "port", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIHttpRequestHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handle (in nsIHttpRequest request, in nsIHttpResponse response); */
                    Method {
                        name: "handle",
                        abi: "C",
                        params: &[Param { name: "request", ty: "*const nsIHttpRequest" }, Param { name: "response", ty: "*const nsIHttpResponse" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIHttpRequest",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute string method; */
                    Method {
                        name: "get_method",
                        abi: "C",
                        params: &[Param { name: "aMethod", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string scheme; */
                    Method {
                        name: "get_scheme",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string host; */
                    Method {
                        name: "get_host",
                        abi: "C",
                        params: &[Param { name: "aHost", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long port; */
                    Method {
                        name: "get_port",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string path; */
                    Method {
                        name: "get_path",
                        abi: "C",
                        params: &[Param { name: "aPath", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string queryString; */
                    Method {
                        name: "get_queryString",
                        abi: "C",
                        params: &[Param { name: "aQueryString", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string httpVersion; */
                    Method {
                        name: "get_httpVersion",
                        abi: "C",
                        params: &[Param { name: "aHttpVersion", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* string getHeader (in string fieldName); */
                    Method {
                        name: "getHeader",
                        abi: "C",
                        params: &[Param { name: "fieldName", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* boolean hasHeader (in string fieldName); */
                    Method {
                        name: "hasHeader",
                        abi: "C",
                        params: &[Param { name: "fieldName", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator headers; */
                    Method {
                        name: "get_headers",
                        abi: "C",
                        params: &[Param { name: "aHeaders", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIInputStream bodyInputStream; */
                    Method {
                        name: "get_bodyInputStream",
                        abi: "C",
                        params: &[Param { name: "aBodyInputStream", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIHttpResponse",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setStatusLine (in string httpVersion, in unsigned short statusCode, in string description); */
                    Method {
                        name: "setStatusLine",
                        abi: "C",
                        params: &[Param { name: "httpVersion", ty: "*const libc::c_char" }, Param { name: "statusCode", ty: "libc::uint16_t" }, Param { name: "description", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void setHeader (in string name, in string value, in boolean merge); */
                    Method {
                        name: "setHeader",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }, Param { name: "merge", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setHeaderNoCheck (in string name, in string value); */
                    Method {
                        name: "setHeaderNoCheck",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIOutputStream bodyOutputStream; */
                    Method {
                        name: "get_bodyOutputStream",
                        abi: "C",
                        params: &[Param { name: "aBodyOutputStream", ty: "*mut *const nsIOutputStream" }],
                        ret: "nsresult",
                    },

                    /* void write (in string data); */
                    Method {
                        name: "write",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void processAsync (); */
                    Method {
                        name: "processAsync",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void seizePower (); */
                    Method {
                        name: "seizePower",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void finish (); */
                    Method {
                        name: "finish",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}

