#!/bin/bash

if [ "$RMODE" == "size" ]; then
  avr-size -C --mcu=atmega328p $@
elif [ "$RMODE" == "sim" ]; then
  simavr -m atmega328p $@
else
  ravedude "$@"
fi
