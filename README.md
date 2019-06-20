# RustRadio

RustRadio is an SDR-focused signal processing framework built using Rust. It provides a library with low- and high-level signal processing blocks, as well as a command line (CLI) tool for building complete lightweight signal processing pipelines.

It is still in the inital design and specification phase.

## Motivation

Recent years have seen a meteoric rise in usage of SDR applications. Many various software pipelines have been built, some based on the largely popular GNU Radio framework, and others rolling their own signal processing code.

RustRadio proposes a design for a modern SDR-focused signal processing framework that provides a holistic solution for building modern pipelines:

- **Composability** - processing blocks should be arbitrarily composable at various abstractions levels from the lowest blocks that convert between numerical types to the highest level de/modulation blocks.
- **Distribution** - existing code should either be available upstream within RustRadio, or otherwise should be easily discoverable and packaged ready for use.
- **Availability** - modern SDR applications are increasingly run on low-power single board computers (SBCs), the final binaries should support running on multiple architectures (mainly targeting `x86_64`, `i686`, `aarch64`, `armv7` and even `riscv` once it is stabilised and usable).

## Usage

RustRadio is designed to support two main usages:

- **Prebuilt binary** - the entire flow is coded in Rust, and a final executable binary representing the flow execution is built, ready for deployment.
- **Script-based invocation** - a flow is scripted using a simple text-based declarative format. The script can then be fed into a RustRadio execution engine which will dynamically parse, build and execute the flow.

Performance charecteristics of both methods should be relatively similar. Prebuilding a binary is easier to deploy and allows integration with arbitrary Rust code.

## Design

RustRadio is designed to be implemented using the new `async`/`await` ecosystem, which as of now is still pending a stable release. Expect design to change heavily as the ecosystem matures.

Generally speaking, each block implementes a `Stream` and/or `Sink` as neccesary, while the scheduling is left to the async runtime. Thus, RustRadio optimizes for block implementation simplicity.

### Blocks

- Each block implements `Read` and `Write` traits as neccesary
- Flow of new data items is `await`-ed upon with scheduling left to default async scheduling behavior (pending any specific optimizations)
- Blocks may hold internal buffers for queueing partial data items (e.g. a strict moving average filter of size `N` might choose not to output any items until the sliding window is saturated with `N` items which have been received)

## License

[GPLv3](LICENSE)

