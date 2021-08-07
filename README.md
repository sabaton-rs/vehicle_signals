# Vehicle Signal Specification for Cyclonedds-rs

This crate generates the DDS Topic types for use in an automotive platform. The types are
based on the GENIVI VSS Specification. The aim of the Vehicle Signal Specification (VSS) is to create a common understanding of vehicle signals independent of the protocol or serialization format.

## Version

The current Version used is <https://github.com/GENIVI/vehicle_signal_specification/commit/e851f5aa9a31e0ec836061bb263a215eae46a24d>

## Usage of DDS Keys

This implementation uses keys in the generated types. This simplifies the type path considerably.  For example, in the GENIVI VSS, you will find the following type names

1. Vehicle.Cabin.Door.Row1.Left.Window.Position
2. Vehicle.Cabin.Door.Row2.Left.Window.Position
3. Vehicle.Cabin.Door.Row1.Right.Window.Position
4. Vehicle.Cabin.Door.Row2.Right.Window.Position

In this implementation, you will find just one type
1. Vehicle.Cabin.Door.Window.Position

The Row and the Side of the window is converted into values within the Position structure.[vehicle_signals::vehicle::cabin::door::window::Position] The row and the side are marked as topic keys.

## Build Instructions

The signals are generated from the CSV output of the GENIVI vehicle signal specification. Copy the generated CSV file into this repo and update the build.rs to reflect the correct file.

Vehicle signal specification : https://github.com/GENIVI/vehicle_signal_specification/




