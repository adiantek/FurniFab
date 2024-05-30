# Building

## Architecture overview

<img src="Architecture.svg" alt="Graphic architecture schema">

FurniFab is a polyglot application.
It is written in C, Rust, Python and TypeScript.
The UI is written in TypeScript using [Vue.js](https://github.com/vuejs/)
and [Bootstrap Vue Next](https://github.com/bootstrap-vue-next/bootstrap-vue-next).
For the integration, executable generation and OS interaction parts, the [Tauri](https://github.com/tauri-apps/tauri)
framework is used.
The backend part is written in C, Rust and Python.
C and Rust are used because of their speed as they are compiled languages, Python for its rich library base.

Application is divided into several modules based on business logic, responsibilities and used technologies.
The main modules are:

- Visualization module (TypeScript) responsible for displaying the results of the computation modules
- Integration module (Rust) responsible for integrating the results of the computation modules and sending them to the
  visualization module
- Four computation modules responsible for running algorithms and calculations on the data:
    - Delivery module (Python)
    - Material management module (C)
    - Cutting management module (Rust)
    - Assembly management module (Python)

### Visualization module

Visualisation module is a Vue.js application that uses Bootstrap Vue Next for styling.
It is bundled using Vite and tested using Vitest.
For navigation, Vue Router is used.

Folder structure:

- `assets` - contains images, fonts, etc.
- `components` - contains Vue components for general use
- `composables` - contains Vue composable functions
- `views` - contains Vue views that are used as pages
- `api.ts` - contains API calls for Tauri backend
- `router.ts` - contains Vue router configuration
- `main.ts` - entry point for the application

#### Prerequisites

Ensure you have the following installed:

- Node >= 20.12
- NPM >= 8.1

#### Running

Run the following commands to start development server:

```bash
 npm install 
 npm run dev
```

#### Building

Run the following command to build the project:

```bash
 npm run build
```

#### Testing

Run the following command to run tests:

```bash
 npm run test:unit
```

### Integration module

Integration module is a Tauri application that uses Rust for the backend.
Source code is located in `src-tauri` folder.
It mostly consists of Rust code used to integrate the computation modules and send the results to the
visualization.
To communicate with Python modules, it uses custom C bindings running cpython api.
C code is integrated with C ffi and bindings generated using [rust-bindgen](https://github.com/rust-lang/rust-bindgen).
Rust code is compiled using Tauri CLI, as well as the final executable installers.
This module also contains Python interpreter as static file and required libraries that are embedded in the final
executable.
We are also using Tauri updater to provide automatic updates for the application with combination of GitHub Pages.

#### Prerequisites

Ensure you have the following installed:

- Rust >= 1.77

All below cargo commands (except `tauri` commands) should be run in `src-tauri` folder.
Also, to build installers, you need Tauri CLI:

```bash
cargo install tauri-cli
```

#### Running

Run the following command to start the application:

```bash
cargo run
```

#### Building

Run the following command to build the project:

```bash
cargo build
```

#### Testing

Run the following command to run tests:

```bash
cargo test
```

#### Building installers

To build installers, you need to generate a key pair:

```bash
cargo tauri signer generate
```

The above line will generate instructions on how to provide a private key during the build phase.
The public key must be replaced in `src-tauri/tauri.conf.json`.

Run the following commands to build installers:

```bash
cargo tauri build
```

#### Running whole application

To run the whole application, you need to run:

```bash
npm install
cargo tauri dev
```

### Delivery module

Delivery module is a Python script that is responsible for calculating optimal delivery of the materials.

#### Algorithms

It solves a `minimum cost flow problem` using custom implementation of:

- Cycle canceling algorithm
- Bellman-Ford algorithm
- Ford-Fulkerson algorithm

#### Prerequisites

Ensure you have the following installed:

- Python >= 3.10

#### Running

Run the following command to start the application:

```bash
python algo/max_flow_min_cost/negative_cycle_removal.py
```

By default, the script will read data from `file_negative_cycle_x2.txt` file and write results to standard output.
Example data files are located in `algo/max_flow_min_cost` directory.

#### Tests

Tests are written using `unittest` module.

Run the following command inside `algo/max_flow_min_cost` directory to run tests:

```bash
python -m unittest discover -p automatic_tests*.py
``` 

### Material management module

TODO: Add Material management module description

#### Algorithms

TODO: Add Material management module algorithms description

#### Prerequisites

TODO: Add Material management module prerequisites

#### Running

TODO: Add Material management module running description

#### Building

TODO: Add Material management module building description

#### Tests

TODO: Add Material management module tests description

### Cutting management module

Cutting management module is a Rust library
that is responsible for calculating optimal schedule for cutting of the materials.

#### Algorithms

It solves `The Capacitated Scheduling Problem with Conflict Jobs` (`P|conflict|ΣwjUj`) using custom implementation of:

- List algorithm
- Variable neighborhood search algorithm
- Heuristic algorithm

These algorithms are heuristic algorithms that are used to solve the problem in a reasonable time
and may not always find the optimal solution.
Furthermore, the algorithms are randomized and may produce different results each time they are run.

#### Prerequisites

Ensure you have the following installed:

- Rust >= 1.77

Following commands should be run in `algo/scheduling_conflicts` directory.

#### Running

Run the following command to start the application:

```bash
cargo run --bin <algorithm_name>
```

Where `<algorithm_name>` is one of the following:

- `list_algorithm`
- `vns`
- `tresoldi`

By default, the script will read data from standard input and write results to standard output.
Example data files are located in `algo/scheduling_conflicts` directory.

#### Building

Run the following command to build the project:

```bash
cargo build
```

#### Tests

Run the following command to run tests:

```bash
cargo test
```

### Assembly management module

Assembly management module is a Python script that is responsible for calculating optimal schedule for assembly of the
materials.

#### Algorithms

It solves a flow shop for two machines scheduling problem (F2|rj, pmtn|Cmax) using custom implementation of:

- Johnson's algorithm
- Branch and bound algorithm
- Horn's algorithm
- Nawaz-Enscore-Ham algorithm

These algorithms are heuristic algorithms that are used to solve the problem in a reasonable time
and may not always find the optimal solution.
Furthermore, the algorithms are randomized and may produce different results each time they are run.

#### Prerequisites

Ensure you have the following installed:

- Python >= 3.10

#### Running

Run the following command to start the application:

```bash
python algo/F2,rj,pmtn,Cmax/<algorithm_name>.py
```

Where `<algorithm_name>` is one of the following:

- `Johnson`
- `BB`
- `Horn`
- `neh`

By default, the scripts will read data source filename from standard input and write results to standard output.
Example data files are located in `algo/F2,rj,pmtn,Cmax` directory.

#### Tests

Tests are written using `unittest` module.

Run the following command inside `algo/F2,rj,pmtn,Cmax` directory to run tests:

```bash
python -m unittest discover -p automatic_tests*.py
``` 

## CI & CD overview

For CI/CD, GitHub Actions, GitHub Pages, and GitHub Releases are used.

### Continuous Integration

The Repository contains GitHub Actions workflows that are triggered on push and pull request events.
There are three workflows:

- `build` - runs on every push and pull request event, builds the project and runs tests
- `eslint` - runs on every push and pull request event, checks the code for linting errors
- `rust-clippy` - runs on every push and pull request event, checks the Rust code for clippy errors

Tests for each module are run separately in their respective directories.
Key for signing the installers is added as a secret in the repository.
For now, a project is built on these platforms:

- Windows AMD64
- Windows ARM64
- Linux AMD64
- MacOS AMD64
- MacOS ARM64

There is a python script helper in the repository that is used to generate and move the installers and their signatures
to the correct directory.
This script is placed in the `scripts` directory with the name `build.py`.

### Continuous Deployment

The Repository contains GitHub Action workflow triggered on push events and on release creation that is responsible for
deploying the application to GitHub Pages.
It is part of the `build` workflow.
It uses helper script `scripts/deploy.py` to deploy the application to GitHub Pages, prepare installers and update
packages with signatures.
On the GitHub Pages branch, there are two directories:

- `release` - contains installers for stable releases
- `snapshot` - contains installers for snapshot releases

Stable releases are created by creating a new release in the repository.
Snapshot releases are created by every push to the main branch.
Each folder contains installers for all supported platforms and update packages along with json files that contain
information about the release and signatures.
More detail about json file format can be found in
the [Tauri documentation](https://tauri.app/v1/guides/distribution/updater/).

## Other

### Lib directory

Lib directory contains compiled static libraries for all supported platforms of the C part of the project.

### Generator script

Generator script is a Python script used to generate random data for the computation modules.
It is located in the `scripts` directory.