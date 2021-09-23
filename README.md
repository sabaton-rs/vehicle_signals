# Vehicle Signal Specification for Cyclonedds-rs

This crate generates the DDS Topic types for use in an automotive platform. The types are
based on the GENIVI VSS Specification. The aim of the Vehicle Signal Specification (VSS) is to create a common understanding of vehicle signals independent of the protocol or serialization format.

This crate depends on cyclonedds-rs, the safe Rust binding for cyclonedds.

## Version

The current Version used is <https://github.com/GENIVI/vehicle_signal_specification/commit/e851f5aa9a31e0ec836061bb263a215eae46a24d>
Version 2 of the VSS is not yet released, so please expect changes.

## Usage of DDS Keys

This implementation uses keys in the generated types. This simplifies the type path considerably.  For example, in the GENIVI VSS, you will find the following type names

1. Vehicle.Cabin.Door.Row1.Left.Window.Position
2. Vehicle.Cabin.Door.Row2.Left.Window.Position
3. Vehicle.Cabin.Door.Row1.Right.Window.Position
4. Vehicle.Cabin.Door.Row2.Right.Window.Position

In the generated binding, you will find just one type
1. Vehicle.Cabin.Door.Window.Position

The Row and the Side of the window is converted into values within the Position structure.[vehicle_signals::vehicle::cabin::door::window::Position] The row and the side are marked as topic keys.

## Build Instructions (If you want to move to a newer version of the VSS)

The signals are generated from the CSV output of the GENIVI vehicle signal specification. Copy the generated CSV file into this repo and update the build.rs to reflect the correct file.

Vehicle signal specification : https://github.com/GENIVI/vehicle_signal_specification/

## Examples

1. Vehicle Speed Publisher https://github.com/sjames/demo-vehicle-speed-publisher.git
2. Vehicle Speed Subscriber (async support) https://github.com/sjames/demo-vehicle-speed-subscriber.git




