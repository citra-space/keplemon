# KepLemon

[Citra Space Corporation's](https://citra.space) Rust-accelerated astrodynamics package built on the shared libraries
provided by [space-track](https://space-track.org). Please visit the [documentation](https://keplemon.citra.space)
page for additional details.

## Installation

### From PyPI (Preferred)

KepLemon can be installed directly for any operating system from a command line using `pip install keplemon`.

### From a Local Build

For python users, the **_preferred installation method is through [PyPI](https://www.pypi.org)_**; however, the package can be
installed locally by following the steps below.

1. `git clone https://github.com/citra-space/keplemon.git`
2. `cargo install cargo-make`
3. `cargo make build-<os>-<architecture>` (e.g. for Linux x86 `cargo make build-linux-x86`)
4. `pip install target/wheels/*.whl`

To avoid potential conflicts with system libraries, you can alternatively build the wheel using Docker.

```bash
docker buildx build \
  --build-arg OS="${OS:-linux}" \
  --build-arg ARCH="${ARCH:-x86}" \
  --build-arg PYTHON_VERSION="${PYTHON_VERSION:-310}" \
  --output type=local,dest=./target .
```

Then install the wheel the same way with:
`pip install target/wheels/*.whl`

## Environment Settings

Although not required, it is recommended to explicitly apply the settings in this section before using KepLemon to avoid
unexpected behaviors and inaccurate calculations.

### CPU Limits

By default, KepLemon will have access to all available cores when performing parallel functions. Limit this by calling
`set_thread_count` **_before_** using other KepLemon functions.

```python
from keplemon import set_thread_count, get_thread_count

# Update this to the desired core count
set_thread_count(4)
```

### Time Constants and Earth-Orientation Parameters (EOP)

All astrodynamics packages have a strict dependence on measured changes to time and Earth's orientation. Since KepLemon
uses the public Standardized Astrodynamics Algorithms Library (SAAL) at the core, the time and (EOP) data must conform
to a specific format required by the underlying binaries. Rather than referencing data directly provided by the
[USNO](https://maia.usno.navy.mil/), utility scripts are provided in KepLemon to request and export the relevant data.

#### Global Update

Use the command below from a terminal to update time constants and EOP data package-wide.

```bash
keplemon --update-eop global
```

#### Local Override

EOP data can also be written to explicit paths for inspection or package overrides using the commands below.

```bash
keplemon --update-eop custom_path.txt
```

!!! note
If you intend to use the data written to a local override, you must use the `load_time_constants` method at the
beginning of your scripts. **_This is not needed if you maintain constants using the global method_**.

```python
from keplemon.time import load_time_constants

# Update this to reflect the desired override path
load_time_constants("custom_path.txt")
```

## Contributing

Anyone is welcome to contribute to KepLemon. Users are encouraged to start by opening issues or forking the repository.
Changes directly to the baseline may be approved by contacting the owner at <brandon@citra.space>.
