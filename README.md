# S.O.A.R.E.D.S
Surge's Operations Assistant and Research, Entertainment, and Development System

Or REN for short ;)

## What is it

SOAREDS is a custom IoT Smart System that will be designed to be capable of smart
home and other IoT enabled structures. The system is written in RUST, with extra
functionality to host a small local webserver for easy configuring of the network
structure. The network structure consists of a root device and peripherals. See 
below for a list of peripherals.

## Network Structure

The root device will host its own wifi network to allow for easy connection
of peripherals. This network has two different areas, one that acts as a waiting
room for connected devices, and one that acts as the actual data layer for devices
to communicate on. The reason for two areas is to add an extra layer of security.
To add devices to the SOAREDS network, they will need to manually be added through
the web console. Documentation coming soon.

## Peripherals

The peripherals act as the main way to interact with the SOAREDS ecosystem. All
peripherals can be used in multiple ways. The peripherals will be designed to be 
modular, meaning if you want to connect multiple I/O devices to a single peripheral,
the device will be more than capable. Each peripheral will be highly customizable
through the web console. 

## Mobile Applications

In addition to the peripherals, the SOAREDS root device can be registered to function
alongside a mobile app that you will be able to download on your phone or tablet.
Need to set a reminder when you get home? Need to check if your doors are locked amd
lights are off if you aren't home? The mobile app will be able to handle any
interactions with your local system if you are away.

## Voice activation and Natural Language Processing

SOAREDS will be able to be controlled by your voice. This will be an optional feature
who dislike the use of it, but can be useful. It will listen for a name that you give
the root device when you configure NLP functionality. It will also be able to turn off
specific zones to avoid unwanted use (like after bedtime), as well as disable entirely 
with a touch of a button.