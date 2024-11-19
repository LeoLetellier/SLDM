# QuickStart Guide

This short guide presents the basic features of this software and related information.

## Introduction

This software features a simple model of ground displacement for slow moving landslide. It needs only a few geometrical parameters.

The principle is to assume that the displacement of the moving material of a landslide will follow the slope of the failing surface. Then this displacement is assumed to be conserved for a section perpendicular to the slope, and thus can be migrated onto the surface. 

This methodology gives only a profile of displacement without its magnitude. Additionnal data like InSAR measurements can be used to determine the most probable magnitude of the profile.

To use this workflow, the needed data include for the 2D section :

* Digital Elevation Model (DEM)
* Surfaces
* External displacement data

These data need to be regularly sampled onto the same sampling set of points defined from the DEM. InSAR data can be sparsely defined, but still need to be included inside the range of the DEM sampling.

The output model is one way to define and understand the behaviour of landslide's displacement, but other model or mechanics could achieve similar results.

## Example Project

To test the software, an example project is available at this [repository](https://github.com/LeoLetellier/SLDM-examples). 

It can be loaded directly by opening the ``project.toml`` file.

A common project workflow using this software could be:

1. Loading a DEM file.

2. Loading or creating one or multiple surfaces.

3. Creating a model of interest.

4. Creating a SAR geometry and defining the geometry of the DEM. Loading SAR data.

5. Calibrated the model using the SAR data.

## Interface Details

This section aims to quickly describe how the different parts of the software should be used.

### Menus

The menu bar located at the top of the application provide access to multiples commands used to interact with the project. These commands will often open a command panel located at the left of the application.

1. The ``File`` menu permits to define the current project by loaded / saving to a file, defining the name of the project and a note, and defining the DEM of the section and its geometry.

2. The second menu ``Surface`` is used to load or generate 2D surfaces used as failure surfaces.

3. The ``Model`` menu is used to create a model, which is a combination of possibly multiple surfaces involved in the displacement, and calibrate it using previsouly defined InSAR data.

4. The ``Satellite`` menu permits to define SAR geometry and load displacement data profiles.

The side panel gives access to the explorer, the command panel as well as this documentation page. The graph area can be maximized by clicking on the current panel icon.

### Explorer

The explorer presents a view of all the created objects (DEM, surfaces, models, sar data). It can also be used the choose what element to display in the viewer.

### Viewer

The viewer presents a 2D section representing the DEM, the defined surfaces and the vectors of the associated displacement.

The scale of the vectors can be manually change from the explorer.

The color of each element can be change using the palette button in the explorer, and interacting with the color picker using the **right click** control.

The second viewer named properties can be access using the command `Model > Analyse & Export` to compare displacement value between a model and data.

The graph will by default scale to the data. This means that the scale is not orthonormal and that the angles will not be preserved by the display. To prevent this, a new click on the section icon will change the scale to a orthonormal scale. Double click anywhere on the graph to get back to the auto-scaling.

### Commands

The Command panel also display the currently selected command. After indicated all needed parameters, the Apply button need to be clicked to perform the command. If the command is successfull, the button will change to a checked symbol. If an error occured, an error message should be displayed and the apply button will change to a warning symbol. A new click on the button will make it change back to the Apply state, then the command can be used again.

When selected another command, all data previously entered in the previously command will be lost. Outputs are saved in memory.

For commands involving file data (loading or saving), the file format currently supported is CSV. Please note that special headers need to be used for the file to be considered valid, otherwise an error will be raised.

To retrieve computation results, the surface and model data can be exported. The files generated when saving the project could also be used, but only contain minimal information. The angles are expressed in radians in the saved files.
