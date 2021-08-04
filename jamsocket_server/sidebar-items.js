initSidebarItems({"enum":[["MessageFromClient","Represents a message or event initiated by a client."],["RoomIdStrategy","Determines how new rooms are assigned an id."],["ServiceShutdownPolicy",""]],"struct":[["AssignUserId","Represents a request to reserve a user ID and return it. User IDs are unique only in the context of a room."],["ClientSocketConnection","Represents a connection from a service to a client, which consists of a message receiver and a user ID."],["MessageFromServer","Represents a message sent to one or more clients from the server."],["RoomActor","Actor model representation of a “room”. A room is a set of clients that share an instance of a Jamsocket instance. Conceptually, this is like a room in a chat service. Events (such as messages) and their side-effects are isolated to the room in which they occur."],["Server","Settings used by the server."],["ServiceActorContext",""],["ShortRoomIdGenerator",""],["ShortRoomIdGeneratorFactory",""],["UuidRoomIdGenerator","Assigns a user ID from a UUID."],["UuidRoomIdGeneratorFactory",""]],"trait":[["RoomIdGenerator","Trait for objects capable of generating a user ID."]]});