#!/usr/bin/env python3

import json

def main():
    while True:
        line = input()
        message = json.loads(line)

        client = message["client"]
        response_message = ""
        if message["type"] == "Connect":
            response_message = f"Client {client} connected."
        elif message["type"] == "Disconnect":
            response_message = f"Client {client} disconnected."
        elif message["type"] == "Message":
            client_message = message["message"]["Text"]
            response_message = f"Client {client} sent `{client_message}`."

        response = {
            "type": "Message",
            "recipient": "Broadcast",
            "message": {
                "Text": response_message
            }
        }
        print(json.dumps(response), flush=True)

if __name__ == '__main__':
    main()
