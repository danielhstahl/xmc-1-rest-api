## Curl commands

`curl http://127.0.0.1:8000/info -H 'Content-Type: application/json'`

`curl -X POST http://127.0.0.1:8000/power/on -H 'Content-Type: application/json'`

`curl -X POST http://127.0.0.1:8000/volume/up -H 'Content-Type: application/json'`

`curl -X POST http://127.0.0.1:8000/volume/-55 -H 'Content-Type: application/json'`


`curl -X POST http://127.0.0.1:8000/hdmi/3 -H 'Content-Type: application/json'`

`curl -X POST http://127.0.0.1:8000/mode/stereo -H 'Content-Type: application/json'`

`curl -X POST http://127.0.0.1:8000/power/off -H 'Content-Type: application/json'`

## Get on RPI

`curl -L https://github.com/danielhstahl/xmc-1-rest-api/releases/download/v0.6.0/xmc1_rest_api > xmc1_rest_api`
`chmod +x ./xmc1_rest_api`
`curl -L https://github.com/danielhstahl/xmc-1-rest-api/releases/download/v0.6.0/build.zip > build.zip`
`mkdir xmc-remote`
`unzip build.zip -d xmc-remote/`

## Create service

`sudo cp /home/pi/scripts/xmc1RestApi.service /lib/systemd/system/`