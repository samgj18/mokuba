mod core;

use crate::core::gen_with_seed;
pub use crate::core::model::{GenError, Params};

/**

## Password Generator

### Examples
```
use mokuba::{gen, Params};
let password = gen(Params { length: 10 });
```

This function will return an error if the length is less than 1 or if the acc seed is different than an empty string.
*/
pub fn gen(params: Params) -> Result<String, GenError> {
    gen_with_seed(params, "")
}
