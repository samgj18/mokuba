mod core;

pub use crate::core::model::{error, params};
pub use crate::core::mstd::{read_line_from, write_line_to};

use crate::core::gen_with_seed;
use crate::core::model::{error::GenError, params::PassParams};

/**

## Password Generator

### Examples
```
use mokuba::{gen, params::PassParams};
let password = gen(PassParams { length: 10 });
```

This function will return an error if the length is less than 1 or if the acc seed is different than an empty string.
*/
pub fn gen(params: PassParams) -> Result<String, GenError> {
    gen_with_seed(params, "")
}
