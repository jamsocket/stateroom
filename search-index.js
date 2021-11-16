var searchIndex = JSON.parse('{\
"jamsocket":{"doc":"<code>jamsocket-cli</code>: a command-line interface to Jamsocket","t":[18,13,13,3,13,3,8,24,13,13,4,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,5,11,11,11,11,11,11,5,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,5,5,11,11,11,11,11,11,11,11,11,11,11,11,5,11,5,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12],"n":["DEFAULT_ENV","Deploy","Dev","EnvFilter","Login","Opts","Parser","Parser","Register","Serve","SubCommand","add_directive","augment_args","augment_args_for_update","augment_subcommands","augment_subcommands_for_update","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","by_cs","by_id","debug","debug","debug","default","deploy","deref","deref","deref","deref_mut","deref_mut","deref_mut","dev","drop","drop","drop","dynamics","enabled","fmt","fmt","from","from","from","from","from_arg_matches","from_arg_matches","from_default_env","from_env","from_str","get_fd_flags","get_fd_flags","get_fd_flags","has_dynamics","has_subcommand","init","init","init","into","into","into","into_app","into_app","into_app_for_update","into_app_for_update","login","main","max_level_hint","new","new_set_fd_flags","new_set_fd_flags","new_set_fd_flags","on_close","on_enter","on_exit","on_new_span","on_record","parse","parse_from","register","register_callsite","serve","set_fd_flags","set_fd_flags","set_fd_flags","statics","subcommand","to_string","try_from","try_from","try_from","try_from_default_env","try_from_env","try_into","try_into","try_into","try_new","try_parse","try_parse_from","try_update_from","type_id","type_id","type_id","update_from","update_from_arg_matches","update_from_arg_matches","vzip","vzip","vzip","0","0","0"],"q":["jamsocket","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","jamsocket::SubCommand","",""],"d":["<code>RUST_LOG</code> is the default environment variable used by …","","","A <code>Layer</code> which filters spans and events based on a set of …","","","Parse command-line arguments into <code>Self</code>.","Generates the <code>Parser</code> implementation.","","Run a dev server to host a given Jamsocket module.","","Add a filtering directive to this <code>EnvFilter</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns a new <code>EnvFilter</code> from the value of the <code>RUST_LOG</code> …","Returns a new <code>EnvFilter</code> from the value of the given …","","","","","","","","","","","","","","","","","","","","Returns a new <code>EnvFilter</code> from the directives in the given …","","","","","","","","","Parse from <code>std::env::args_os()</code>, exit on error","Parse from iterator, exit on error","","","","","","","","","","","","","Returns a new <code>EnvFilter</code> from the value of the <code>RUST_LOG</code> …","Returns a new <code>EnvFilter</code> from the value of the given …","","","","Returns a new <code>EnvFilter</code> from the directives in the given …","Parse from <code>std::env::args_os()</code>, return Err on error.","Parse from iterator, return Err on error.","Update from iterator, return Err on error.","","","","Update from iterator, exit on error","","","","","","","",""],"i":[1,2,2,0,2,0,0,0,2,2,0,1,3,3,2,2,3,2,1,3,2,1,1,1,3,2,1,1,0,3,2,1,3,2,1,0,3,2,1,1,1,1,1,3,2,1,1,3,2,1,1,1,3,2,1,1,2,3,2,1,3,2,1,3,2,3,2,0,0,1,1,3,2,1,1,1,1,1,1,4,4,0,1,0,3,2,1,1,3,1,3,2,1,1,1,3,2,1,1,4,4,4,3,2,1,4,3,2,3,2,1,5,6,7],"f":[null,null,null,null,null,null,null,null,null,null,null,[[["directive",3]],["envfilter",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[],["envfilter",3]],[[["deploycommand",3]],["result",4,[["error",3]]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[],["result",4,[["error",3]]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],null,[[["metadata",3],["context",3]],["bool",15]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[]],[[]],[[],["envfilter",3]],[[]],[[["argmatches",3]],["option",4,[["opts",3]]]],[[["argmatches",3]],["option",4,[["subcommand",4]]]],[[],["envfilter",3]],[[],["envfilter",3]],[[["str",15]],["result",4,[["envfilter",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],null,[[["str",15]],["bool",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[]],[[]],[[]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[["logincommand",3]],["result",4,[["error",3]]]],[[],["result",6]],[[],["option",4,[["levelfilter",3]]]],[[],["envfilter",3]],[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],[[["id",3],["context",3]]],[[["id",3],["context",3]]],[[["id",3],["context",3]]],[[["attributes",3],["id",3],["context",3]]],[[["id",3],["record",3],["context",3]]],[[]],[[]],[[],["result",4,[["error",3]]]],[[["metadata",3]],["interest",3]],[[["servecommand",3]],["result",4,[["error",3]]]],[[["setfdflags",3]],["result",4,[["error",3]]]],[[["setfdflags",3]],["result",4,[["error",3]]]],[[["setfdflags",3]],["result",4,[["error",3]]]],null,null,[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4,[["envfilter",3],["fromenverror",3]]]],[[],["result",4,[["envfilter",3],["fromenverror",3]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4,[["envfilter",3],["parseerror",3]]]],[[],["result",4,[["error",3]]]],[[],["result",4,[["error",3]]]],[[],["result",4,[["error",3]]]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[["argmatches",3]]],[[["argmatches",3]]],[[]],[[]],[[]],null,null,null],"p":[[3,"EnvFilter"],[4,"SubCommand"],[3,"Opts"],[8,"Parser"],[13,"Serve"],[13,"Login"],[13,"Deploy"]]},\
"jamsocket_api":{"doc":"","t":[17,3,3,3,3,12,11,11,11,11,11,11,11,11,11,12,11,11,11,12,11,11,11,11,11,11,11,11,12,11,11,12,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12],"n":["API_BASE","AuthcheckResponse","CreateServiceResponse","JamsocketApi","UploadServiceResponse","activated","authenticate","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","create_room_url","deserialize","deserialize","deserialize","email","from","from","from","from","into","into","into","into","module","new","new_service","provider","serialize","serialize","serialize","service","service_id","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","upload","url_base","username"],"q":["jamsocket_api","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,1,2,2,3,4,1,2,3,4,1,3,3,4,1,1,2,3,4,1,2,3,4,1,3,2,2,1,3,4,1,3,4,2,3,4,1,2,3,4,1,2,3,4,1,2,3,1],"f":[null,null,null,null,null,null,[[],["result",6,[["option",4,[["authcheckresponse",3]]]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[["str",15]]],[[],["result",6,[["string",3]]]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[["str",15]],["result",6,[["uploadserviceresponse",3]]]],null,null],"p":[[3,"AuthcheckResponse"],[3,"JamsocketApi"],[3,"UploadServiceResponse"],[3,"CreateServiceResponse"]]},\
"jamsocket_cli":{"doc":"","t":[0,5,5,5,5,5,13,3,13,13,3,3,13,13,3,4,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,12,12,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12],"n":["cli_opts","deploy","dev","login","register","serve","Deploy","DeployCommand","Dev","Login","LoginCommand","Opts","Register","Serve","ServeCommand","SubCommand","augment_args","augment_args","augment_args","augment_args","augment_args_for_update","augment_args_for_update","augment_args_for_update","augment_args_for_update","augment_subcommands","augment_subcommands_for_update","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clear","debug","debug","debug","debug","debug","deref","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","drop","drop","drop","drop","drop","from","from","from","from","from","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","get_fd_flags","get_fd_flags","get_fd_flags","get_fd_flags","get_fd_flags","has_subcommand","heartbeat_interval","heartbeat_timeout","init","init","init","init","init","into","into","into","into","into","into_app","into_app","into_app","into_app","into_app","into_app_for_update","into_app_for_update","into_app_for_update","into_app_for_update","into_app_for_update","module","new_set_fd_flags","new_set_fd_flags","new_set_fd_flags","new_set_fd_flags","new_set_fd_flags","port","service_id","set_fd_flags","set_fd_flags","set_fd_flags","set_fd_flags","set_fd_flags","subcommand","token","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","vzip","vzip","vzip","vzip","vzip","0","0","0"],"q":["jamsocket_cli","","","","","","jamsocket_cli::cli_opts","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","jamsocket_cli::cli_opts::SubCommand","",""],"d":["","","","","","","","","","","","","","Run a dev server to host a given Jamsocket module.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The time interval (in seconds) between WebSocket heartbeat …","The duration of time without hearing from a client before …","","","","","","","","","","","","","","","","","","","","","The module (.wasm file) to serve.","","","","","","The port to serve on.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,1,0,1,1,0,0,1,1,0,0,2,3,4,5,2,3,4,5,1,1,2,1,3,4,5,2,1,3,4,5,4,2,1,3,4,5,2,1,3,4,5,2,1,3,4,5,2,1,3,4,5,2,1,3,4,5,2,1,3,4,5,2,1,3,4,5,1,5,5,2,1,3,4,5,2,1,3,4,5,2,1,3,4,5,2,1,3,4,5,5,2,1,3,4,5,5,3,2,1,3,4,5,2,4,2,1,3,4,5,2,1,3,4,5,2,1,3,4,5,2,1,3,4,5,2,1,3,4,5,6,7,8],"f":[null,[[["deploycommand",3]],["result",6]],[[],["result",6]],[[["logincommand",3]],["result",6]],[[],["result",6]],[[["servecommand",3]],["result",6]],null,null,null,null,null,null,null,null,null,null,[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[]],[[]],[[]],[[]],[[]],[[["argmatches",3]],["option",4]],[[["argmatches",3]],["option",4]],[[["argmatches",3]],["option",4]],[[["argmatches",3]],["option",4]],[[["argmatches",3]],["option",4]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],[[["str",15]],["bool",15]],null,null,[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[]],[[]],[[]],[[]],[[]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],null,[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],null,null,[[["setfdflags",3]],["result",4,[["error",3]]]],[[["setfdflags",3]],["result",4,[["error",3]]]],[[["setfdflags",3]],["result",4,[["error",3]]]],[[["setfdflags",3]],["result",4,[["error",3]]]],[[["setfdflags",3]],["result",4,[["error",3]]]],null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[["argmatches",3]]],[[["argmatches",3]]],[[["argmatches",3]]],[[["argmatches",3]]],[[["argmatches",3]]],[[]],[[]],[[]],[[]],[[]],null,null,null],"p":[[4,"SubCommand"],[3,"Opts"],[3,"DeployCommand"],[3,"LoginCommand"],[3,"ServeCommand"],[13,"Serve"],[13,"Login"],[13,"Deploy"]]},\
"jamsocket_server":{"doc":"","t":[3,3,13,13,13,4,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,12,12,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12],"n":["AssignClientId","ClientSocketConnection","Connect","Disconnect","Message","MessageFromClient","MessageFromServer","RoomActor","Server","ServiceActor","ServiceActorContext","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","client_id","clone","clone","clone","clone_into","clone_into","clone_into","data","default","fmt","fmt","from","from","from","from","from","from","from","from","handle","handle","handle","handle","handle","handle","heartbeat_interval","heartbeat_interval","heartbeat_timeout","heartbeat_timeout","interval_handle","into","into","into","into","into","into","into","into","ip","last_seen","new","new","new","new","new_binary","port","room","send_binary","send_message","serve","set_timer","started","stopping","to_client","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","with_heartbeat_interval","with_heartbeat_timeout","with_ip","with_port","0","0","1","data","from_client"],"q":["jamsocket_server","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","jamsocket_server::MessageFromClient","","","",""],"d":["Represents a request to reserve a client ID and return it. …","Represents a connection from a service to a client, which …","A client opens a connection to the server.","A client disconnects from the server (or their connection …","A client sends a message.","Represents a message or event initiated by a client.","Represents a message sent to one or more clients from the …","Actor model representation of a “room”. A room is a …","Settings used by the server.","","A JamsocketContext implementation for JamsocketServices …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The duration of time between server-initiated WebSocket …","","The minimum amount of time between client heartbeats …","","","","","","","","","","The IP to listen on. Defaults to 0.0.0.0.","","","","","","","The port to run the server on. Defaults to 8080.","","","","Start a server given a [JamsocketService].","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,1,1,0,0,0,0,0,0,2,3,4,5,6,1,7,8,2,3,4,5,6,1,7,8,2,1,7,8,1,7,8,7,6,1,7,2,3,4,5,6,1,7,8,2,2,4,4,4,5,2,6,2,6,2,2,3,4,5,6,1,7,8,6,2,4,5,6,7,7,6,2,8,8,6,8,2,5,7,1,7,8,2,3,4,5,6,1,7,8,2,3,4,5,6,1,7,8,2,3,4,5,6,1,7,8,2,3,4,5,6,1,7,8,6,6,6,6,9,10,9,11,11],"f":[null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[],["messagefromclient",4]],[[],["messagefromserver",3]],[[],["serviceactorcontext",3]],[[]],[[]],[[]],null,[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["result",4,[["message",4],["protocolerror",4]]]]],[[["messagefromserver",3]]],[[["messagefromserver",3],["context",3]]],[[["messagefromclient",4],["context",3]]],[[["assignclientid",3],["context",3]],["clientid",3]],[[["messagefromclient",4]]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[["recipient",3,[["messagefromclient",4]]]]],[[["context",3],["recipient",3,[["messagefromserver",3]]]],["option",4]],[[]],[[["messagerecipient",4],["string",3]]],[[["messagerecipient",4],["vec",3,[["u8",15]]]]],null,null,[[]],[[["str",15]]],[[],["result",6]],[[["u32",15]]],[[]],[[],["running",4]],null,[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["u64",15]]],[[["u64",15]]],[[["string",3]]],[[["u32",15]]],null,null,null,null,null],"p":[[4,"MessageFromClient"],[3,"ClientSocketConnection"],[3,"AssignClientId"],[3,"RoomActor"],[3,"ServiceActor"],[3,"Server"],[3,"MessageFromServer"],[3,"ServiceActorContext"],[13,"Connect"],[13,"Disconnect"],[13,"Message"]]},\
"jamsocket_stdio":{"doc":"","t":[3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["StdioProcessService","StdioProcessServiceFactory","binary","borrow","borrow","borrow_mut","borrow_mut","build","connect","disconnect","from","from","into","into","message","new","timer","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["jamsocket_stdio","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,2,1,2,1,2,1,1,2,1,2,1,1,2,1,2,1,2,1,2,1],"f":[null,null,[[["clientid",3]]],[[]],[[]],[[]],[[]],[[["str",15]],["result",4]],[[["clientid",3]]],[[["clientid",3]]],[[]],[[]],[[]],[[]],[[["clientid",3],["str",15]]],[[["str",15]]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[3,"StdioProcessService"],[3,"StdioProcessServiceFactory"]]},\
"jamsocket_wasm":{"doc":"","t":[23,0,13,13,3,16,8,8,8,4,16,8,3,10,11,11,11,11,11,11,11,11,10,11,11,11,11,10,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,23,10,11,11,11,11,10,11,11,10,10,11,11,10,10,11,11,11,11,11,11,11,11,11,11,11,11,11,12],"n":["jamsocket_wasm","prelude","Broadcast","Client","ClientId","Error","JamsocketContext","JamsocketService","JamsocketServiceFactory","MessageRecipient","Service","SimpleJamsocketService","WrappedJamsocketService","binary","binary","binary","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","build","clone","clone","clone_into","clone_into","connect","connect","connect","decode_u32","deserialize","deserialize","disconnect","disconnect","disconnect","encode_u32","eq","eq","fmt","fmt","from","from","from","from","from","hash","into","into","into","jamsocket_wasm","message","message","message","ne","ne","new","new","partial_cmp","send_binary","send_message","serialize","serialize","set_timer","timer","timer","timer","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","0"],"q":["jamsocket_wasm","","jamsocket_wasm::prelude","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","jamsocket_wasm::prelude::MessageRecipient"],"d":["Exposes a <code>jamsocket_wasm::SimpleJamsocketService</code>…","","","","Re-exports useful items from <code>jamsocket</code> and …","","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","The type of JamsocketService that the object implementing …","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","Called each time a client sends a binary message to the …","Called each time a client sends a binary message to the …","","","","","","","","Non-destructively build a JamsocketService from <code>self</code>.","","","","","Called each time a client connects to the service.","Called each time a client connects to the service.","","","","","Called each time a client disconnects from the service, …","Called each time a client disconnects from the service, …","","","","","","","","","","","","","","","","Exposes a <code>jamsocket_wasm::SimpleJamsocketService</code>…","Called each time a client sends a text message to the …","Called each time a client sends a text message to the …","","","","Called when the service is created, before any client has …","","","Sends a binary message to a currently connected user, or …","Sends a message to a currently connected user, or …","","","Sets a timer to wake up the service in the given number of …","Called when JamsocketContext::set_timer has been called on …","Called when JamsocketContext::set_timer has been called on …","","","","","","","","","","","","",""],"i":[0,0,1,1,0,2,0,0,0,0,2,0,0,3,4,5,5,6,1,5,6,1,2,6,1,6,1,3,4,5,1,6,1,3,4,5,1,6,1,6,1,5,6,6,1,1,6,5,6,1,0,3,4,5,6,1,4,5,6,7,7,6,1,7,3,4,5,6,1,5,6,1,5,6,1,5,6,1,8],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,[[["clientid",3]]],[[["clientid",3]]],[[["clientid",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],["result",4]],[[],["clientid",3]],[[],["messagerecipient",4]],[[]],[[]],[[["clientid",3]]],[[["clientid",3]]],[[["clientid",3]]],[[["u32",15]],["messagerecipient",4]],[[],["result",4,[["clientid",3]]]],[[],["result",4,[["messagerecipient",4]]]],[[["clientid",3]]],[[["clientid",3]]],[[["clientid",3]]],[[],["u32",15]],[[["clientid",3]],["bool",15]],[[["messagerecipient",4]],["bool",15]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[]],[[["u32",15]],["clientid",3]],[[]],[[]],[[["clientid",3]],["messagerecipient",4]],[[]],[[]],[[]],[[]],null,[[["clientid",3],["str",15]]],[[["clientid",3],["str",15]]],[[["clientid",3],["str",15]]],[[["clientid",3]],["bool",15]],[[["messagerecipient",4]],["bool",15]],[[["str",15]]],[[],["wrappedjamsocketservice",3]],[[["clientid",3]],["option",4,[["ordering",4]]]],[[]],[[["str",15]]],[[],["result",4]],[[],["result",4]],[[["u32",15]]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null],"p":[[4,"MessageRecipient"],[8,"JamsocketServiceFactory"],[8,"JamsocketService"],[8,"SimpleJamsocketService"],[3,"WrappedJamsocketService"],[3,"ClientId"],[8,"JamsocketContext"],[13,"Client"]]},\
"jamsocket_wasm_host":{"doc":"This module provides a jamsocket::JamsocketService …","t":[13,13,13,13,3,3,4,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["CouldNotImportGlobal","CouldNotImportMemory","InvalidApiVersion","InvalidProtocolVersion","WasmHost","WasmHostFactory","WasmRuntimeError","binary","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","build","clone","clone_into","connect","debug","debug","debug","deref","deref","deref","deref_mut","deref_mut","deref_mut","description","disconnect","drop","drop","drop","fmt","fmt","from","from","from","get_fd_flags","get_fd_flags","get_fd_flags","init","init","init","into","into","into","message","new","new","new_set_fd_flags","new_set_fd_flags","new_set_fd_flags","new_with_shared_module","set_fd_flags","set_fd_flags","set_fd_flags","timer","to_owned","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip"],"q":["jamsocket_wasm_host","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","Hosts a jamsocket::JamsocketService implemented by a …","Loads and caches a WebAssembly module such that a WasmHost …","An error encountered while running WebAssembly.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,1,1,1,0,0,0,2,2,3,1,2,3,1,3,3,3,2,2,3,1,2,3,1,2,3,1,1,2,2,3,1,1,1,2,3,1,2,3,1,2,3,1,2,3,1,2,2,3,2,3,1,3,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1],"f":[null,null,null,null,null,null,null,[[["clientid",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],["result",6]],[[],["wasmhostfactory",3]],[[]],[[["clientid",3]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[],["str",15]],[[["clientid",3]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[]],[[]],[[]],[[["clientid",3],["str",15]]],[[["str",15],["module",3],["engine",3],["arc",3]],["result",6]],[[],["result",6]],[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],[[["fdflags",3]],["result",4,[["setfdflags",3],["error",3]]]],[[["arc",3,[["engine",3]]],["arc",3,[["module",3]]]]],[[["setfdflags",3]],["result",4,[["error",3]]]],[[["setfdflags",3]],["result",4,[["error",3]]]],[[["setfdflags",3]],["result",4,[["error",3]]]],[[]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]]],"p":[[4,"WasmRuntimeError"],[3,"WasmHost"],[3,"WasmHostFactory"]]},\
"jamsocket_wasm_macro":{"doc":"","t":[23],"n":["jamsocket_wasm"],"q":["jamsocket_wasm_macro"],"d":["Exposes a <code>jamsocket_wasm::SimpleJamsocketService</code>…"],"i":[0],"f":[null],"p":[]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};