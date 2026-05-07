<p align="center">
  <img src="https://raw.githubusercontent.com/JGGR/F-value/master/assets/logo.png" alt="F-Value" width="240" align="middle" />&nbsp;&nbsp;&nbsp;
  <img src="https://raw.githubusercontent.com/JGGR/F-value/master/assets/logo_name.png" alt="" align="middle" />
</p>

# F-value

## Tool for NISECI and HFBI calc

## Table of Contents

+ [What is this thing?](#witt)
+ [Input templates](#input_templates)
+ [Locale](#locale)
+ [References](#references)

## What is this thing? <a name = "witt"></a>

This is a tool for calculating the NISECI and/or HFBI index for a dataset.

## Input templates <a name = "input_templates"></a>

You can find templates for the input files in the `templates` folder.

## Locale <a name = "locale"></a>

Since this tool is built with knowing that Excel uses some specific separators with the Italian locale, the tool supports two formats for input/output depending on the platform locale.

You can change the detected locale in the settings.

The format differences are:
- Italian:
  - Input expects `;` as csv field delimiter, and `,` can be used as float decimal delimiter
  - Output uses `;` as csv field delimiter, and floats are printed with `,` as decimal delimiter
- International:
  - Input expectes `,` as csv field delimiter, and `.` as float decimal delimiter
  - Output uses `,` as csv field delimiter, and floats are printed with `.` as decimal delimiter

## References <a name = "references"></a>

- [ISPRA - Nuove indice dello stato ecologico delle comunità ittiche - NISECI](https://www.isprambiente.gov.it/it/pubblicazioni/manuali-e-linee-guida/nuovo-indice-dello-stato-ecologico-delle-comunita-ittiche-niseci)
- [ISPRA - Manuale per la classificazione dell'Elemento di Qualità Biologica "Fauna Ittica" nelle lagune costiere italiane - HFBI](https://www.isprambiente.gov.it/it/pubblicazioni/manuali-e-linee-guida/manuale-per-la-classificazione-dell-elemento-di-qualita-biologica-fauna-ittica-nelle-lagune-costiere-italiane)
