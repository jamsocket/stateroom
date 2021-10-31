var searchIndex = JSON.parse('{\
"jamsocket":{"doc":"Jamsocket is a minimalist framework for developing …","t":[13,13,13,3,13,13,16,8,8,8,13,13,4,4,4,4,16,8,3,13,13,3,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,10,11,11,10,10,11,11,11,11,11,10,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12],"n":["Broadcast","Bytes","Client","ClientId","Connect","Disconnect","Error","JamsocketContext","JamsocketService","JamsocketServiceFactory","Message","Message","MessageFromProcess","MessagePayload","MessageRecipient","MessageToProcess","Service","SimpleJamsocketService","SimpleJamsocketServiceFactory","Text","Timer","WrappedJamsocketService","binary","binary","binary","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","build","build","clone","clone","clone_into","clone_into","connect","connect","connect","decode_u32","default","deserialize","deserialize","deserialize","deserialize","deserialize","disconnect","disconnect","disconnect","encode_u32","eq","eq","fmt","fmt","from","from","from","from","from","from","from","from","from","hash","into","into","into","into","into","into","into","message","message","message","ne","ne","new","new","partial_cmp","send_binary","send_message","serialize","serialize","serialize","serialize","serialize","set_timer","timer","timer","timer","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","message","recipient","0","0","0","client","client","client","message"],"q":["jamsocket","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","jamsocket::MessageFromProcess","","jamsocket::MessagePayload","","jamsocket::MessageRecipient","jamsocket::MessageToProcess","","",""],"d":["","","","","","","","Provides an interface for a JamsocketService instance to …","The host interface to a Jamsocket service. Implementations …","Enables an object to become a JamsocketService of the …","","","","","Represents the recipient(s) of a message.","","The type of JamsocketService that the object implementing …","A simplified interface for creating a JamsocketService …","A JamsocketServiceFactory that passes through <code>build()</code> …","","","Combines a SimpleJamsocketService with an owned …","Called each time a client sends a binary message to the …","Called each time a client sends a binary message to the …","","","","","","","","","","","","","","","","Non-destructively build a JamsocketService from <code>self</code>.","","","","","","Called each time a client connects to the service.","Called each time a client connects to the service.","","","","","","","","","Called each time a client disconnects from the service, …","Called each time a client disconnects from the service, …","","","","","","","","","","","","","","","","","","","","","","","","Called each time a client sends a text message to the …","Called each time a client sends a text message to the …","","","","Called when the service is created, before any client has …","","","Sends a binary message to a currently connected user, or …","Sends a message to a currently connected user, or …","","","","","","Sets a timer to wake up the service in the given number of …","Called when JamsocketContext::set_timer has been called on …","Called when JamsocketContext::set_timer has been called on …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,2,1,0,3,3,4,0,0,0,3,5,0,0,0,0,4,0,0,2,3,0,6,7,8,9,8,10,1,2,3,5,9,8,10,1,2,3,5,4,9,10,1,10,1,6,7,8,1,9,10,1,2,3,5,6,7,8,1,10,1,10,1,9,8,10,10,1,1,2,3,5,10,9,8,10,1,2,3,5,6,7,8,10,1,7,8,10,11,11,10,1,2,3,5,11,6,7,8,10,1,9,8,10,1,2,3,5,9,8,10,1,2,3,5,9,8,10,1,2,3,5,12,12,13,14,15,16,17,18,18],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["clientid",3]]],[[["clientid",3]]],[[["clientid",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],["result",4]],[[["str",15]],["result",4,[["infallible",4]]]],[[],["clientid",3]],[[],["messagerecipient",4]],[[]],[[]],[[["clientid",3]]],[[["clientid",3]]],[[["clientid",3]]],[[["u32",15]]],[[]],[[["",26],["",26]],["result",4,[["",26]]]],[[["",26],["",26]],["result",4,[["",26]]]],[[["",26],["",26]],["result",4,[["",26]]]],[[["",26],["",26]],["result",4,[["",26]]]],[[["",26],["",26]],["result",4,[["",26]]]],[[["clientid",3]]],[[["clientid",3]]],[[["clientid",3]]],[[],["u32",15]],[[["clientid",3]],["bool",15]],[[["messagerecipient",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["u32",15]]],[[]],[[]],[[["clientid",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["clientid",3],["str",15]]],[[["clientid",3],["str",15]]],[[["clientid",3],["str",15]]],[[["clientid",3]],["bool",15]],[[["messagerecipient",4]],["bool",15]],[[["str",15]]],[[]],[[["clientid",3]],["option",4,[["ordering",4]]]],[[]],[[["str",15]]],[[["",26],["",26]],["result",4]],[[["",26],["",26]],["result",4]],[[["",26],["",26]],["result",4]],[[["",26],["",26]],["result",4]],[[["",26],["",26]],["result",4]],[[["u32",15]]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null],"p":[[4,"MessageRecipient"],[4,"MessagePayload"],[4,"MessageToProcess"],[8,"JamsocketServiceFactory"],[4,"MessageFromProcess"],[8,"JamsocketService"],[8,"SimpleJamsocketService"],[3,"WrappedJamsocketService"],[3,"SimpleJamsocketServiceFactory"],[3,"ClientId"],[8,"JamsocketContext"],[13,"Message"],[13,"Bytes"],[13,"Text"],[13,"Client"],[13,"Connect"],[13,"Disconnect"],[13,"Message"]]},\
"jamsocket_api":{"doc":"","t":[17,3,3,3,3,12,11,11,11,11,11,11,11,11,11,12,11,11,11,12,11,11,11,11,11,11,11,11,12,11,11,12,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12],"n":["API_BASE","AuthcheckResponse","CreateServiceResponse","JamsocketApi","UploadServiceResponse","activated","authenticate","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","create_room_url","deserialize","deserialize","deserialize","email","from","from","from","from","into","into","into","into","module","new","new_service","provider","serialize","serialize","serialize","service","service_id","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","upload","url_base","username"],"q":["jamsocket_api","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,1,2,2,3,4,1,2,3,4,1,3,3,4,1,1,2,3,4,1,2,3,4,1,3,2,2,1,3,4,1,3,4,2,3,4,1,2,3,4,1,2,3,4,1,2,3,1],"f":[null,null,null,null,null,null,[[],["result",6,[["option",4,[["authcheckresponse",3]]]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[["",26],["",26]],["result",4,[["",26]]]],[[["",26],["",26]],["result",4,[["",26]]]],[[["",26],["",26]],["result",4,[["",26]]]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[["str",15]]],[[],["result",6,[["string",3]]]],null,[[["",26],["",26]],["result",4]],[[["",26],["",26]],["result",4]],[[["",26],["",26]],["result",4]],null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[["str",15]],["result",6,[["uploadserviceresponse",3]]]],null,null],"p":[[3,"AuthcheckResponse"],[3,"JamsocketApi"],[3,"UploadServiceResponse"],[3,"CreateServiceResponse"]]},\
"jamsocket_server":{"doc":"","t":[3,3,13,13,13,4,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,12,12,12,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12],"n":["AssignClientId","ClientSocketConnection","Connect","Disconnect","Message","MessageFromClient","MessageFromServer","RoomActor","Server","ServiceActor","ServiceActorContext","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","client_id","clone","clone","clone","clone_into","clone_into","clone_into","data","default","fmt","fmt","from","from","from","from","from","from","from","from","handle","handle","handle","handle","handle","handle","heartbeat_interval","heartbeat_interval","heartbeat_timeout","heartbeat_timeout","interval_handle","into","into","into","into","into","into","into","into","ip","last_seen","new","new","new","new","new_binary","port","room","room_id","send_binary","send_message","serve","set_timer","started","stopping","to_client","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","with_heartbeat_interval","with_heartbeat_timeout","with_port","0","0","1","data","from_client"],"q":["jamsocket_server","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","jamsocket_server::MessageFromClient","","","",""],"d":["Represents a request to reserve a client ID and return it. …","Represents a connection from a service to a client, which …","A client opens a connection to the server.","A client disconnects from the server (or their connection …","A client sends a message.","Represents a message or event initiated by a client.","Represents a message sent to one or more clients from the …","Actor model representation of a “room”. A room is a …","Settings used by the server.","","A JamsocketContext implementation for JamsocketServices …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The duration of time between server-initiated WebSocket …","","The minimum amount of time between client heartbeats …","","","","","","","","","","","","","","","","","The port to run the server on. Defaults to 8080.","","","","","Start a server given a [JamsocketService].","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,1,1,0,0,0,0,0,0,2,3,4,5,6,1,7,8,2,3,4,5,6,1,7,8,2,1,7,8,1,7,8,7,6,1,7,2,3,4,5,6,1,7,8,2,2,4,4,4,5,2,6,2,6,2,2,3,4,5,6,1,7,8,2,2,4,5,6,7,7,6,2,2,8,8,6,8,2,5,7,1,7,8,2,3,4,5,6,1,7,8,2,3,4,5,6,1,7,8,2,3,4,5,6,1,7,8,2,3,4,5,6,1,7,8,6,6,6,9,10,9,11,11],"f":[null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[],["messagefromclient",4]],[[],["messagefromserver",3]],[[],["serviceactorcontext",3]],[[]],[[]],[[]],null,[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["result",4,[["message",4],["protocolerror",4]]]]],[[["messagefromserver",3]]],[[["messagefromserver",3],["context",3]]],[[["messagefromclient",4],["context",3]]],[[["assignclientid",3],["context",3]],["clientid",3]],[[["messagefromclient",4]]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[["recipient",3,[["messagefromclient",4]]]]],[[["context",3],["recipient",3,[["messagefromserver",3]]]],["option",4]],[[]],[[["messagerecipient",4],["string",3]]],[[["messagerecipient",4],["vec",3,[["u8",15]]]]],null,null,null,[[]],[[["str",15]]],[[],["result",6]],[[["u32",15]]],[[]],[[],["running",4]],null,[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["u64",15]]],[[["u64",15]]],[[["u32",15]]],null,null,null,null,null],"p":[[4,"MessageFromClient"],[3,"ClientSocketConnection"],[3,"AssignClientId"],[3,"RoomActor"],[3,"ServiceActor"],[3,"Server"],[3,"MessageFromServer"],[3,"ServiceActorContext"],[13,"Connect"],[13,"Disconnect"],[13,"Message"]]},\
"jamsocket_stdio":{"doc":"","t":[3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["StdioProcessService","StdioProcessServiceFactory","binary","borrow","borrow","borrow_mut","borrow_mut","build","connect","disconnect","from","from","into","into","message","new","timer","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["jamsocket_stdio","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,2,1,2,1,2,1,1,2,1,2,1,1,2,1,2,1,2,1,2,1],"f":[null,null,[[["clientid",3]]],[[]],[[]],[[]],[[]],[[["str",15]],["result",4]],[[["clientid",3]]],[[["clientid",3]]],[[]],[[]],[[]],[[]],[[["clientid",3],["str",15]]],[[["str",15]]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[3,"StdioProcessService"],[3,"StdioProcessServiceFactory"]]},\
"jamsocket_wasm":{"doc":"","t":[23,0,13,13,3,16,8,8,8,4,16,8,3,10,11,11,11,11,11,11,11,11,10,11,11,11,11,10,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,23,10,11,11,11,11,10,11,11,10,10,11,11,10,10,11,11,11,11,11,11,11,11,11,11,11,11,11,12],"n":["jamsocket_wasm","prelude","Broadcast","Client","ClientId","Error","JamsocketContext","JamsocketService","JamsocketServiceFactory","MessageRecipient","Service","SimpleJamsocketService","WrappedJamsocketService","binary","binary","binary","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","build","clone","clone","clone_into","clone_into","connect","connect","connect","decode_u32","deserialize","deserialize","disconnect","disconnect","disconnect","encode_u32","eq","eq","fmt","fmt","from","from","from","from","from","hash","into","into","into","jamsocket_wasm","message","message","message","ne","ne","new","new","partial_cmp","send_binary","send_message","serialize","serialize","set_timer","timer","timer","timer","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","0"],"q":["jamsocket_wasm","","jamsocket_wasm::prelude","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","jamsocket_wasm::prelude::MessageRecipient"],"d":["Exposes a <code>jamsocket_wasm::SimpleJamsocketService</code>…","","","","Re-exports useful items from <code>jamsocket</code> and …","","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","The type of JamsocketService that the object implementing …","Re-exports useful items from <code>jamsocket</code> and …","Re-exports useful items from <code>jamsocket</code> and …","Called each time a client sends a binary message to the …","Called each time a client sends a binary message to the …","","","","","","","","Non-destructively build a JamsocketService from <code>self</code>.","","","","","Called each time a client connects to the service.","Called each time a client connects to the service.","","","","","Called each time a client disconnects from the service, …","Called each time a client disconnects from the service, …","","","","","","","","","","","","","","","","Exposes a <code>jamsocket_wasm::SimpleJamsocketService</code>…","Called each time a client sends a text message to the …","Called each time a client sends a text message to the …","","","","Called when the service is created, before any client has …","","","Sends a binary message to a currently connected user, or …","Sends a message to a currently connected user, or …","","","Sets a timer to wake up the service in the given number of …","Called when JamsocketContext::set_timer has been called on …","Called when JamsocketContext::set_timer has been called on …","","","","","","","","","","","","",""],"i":[0,0,1,1,0,2,0,0,0,0,2,0,0,3,4,5,5,6,1,5,6,1,2,6,1,6,1,3,4,5,1,6,1,3,4,5,1,6,1,6,1,5,6,6,1,1,6,5,6,1,0,3,4,5,6,1,4,5,6,7,7,6,1,7,3,4,5,6,1,5,6,1,5,6,1,5,6,1,8],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,[[["clientid",3]]],[[["clientid",3]]],[[["clientid",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],["result",4]],[[],["clientid",3]],[[],["messagerecipient",4]],[[]],[[]],[[["clientid",3]]],[[["clientid",3]]],[[["clientid",3]]],[[["u32",15]],["messagerecipient",4]],[[["",26],["",26]],["result",4,[["clientid",3]]]],[[["",26],["",26]],["result",4,[["messagerecipient",4]]]],[[["clientid",3]]],[[["clientid",3]]],[[["clientid",3]]],[[],["u32",15]],[[["clientid",3]],["bool",15]],[[["messagerecipient",4]],["bool",15]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[]],[[["u32",15]],["clientid",3]],[[]],[[["clientid",3]],["messagerecipient",4]],[[]],[[]],[[]],[[]],[[]],null,[[["clientid",3],["str",15]]],[[["clientid",3],["str",15]]],[[["clientid",3],["str",15]]],[[["clientid",3]],["bool",15]],[[["messagerecipient",4]],["bool",15]],[[["str",15]]],[[],["wrappedjamsocketservice",3]],[[["clientid",3]],["option",4,[["ordering",4]]]],[[]],[[["str",15]]],[[["",26],["",26]],["result",4]],[[["",26],["",26]],["result",4]],[[["u32",15]]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null],"p":[[4,"MessageRecipient"],[8,"JamsocketServiceFactory"],[8,"JamsocketService"],[8,"SimpleJamsocketService"],[3,"WrappedJamsocketService"],[3,"ClientId"],[8,"JamsocketContext"],[13,"Client"]]},\
"jamsocket_wasm_host":{"doc":"This module provides a jamsocket::JamsocketService …","t":[13,13,13,13,3,3,4,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["CouldNotImportGlobal","CouldNotImportMemory","InvalidApiVersion","InvalidProtocolVersion","WasmHost","WasmHostFactory","WasmRuntimeError","binary","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","build","clone","clone_into","connect","debug","debug","debug","deref","deref","deref","deref_mut","deref_mut","deref_mut","description","disconnect","drop","drop","drop","fmt","fmt","from","from","from","get_fd_flags","get_fd_flags","get_fd_flags","init","init","init","into","into","into","message","new","new","new_set_fd_flags","new_set_fd_flags","new_set_fd_flags","new_with_shared_module","set_fd_flags","set_fd_flags","set_fd_flags","timer","to_owned","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip"],"q":["jamsocket_wasm_host","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","Hosts a jamsocket::JamsocketService implemented by a …","Loads and caches a WebAssembly module such that a WasmHost …","An error encountered while running WebAssembly.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,1,1,1,0,0,0,2,2,3,1,2,3,1,3,3,3,2,2,3,1,2,3,1,2,3,1,1,2,2,3,1,1,1,2,3,1,2,3,1,2,3,1,2,3,1,2,2,3,2,3,1,3,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1],"f":[null,null,null,null,null,null,null,[[["clientid",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],["result",6]],[[],["wasmhostfactory",3]],[[]],[[["clientid",3]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["formatter",3]],["result",4,[["error",3]]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[],["str",15]],[[["clientid",3]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["result",4,[["fdflags",3],["error",3]]]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[]],[[]],[[]],[[["clientid",3],["str",15]]],[[["str",15],["module",3],["engine",3],["arc",3]],["result",6]],[[["",26],["",26]],["result",6,[["",26]]]],[[["fdflags",3]],["result",4,[["setfdflags",3,[["",26]]],["error",3]]]],[[["fdflags",3]],["result",4,[["setfdflags",3,[["",26]]],["error",3]]]],[[["fdflags",3]],["result",4,[["setfdflags",3,[["",26]]],["error",3]]]],[[["arc",3,[["engine",3]]],["arc",3,[["module",3]]]]],[[["setfdflags",3,[["",26]]]],["result",4,[["error",3]]]],[[["setfdflags",3,[["",26]]]],["result",4,[["error",3]]]],[[["setfdflags",3,[["",26]]]],["result",4,[["error",3]]]],[[]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]]],"p":[[4,"WasmRuntimeError"],[3,"WasmHost"],[3,"WasmHostFactory"]]},\
"jamsocket_wasm_macro":{"doc":"","t":[23],"n":["jamsocket_wasm"],"q":["jamsocket_wasm_macro"],"d":["Exposes a <code>jamsocket_wasm::SimpleJamsocketService</code>…"],"i":[0],"f":[null],"p":[]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};