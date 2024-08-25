#!/bin/sh
stty -F /dev/ttyACM0 115200 -crtscts && timeout 10 bash -c '(while :; do echo -n -e "\x42"; done)' > /dev/ttyACM0
