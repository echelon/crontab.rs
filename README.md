crontab.rs
==========
A lightweight crontab parser with minimal features that does not rely on
Rust nightly. Has functions to enumerate the next time a cron schedule
should be invoked.

Crontab expressions
-------------------
```
MINUTES    HOURS    DAY_OF_MONTH    MONTHS    DAY_OF_WEEK

             Range:    Special characters:
Minutes      = [0,59]  , - * /
Hours        = [0,23]  , - * /
Day of month = [1,31]  , - * /
Months       = [1,12]  , - * /
Day of week  = [0,6]   , - * /

Ranges are inclusive.
```

(TODO: Write as EBNF notation.)

For example, **crontab.rs** supports the following:

- Wildcards: `* * * * *`
- Values: `0 0 1 1 *`
- Multiple values: `0,5,10 * * * *`
- Ranges: `0-30 * * * *`
- Steps: `*/15 * * * *`
- Combinations of all of the above: `1,2,3,5-10,*/15 * * * *`

You can use [crontab.guru](https://crontab.guru/) to build and test your crontab expressions.

Usage
-----
```rust
extern crate crontab;
extern crate time;

use crontab::Crontab;
use time::{Timespec, at_utc};

let crontab = Crontab::parse("0 * * * *").expect("unknown parse error"); // every hour

// Access to the underlying schedule components:
println!("Minutes: {:?}", crontab.schedule.minutes);
println!("Hours: {:?}", crontab.schedule.hours);

// See when the next event will occur:
crontab.find_next_event(); // Option<Tm>
crontab.find_next_event_utc(); // Option<Tm>

// Or when the next event relative to a given time is:
let time = at_utc(Timespec::new(1500001200, 0));
crontab.find_event_after(&time); // Option<Tm>
```

See `examples/usage.rs`, which is guaranteed to compile with the current
library version and make use of the library features.

TODO
----
- Fix the scheduler to support day of week (currently ignored, though the values are correctly parsed).

- Support [crontab extensions](https://docs.oracle.com/cd/E12058_01/doc/doc.1014/e12030/cron_expressions.htm),
  such as second-resolution, year numbers, and stringly-valued components.

- Support [keywords](https://www.pantz.org/software/cron/croninfo.html) such as `@yearly`, etc.

License
-------
**BSD 4-clause**

Copyright (c) 2017, Brandon Thomas. All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

3. All advertising materials mentioning features or use of this software
   must display the following acknowledgement:

   This product includes software developed by Brandon Thomas
   (bt@brand.io, echelon@gmail.com).

4. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY COPYRIGHT HOLDER "AS IS" AND ANY EXPRESS OR
IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL COPYRIGHT HOLDER BE LIABLE FOR ANY DIRECT,
INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.

