docker build .
docker tag todo_web:latest raspberrypi.local:5000/todo_web:latest
docker push raspberrypi.local:5000/todo_web:latest

ssh -Y pi@raspberrypi.local 'bash -s' < ./scripts/reboot-pi-services.sh