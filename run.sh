#!/bin/bash

if [ "$RMODE" == "size" ]; then
  avr-size -C --mcu=atmega328p $@
else
  ravedude "$@"
fi
