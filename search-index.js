var searchIndex = JSON.parse('{\
"jamsocket":{"doc":"Jamsocket is a minimalist framework for developing …","t":[13,8,8,8,4,16,8,13,3,10,11,11,11,11,11,11,10,11,11,10,11,11,11,10,11,11,11,11,11,11,11,11,11,11,10,11,11,10,10,10,10,11,11,11,11,11,11,11,11,11],"n":["Broadcast","JamsocketContext","JamsocketService","JamsocketServiceBuilder","MessageRecipient","Service","SimpleJamsocketService","User","WrappedJamsocketService","binary","binary","binary","borrow","borrow","borrow_mut","borrow_mut","build","clone","clone_into","connect","connect","connect","decode_u32","disconnect","disconnect","disconnect","encode_u32","fmt","from","from","from","initialize","into","into","message","message","message","send_binary","send_message","set_timer","timer","timer","timer","to_owned","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["jamsocket","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","Provides an interface for a JamsocketService instance to …","The main interface to a Jamsocket service.","Enables an object to become a JamsocketService of the …","Represents the recipient(s) of a message.","The type of JamsocketService that the object implementing …","A simplified interface for creating a JamsocketService …","","Combines a SimpleJamsocketService with an owned …","Called each time a client sends a binary message to the …","Called each time a client sends a binary message to the …","","","","","","Transform <code>self</code> into a JamsocketService.","","","Called each time a client connects to the service.","Called each time a client connects to the service.","","","Called each time a client disconnects from the service, …","Called each time a client disconnects from the service, …","","","","","","","Called when the service is created, before any client has …","","","Called each time a client sends a text message to the …","Called each time a client sends a text message to the …","","Sends a binary message to a currently connected user, or …","Sends a message to a currently connected user, or …","Sets a timer to wake up the service in the given number …","Called when JamsocketContext::set_timer has been called …","Called when JamsocketContext::set_timer has been called …","","","","","","","",""],"i":[1,0,0,0,0,2,0,1,0,3,4,5,5,1,5,1,2,1,1,3,4,5,1,3,4,5,1,1,5,1,1,4,5,1,3,4,5,6,6,6,3,4,5,1,5,1,5,1,5,1],"f":[null,null,null,null,null,null,null,null,null,[[["u32",15]]],[[["u32",15]]],[[["u32",15]]],[[]],[[]],[[]],[[]],[[["str",15]]],[[],["messagerecipient",4]],[[]],[[["u32",15]]],[[["u32",15]]],[[["u32",15]]],[[["u32",15]]],[[["u32",15]]],[[["u32",15]]],[[["u32",15]]],[[],["u32",15]],[[["formatter",3]],["result",6]],[[]],[[]],[[["u32",15]]],[[["str",15]]],[[]],[[]],[[["str",15],["u32",15]]],[[["str",15],["u32",15]]],[[["str",15],["u32",15]]],[[]],[[["str",15]]],[[["u32",15]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[4,"MessageRecipient"],[8,"JamsocketServiceBuilder"],[8,"JamsocketService"],[8,"SimpleJamsocketService"],[3,"WrappedJamsocketService"],[8,"JamsocketContext"]]},\
"jamsocket_cli":{"doc":"<code>jamsocket-cli</code>: a command-line interface to Jamsocket","t":[0,5,0,3,13,3,4,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5],"n":["cli_opts","main","serve","Opts","Serve","ServeCommand","SubCommand","Validate","ValidateCommand","augment_clap","augment_clap","augment_clap","augment_clap","augment_subcommands","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","debug","debug","debug","debug","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","drop","drop","drop","drop","from","from","from","from","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_subcommand","heartbeat_interval","heartbeat_timeout","init","init","init","init","into","into","into","into","into_app","into_app","into_app","into_app","module","module","port","rooms","subcommand","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","vzip","vzip","vzip","vzip","serve"],"q":["jamsocket_cli","","","jamsocket_cli::cli_opts","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","jamsocket_cli::serve"],"d":["","","","","Run a dev server to host a given Jamsocket module.","","","Validate a given Jamsocket module.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The time interval (in seconds) between WebSocket …","The duration of time without hearing from a client before …","","","","","","","","","","","","","The module (.wasm file) to serve.","The module (.wasm file) to validate.","The port to serve on.","The strategy for assigning new room IDs.","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,1,0,0,1,0,2,1,3,4,1,2,1,3,4,2,1,3,4,2,1,3,4,2,1,3,4,2,1,3,4,2,1,3,4,2,1,3,4,2,1,3,4,1,3,3,2,1,3,4,2,1,3,4,2,1,3,4,3,4,3,3,2,2,1,3,4,2,1,3,4,2,1,3,4,2,1,3,4,0],"f":[null,[[],["result",6]],null,null,null,null,null,null,null,[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["formatter",3]],[["error",3],["result",4,["error"]]]],[[["formatter",3]],[["error",3],["result",4,["error"]]]],[[["formatter",3]],[["error",3],["result",4,["error"]]]],[[["formatter",3]],[["error",3],["result",4,["error"]]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[]],[[]],[[]],[[]],[[["argmatches",3]]],[[["argmatches",3]]],[[["argmatches",3]]],[[["argmatches",3]]],[[["option",4]],["option",4]],null,null,[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[]],[[]],[[]],[[]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],null,null,null,null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[["servecommand",3]],["result",6]]],"p":[[4,"SubCommand"],[3,"Opts"],[3,"ServeCommand"],[3,"ValidateCommand"]]},\
"jamsocket_server":{"doc":"","t":[3,3,13,13,13,13,13,13,4,3,3,8,4,3,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,5,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,12,12,11,11,11,12,12,12,12,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,12,12],"n":["AssignUserId","ClientSocketConnection","Connect","Disconnect","Explicit","Generator","Implicit","Message","MessageFromClient","MessageFromServer","RoomActor","RoomIdGenerator","RoomIdStrategy","ServerSettings","Singleton","UuidRoomIdGenerator","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","data","do_serve","fmt","fmt","from","from","from","from","from","from","from","from","from_str","generate","generate","handle","handle","handle","handle","handle","heartbeat_interval","heartbeat_interval","heartbeat_timeout","heartbeat_timeout","interval_handle","into","into","into","into","into","into","into","into","ip","last_seen","new","new","new_binary","port","room","room_id","room_id_strategy","started","to_owned","to_owned","to_user","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","user","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","data","from_user"],"q":["jamsocket_server","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","jamsocket_server::MessageFromClient",""],"d":["Represents a request to reserve a user ID and return it. …","Represents a connection from a service to a client, which …","A client opens a connection to the server.","A client disconnects from the server (or their connection …","Rooms are created with an explicit API call that provides …","Room IDs are created by an endpoint, which returns an ID …","Rooms are created when they are first accessed.","A client sends a message.","Represents a message or event initiated by a client.","Represents a message sent to one or more clients from the …","Actor model representation of a “room”. A room is a …","Trait for objects capable of generating a user ID.","Determines how new rooms are assigned an id.","Settings used by the server.","The server only has one room.","Assigns a user ID from a UUID.","","","","","","","","","","","","","","","","","","","","","","Start a server given a cloneable JamsocketServiceBuilder …","","","","","","","","","","","","","","","","","","","","The duration of time between server-initiated WebSocket …","","The minimum amount of time between client heartbeats …","","","","","","","","","","","","","","","The port to run the server on.","","","The method by which new rooms are created and assigned …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,1,2,2,2,1,0,0,0,0,0,0,2,0,3,4,5,6,2,7,1,8,3,4,5,6,2,7,1,8,1,8,1,8,8,0,1,8,3,4,5,6,2,7,1,8,2,9,6,3,3,5,5,5,3,7,3,7,3,3,4,5,6,2,7,1,8,3,3,5,8,8,7,3,3,7,3,1,8,8,3,4,5,6,2,7,1,8,3,4,5,6,2,7,1,8,3,4,5,6,2,7,1,8,3,3,4,5,6,2,7,1,8,10,10],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["messagefromclient",4]],[[],["messagefromserver",3]],[[]],[[]],null,[[["sync",8],["serversettings",3],["serviceactorcontext",3],["clone",8],["send",8],["jamsocketservicebuilder",8,["serviceactorcontext"]]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],["result",4]],[[],["string",3]],[[],["string",3]],[[["messagefromserver",3]]],[[["message",4],["protocolerror",4],["result",4,["message","protocolerror"]]]],[[["context",3],["messagefromserver",3]]],[[["context",3],["messagefromclient",4]]],[[["assignuserid",3],["context",3]],["u32",15]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[["string",3],["recipient",3,["messagefromclient"]],["messagefromclient",4]]],[[["messagerecipient",4],["string",3]]],[[["messagerecipient",4],["u8",15],["vec",3,["u8"]]]],null,null,null,null,[[]],[[]],[[]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null],"p":[[4,"MessageFromClient"],[4,"RoomIdStrategy"],[3,"ClientSocketConnection"],[3,"AssignUserId"],[3,"RoomActor"],[3,"UuidRoomIdGenerator"],[3,"ServerSettings"],[3,"MessageFromServer"],[8,"RoomIdGenerator"],[13,"Message"]]},\
"jamsocket_wasm":{"doc":"","t":[23,0,13,8,8,8,4,16,8,13,3,10,11,11,11,11,11,11,10,11,11,10,11,11,11,10,11,11,11,11,11,11,11,11,11,11,23,10,11,11,10,10,10,10,11,11,11,11,11,11,11,11,11],"n":["jamsocket_wasm","prelude","Broadcast","JamsocketContext","JamsocketService","JamsocketServiceBuilder","MessageRecipient","Service","SimpleJamsocketService","User","WrappedJamsocketService","binary","binary","binary","borrow","borrow","borrow_mut","borrow_mut","build","clone","clone_into","connect","connect","connect","decode_u32","disconnect","disconnect","disconnect","encode_u32","fmt","from","from","from","initialize","into","into","jamsocket_wasm","message","message","message","send_binary","send_message","set_timer","timer","timer","timer","to_owned","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["jamsocket_wasm","","jamsocket_wasm::prelude","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Exposes a <code>jamsocket_wasm::SimpleJamsocketService</code>…","","","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","The type of JamsocketService that the object implementing …","Re-exports useful items from <code>jamsocket</code> and …","","Re-exports useful items from <code>jamsocket</code> and …","Called each time a client sends a binary message to the …","Called each time a client sends a binary message to the …","","","","","","Transform <code>self</code> into a JamsocketService.","","","Called each time a client connects to the service.","Called each time a client connects to the service.","","","Called each time a client disconnects from the service, …","Called each time a client disconnects from the service, …","","","","","","","Called when the service is created, before any client has …","","","Exposes a <code>jamsocket_wasm::SimpleJamsocketService</code>…","Called each time a client sends a text message to the …","Called each time a client sends a text message to the …","","Sends a binary message to a currently connected user, or …","Sends a message to a currently connected user, or …","Sets a timer to wake up the service in the given number …","Called when JamsocketContext::set_timer has been called …","Called when JamsocketContext::set_timer has been called …","","","","","","","",""],"i":[0,0,1,0,0,0,0,2,0,1,0,3,4,5,5,1,5,1,2,1,1,3,4,5,1,3,4,5,1,1,5,1,1,4,5,1,0,3,4,5,6,6,6,3,4,5,1,5,1,5,1,5,1],"f":[null,null,null,null,null,null,null,null,null,null,null,[[["u32",15]]],[[["u32",15]]],[[["u32",15]]],[[]],[[]],[[]],[[]],[[["str",15]]],[[],["messagerecipient",4]],[[]],[[["u32",15]]],[[["u32",15]]],[[["u32",15]]],[[["u32",15]],["messagerecipient",4]],[[["u32",15]]],[[["u32",15]]],[[["u32",15]]],[[],["u32",15]],[[["formatter",3]],[["error",3],["result",4,["error"]]]],[[]],[[]],[[["u32",15]],["messagerecipient",4]],[[["str",15]]],[[]],[[]],null,[[["str",15],["u32",15]]],[[["str",15],["u32",15]]],[[["str",15],["u32",15]]],[[]],[[["str",15]]],[[["u32",15]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[4,"MessageRecipient"],[8,"JamsocketServiceBuilder"],[8,"JamsocketService"],[8,"SimpleJamsocketService"],[3,"WrappedJamsocketService"],[8,"JamsocketContext"]]},\
"jamsocket_wasm_host":{"doc":"This module provides a jamsocket::JamsocketService …","t":[13,13,13,13,3,3,4,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["CouldNotImportGlobal","CouldNotImportMemory","InvalidApiVersion","InvalidProtocolVersion","WasmHost","WasmHostFactory","WasmRuntimeError","binary","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","build","clone","clone_into","connect","debug","debug","debug","deref","deref","deref","deref_mut","deref_mut","deref_mut","description","disconnect","drop","drop","drop","fmt","fmt","from","from","from","init","init","init","into","into","into","message","new","new","timer","to_owned","to_string","try_binary","try_from","try_from","try_from","try_into","try_into","try_into","try_message","type_id","type_id","type_id","vzip","vzip","vzip"],"q":["jamsocket_wasm_host","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","Hosts a jamsocket::JamsocketService implemented by a …","Loads and caches a WebAssembly module such that a WasmHost…","An error encountered while running WebAssembly.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,1,1,1,0,0,0,2,2,3,1,2,3,1,3,3,3,2,2,3,1,2,3,1,2,3,1,1,2,2,3,1,1,1,2,3,1,2,3,1,2,3,1,2,2,3,2,3,1,2,2,3,1,2,3,1,2,2,3,1,2,3,1],"f":[null,null,null,null,null,null,null,[[["u32",15]]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]]],[[],["wasmhostfactory",3]],[[]],[[["u32",15]]],[[["formatter",3]],[["error",3],["result",4,["error"]]]],[[["formatter",3]],[["error",3],["result",4,["error"]]]],[[["formatter",3]],[["error",3],["result",4,["error"]]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[],["str",15]],[[["u32",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[]],[[]],[[]],[[["str",15],["u32",15]]],[[["arc",3],["engine",3],["str",15],["module",3]],["result",6]],[[["str",15]]],[[]],[[]],[[],["string",3]],[[["u32",15]],["result",6]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["str",15],["u32",15]],["result",6]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]]],"p":[[4,"WasmRuntimeError"],[3,"WasmHost"],[3,"WasmHostFactory"]]},\
"jamsocket_wasm_macro":{"doc":"","t":[23],"n":["jamsocket_wasm"],"q":["jamsocket_wasm_macro"],"d":["Exposes a <code>jamsocket_wasm::SimpleJamsocketService</code>…"],"i":[0],"f":[null],"p":[]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};