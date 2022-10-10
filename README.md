## Curl commands

`curl http://127.0.0.1:8000/info -H 'Content-Type: application/json'`

`curl -X POST http://127.0.0.1:8000/power/on -H 'Content-Type: application/json'`

`curl -X POST http://127.0.0.1:8000/volume/up -H 'Content-Type: application/json'`

`curl -X POST http://127.0.0.1:8000/volume/-55 -H 'Content-Type: application/json'`


`curl -X POST http://127.0.0.1:8000/hdmi/3 -H 'Content-Type: application/json'`

`curl -X POST http://127.0.0.1:8000/mode/stereo -H 'Content-Type: application/json'`

`curl -X POST http://127.0.0.1:8000/power/off -H 'Content-Type: application/json'`