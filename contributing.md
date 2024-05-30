# Contributing


## Contributing a Function Definition
Functions are defined in a `.sdf.yml` block. The SDF YML schema definition is [here](https://cdn.sdf.com/schemas/sdf-schema-1.2.json). For YML autocomplete, please download the [Red Hat YML extension](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml) in VSCode.

Notes:
* Define a new function signature in the appropriate dialect folder in the `signatures/` directory. 
* Please where possible, add the `cross-link` parameter to link to the official function documentation
* Functions are monomorphic, so multiple signatures may be necessary for multiple datatypes
* Functions are unique for each dialect, and each input parameter set

**Note: Please ensure that you are only adding a function that does not yet exist**

```
---
function:
  name: [name of the funtion]
  description: [description of the function]
  parameters:
  - datatype: [datatype]
  optional-parameters: []
  returns:
    datatype: [datatype]
  implemented-by: builtin
  cross-link: [link to official documentation]
  variadic: [any, uniform, non-uniform, even-odd]
```


## Contributing a function Implementation
*todo*