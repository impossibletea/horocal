# HoroCal

Horoscope in the form of widely known `cal`.
Shows you the best and worst days for doing stuff.

## Usage

Currently equivalent to just `cal` with no args: shows current month with days
highlighted appropriately.

## Installation

Requires rustc.

```console
$ rustc nobuild.rs
$ ./nobuild
$ mv build/hcal ~/.local/bin/
$ HCAL_SIGN=scorpio hcal
```

