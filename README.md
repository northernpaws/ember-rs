# ember-rs

`ember-rs` is a Rust crate for interfacing with Silicon Labs EM35x ("Ember") Zigbee radios using the [Rev. 5.2 EZSP (EmberZNet Serial Protocol) interface](https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf). This crate implements all the types and as many of the API interfaces as possible from the [UG100: EZSP Reference Guide (Rev. 5.2)](https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf) application node to allow Rust-based applications to communicate over serial.

All types translated from the [UG100: EZSP Reference Guide (Rev. 5.2)](https://www.silabs.com/documents/public/user-guides/ug100-ezsp-reference-guide.pdf) contain the original documentation comments descripting their functionality, and an additional comment that includes their equivalent name from the reference spec and standard C library to aid in code translation.

## Notes

 * All EZSP data is transmitted and received in little endian