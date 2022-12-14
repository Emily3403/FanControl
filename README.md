# FanControl

This Program provides a way to control your fans while also controlling the "Governor policy" as well as "Energy Preference" of your linux system. Three modes of operating are currently supported.

This Program should transparently switch modes to the most appropriate at this moment. You will be able to disable this feature if you don't like it as it is tailored to my personal workflow

Currently, the only supported Laptop is the Framework Laptop.

## Sustained Load

In this scenario the CPU is at 100% load continuously. 

This scenario will most likely be encountered when compiling huge programs like e.g. the Linux Kernel.

### CPU Frequency

The CPU Frequency should be as high as possible.

If possible, the CPU should be forbid to throttle. This means that the compilation will be done as quickly as possible.

### Fan

The Fan should run at 100% all the time. As a _lot_ of heat will be generated, it has to spin up to the max.


## On-demand / Balanced

In this scenario you are working on stuff with high, bursty, demands. This could be an IDE that has to re-index quite often.

### CPU Frequency

Here the Frequency does not have to turbo all the time - instead it should turbo up when necessary. Thus, a fast speedup is essential to use this mode. It is also crucial that if the CPU is at 0% load, some form of battery saving is done: I want as much as possible battery life.

### Fan

The Fan should be somewhat audible if some load is present. It should speed up at ~60°C and reach peak performance at ~80°C.

## Powersave

This mode is for power saving while still being able to do some sort of lightweight work: Using the Browser / Terminal should be possible and not noticeably different from the On-demand mode. 

In essence, it should create the illusion of the On demand mode and provide much more battery life than it.

### CPU Frequency

As low as possible.

### Fan

As the CPU should not turbo up the fan should not be audible.

