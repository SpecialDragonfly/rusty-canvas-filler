Then, you can register/unregister a client:

curl -X POST 'http://localhost:8000/register' -H 'Content-Type: application/json' -d '{ "user_id": 1 }' 

curl -X DELETE 'http://localhost:8000/register/e2fa90682255472b9221709566dbceba' 

Or connect to the WebSocket using the returned URL: ws://127.0.0.1:8000/ws/625ac78b88e047a1bc7b3f8459702078.

Then, you can publish messages using

curl -X POST 'http://localhost:8000/publish' \
    -H 'Content-Type: application/json' \
    -d '{"user_id": 1, "topic": "cats", "message": "are awesome"}'