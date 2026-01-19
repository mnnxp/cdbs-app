<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://gitlab.com/cadbase/cdbs-app">
    <img src="data/logo_min.svg" alt="Logo" width="80">
  </a>

  <h3>CADBase Platform Frontend</h3>

  <p>
    CADBase is a digital platform for sharing 3D models, drawings, and engineering data. This frontend provides an interface for exploring, managing, and interacting with engineering components and companies.
    <br />
    <a href="https://cadbase.rs">View site</a>
    ·
    <a href="https://gitlab.com/cadbase/cdbs-app/issues">Report Bug</a>
    ·
    <a href="https://gitlab.com/cadbase/cdbs-app/issues">Request Feature</a>
  </p>
</div>


<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a></li>
    <li><a href="#getting-started">Getting Started</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#test">Test</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#repositories">Repositories</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>


<!-- ABOUT THE PROJECT -->
## About The Project

[![Page Component Screen Shot][product-screenshot]](https://cadbase.rs)

This platform facilitates the sharing and versioning of parts, while offering API access and seamless integration options.

There are many solutions for sharing 3D models, drawings and other engineering data. However, the CADBase platform has a number of distinctive features that improve product data management by offering an effective solution for both individual use and collaborative work in engineering and design.

### Additional Description

Since 2018, our platform has grown and evolved through the valuable contributions of Ivan Nosovsky, Xia TianHao (夏添豪), Julia Gerasimova, and several other contributors. Our mission is to cultivate a collaborative community that drives innovation and turns ideas into quality engineering and design solutions.

We hope you enjoy using CADBase. We're currently seeking manufacturers interested in partnering with us and sharing valuable feedback to help us improve and grow together.

### Key Features & Improvements

- **Discussion & Commenting:** Comment on components, companies, and services to foster community engagement and quick feedback, enhancing collaboration.  
- **Import/Export:** Import component parameters and modifications directly through the website from Excel sheets to save time and reduce manual errors. Additionally, utilize APIs to efficiently handle large-scale bulk imports and exports of data.  
- **Markdown & UI:** Edit with Markdown and preview during the editing process, making content creation easier and providing clearer, well-formatted descriptions for documentation.  
- **Integrations:** Pre-built open-source integrations let you easily create add-ons and connect with other tools, streamlining workflows. No licensing fees for API access make expansion and customization simple and cost-effective.  
- **Self-hosted:** Deploying the platform yourself grants full control over its infrastructure, enabling you to tailor and optimize it precisely to meet your specific requirements.  
- **Login-Free Access:** Browse component pages without login for quick and hassle-free access to information.

### Benefits for Individual Users

Mechanical engineers and inventors can leverage this platform to enhance their workflows with the following features:
* Manage complex data dependencies, such as file sets for component modifications, to streamline work across CAD, CAE, CAM, and other systems
* Access API functionalities and connect securely to the server using personal authorization tokens
* Integrate seamlessly with various design and engineering tools, enabling extensive workflow automation
* Utilize file versioning to revert to previous states, review changes

Our goal is to develop a versatile solution that enables efficient project management and seamless data sharing within engineering workflows, tailored to meet the needs of most engineers and creators. 

If you're interested, we're also on YouTube <a href="https://www.youtube.com/channel/UC-dHiTHBGV88ScxFKSC3srw">CADBase Platform</a>.

And if you're not interested in this site and its functionality, you can try to use the code base for your own projects. In any case, have peace and goodwill ;)

<div align="right">(<a href="#about-the-project">back to top</a>)</div>

<!-- INTEGRATION FRAMEWORK -->
### Integration Framework

Creating an ecosystem-agnostic environment to:
- Bridge compatibility gaps between engineering tools
- Enable seamless CAD, PLM, ERP, and supply chain interoperability
- Eliminate manual conversion and vendor lock-in

<div align="right">(<a href="#about-the-project">back to top</a>)</div>

<!-- AVAILABLE INTEGRATIONS -->
### Available Integrations

Solutions for integration with CADBase platform with our support:

 - [CADBase Library (GitHub)](https://github.com/mnnxp/cadbaselibrary-freecad/) workbench for **FreeCAD** parametric 3D modeler
 - [CADBase Library (Blender Extensions)](https://extensions.blender.org/add-ons/cadbase-library/) add-on for **Blender** 3D computer graphics program

We welcome contributions to new integrations!

<div align="right">(<a href="#about-the-project">back to top</a>)</div>


### Built With

<div align="center">
<img src="https://yew.rs/img/logo.svg" alt="Yew Logo" width="40">
<img src="https://github.com/jgthms/bulma/blob/main/docs/assets/images/bulma-icon.png?raw=true" alt="Bulma Logo" width="40">
<img src="https://github.com/mrdoob/three.js/blob/dev/icon.png?raw=true" alt="Three.js Logo" width="40">
</div>


| Other libraries |
| ------------- |
| [![router][router]][router-url] [![yewtil][yewtil]][yewtil-url] [![bindgen][bindgen]][bindgen-url] [![logger][logger]][logger-url] [![instant][instant]][instant-url] [![lipsum][lipsum]][lipsum-url] [![log][log]][log-url] [![getrandom][getrandom]][getrandom-url] [![rand][rand]][rand-url] [![chrono][chrono]][chrono-url] [![dotenv_codegen][dotenv_codegen]][dotenv_codegen-url] [![lazy_static][lazy_static]][lazy_static-url] [![parking_lot][parking_lot]][parking_lot-url] [![cmark][cmark]][cmark-url] [![serde][serde]][serde-url] [![regex][regex]][regex-url] [![serde_json][serde_json]][serde_json-url] [![thiserror][thiserror]][thiserror-url] [![graphql_client][graphql_client]][graphql_client-url] [![wee_alloc][wee_alloc]][wee_alloc-url] [![web-sys][web-sys]][web-sys-url] [![wasm-bindgen-test][wasm-bindgen-test]][wasm-bindgen-test-url] [![toml][toml]][toml-url] [![js-sys][js-sys]][js-sys-url] [![console_error_panic_hook][console_error_panic_hook]][console_error_panic_hook-url] [![anyhow][anyhow]][anyhow-url] |


Many thanks to all of you who have contributed to the projects listed above and to the projects listed in the <a href="#acknowledgments">Acknowledgments</a> section! Your input has allowed us to make CADBase better and faster.

<div align="right">(<a href="#about-the-project">back to top</a>)</div>


<!-- GETTING STARTED -->
## Getting Started

To start the frontend, you must perform the a few steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* [Rust](https://www.rust-lang.org/learn/get-started)

  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* [Trunk](https://trunkrs.dev/)

  ```sh
  # Install via homebrew on Mac, Linux or Windows (WSL).
  brew install trunk
  # Install a release binary (great for CI).
  # You will need to specify a value for ${VERSION}.
  wget -qO- https://github.com/thedodd/trunk/releases/download/${VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
  # Install via cargo.
  cargo install --locked trunk
  # Until wasm-bindgen has pre-built binaries for Apple M1, M1 users will
  # need to install wasm-bindgen manually.
  cargo install --locked wasm-bindgen-cli
  ```

### Installation

Note: Before performing step 2, check the correct settings of the environment in the file '.env'.

1. Clone the repo
   ```sh
   git clone https://gitlab.com/cadbase/cdbs-app.git
   ```
2. Build, watch & serve the Rust WASM app and all of its assets
   ```sh
   trunk serve
   ```

<div align="right">(<a href="#about-the-project">back to top</a>)</div>


<!-- USAGE -->
## Usage

Open app at `http://localhost:8080`.

For API details: [docs.cadbase.rs](https://docs.cadbase.rs)

<div align="right">(<a href="#about-the-project">back to top</a>)</div>

<!-- TEST -->
## Test

If you want to run tests, a few tests are located in the 'tests' folder.

### Prerequisites

To run the tests, you will need to install the Playwright testing framework and its dependencies.

```sh
# Install Playwright
npm init playwright@latest
# Install browser binaries
npx playwright install
# Run tests
npx playwright test tests/slogan.spec.js

# or (if you prefer yarn)

# Install Playwright
yarn create playwright
# Install browser binaries
yarn playwright install
# Run tests
yarn playwright test tests/slogan.spec.js
```

<div align="right">(<a href="#about-the-project">back to top</a>)</div>

<!-- ROADMAP -->
## Roadmap

- [ ] Add Guidance and Manuals
- [x] Add versioning support for files
- [x] Add 3D Viewer STL, GCODE, GLTF/GLB and IFC (via [Three.js](https://github.com/mrdoob/three.js))
- [x] Search page
- [ ] Multi-language Support
    - [x] English
    - [x] Russian
    - [x] Chinese
    - [ ] Spanish

See the [open issues](https://gitlab.com/cadbase/cdbs-app/issues) for a full list of proposed features (and known issues).

<div align="right">(<a href="#about-the-project">back to top</a>)</div>


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<div align="right">(<a href="#about-the-project">back to top</a>)</div>


<!-- REPOSITORIES -->
## Repositories

The code is distributed in several repositories so that people who want access to the code can get it.

We hope that everyone will be able to access the code published here.

### Main

GitLab - https://gitlab.com/cadbase/cdbs-app

### Mirrors

Codeberg - https://codeberg.org/mnnxp/cdbs-app

GitHub - https://github.com/mnnxp/cdbs-app

<div align="right">(<a href="#about-the-project">back to top</a>)</div>


<!-- LICENSE -->
## License

Distributed under the MIT License. See [LICENSE](/LICENSE) for more information.

<div align="right">(<a href="#about-the-project">back to top</a>)</div>


<!-- CONTACT -->
## Contact

Ivan Nosovsky - in@cadbase.rs

Xia TianHao - xth@cadbase.rs or [Sansx](https://github.com/sansx) (GitHub)

<div align="right">(<a href="#about-the-project">back to top</a>)</div>


<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Yew-graphql-demo](https://github.com/sansx/yew-graphql-demo)
* [Webapp.rs](https://github.com/saschagrunert/webapp.rs)
* [Canduma](https://github.com/clifinger/canduma)
* [Best-README-Template](https://github.com/othneildrew/Best-README-Template)
* [Choose an Open Source License](https://choosealicense.com)
* [Container Registry](https://container-registry.com)
* [Img Shields](https://shields.io)
* [Jordan Leigh](https://jordizle.com)

<div align="right">(<a href="#about-the-project">back to top</a>)</div>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[product-screenshot]: data/Main_default.png
[router]: https://img.shields.io/badge/router-blue
[router-url]: https://docs.rs/yew-router
[yewtil]: https://img.shields.io/badge/yewtil-blue
[yewtil-url]: https://docs.rs/yewtil
[bindgen]: https://img.shields.io/badge/bindgen-blue
[bindgen-url]: https://docs.rs/wasm-bindgen
[logger]: https://img.shields.io/badge/logger-blue
[logger-url]: https://docs.rs/wasm-logger
[instant]: https://img.shields.io/badge/instant-blue
[instant-url]: https://docs.rs/instant
[lipsum]: https://img.shields.io/badge/lipsum-blue
[lipsum-url]: https://docs.rs/lipsum
[log]: https://img.shields.io/badge/log-blue
[log-url]: https://docs.rs/log
[getrandom]: https://img.shields.io/badge/getrandom-blue
[getrandom-url]: https://docs.serde.rs/getrandom
[rand]: https://img.shields.io/badge/rand-blue
[rand-url]: https://github.com/bryant/rand
[chrono]: https://img.shields.io/badge/chrono-blue
[chrono-url]: https://docs.rs/chrono
[dotenv_codegen]: https://img.shields.io/badge/dotenv_codegen-blue
[dotenv_codegen-url]: https://github.com/dtolnay/dotenv_codegen
[lazy_static]: https://img.shields.io/badge/lazy_static-blue
[lazy_static-url]: https://github.com/dtolnay/lazy_static
[parking_lot]: https://img.shields.io/badge/parking_lot-blue
[parking_lot-url]: https://docs.rs/parking_lot
[cmark]: https://img.shields.io/badge/cmark-blue
[cmark-url]: https://docs.rs/pulldown-cmark
[serde]: https://img.shields.io/badge/serde-blue
[serde-url]: https://docs.rs/serde
[regex]: https://img.shields.io/badge/regex-blue
[regex-url]: https://docs.rs/regex
[serde_json]: https://img.shields.io/badge/serde_json-blue
[serde_json-url]: https://docs.rs/serde_json
[thiserror]: https://img.shields.io/badge/thiserror-blue
[thiserror-url]: https://docs.rs/thiserror
[graphql_client]: https://img.shields.io/badge/graphql_client-blue
[graphql_client-url]: https://docs.rs/graphql_client
[wee_alloc]: https://img.shields.io/badge/wee_alloc-blue
[wee_alloc-url]: https://docs.rs/wee_alloc
[web-sys]: https://img.shields.io/badge/web_sys-blue
[web-sys-url]: https://docs.rs/web-sys
[wasm-bindgen-test]: https://img.shields.io/badge/wasm_bindgen_test-blue
[wasm-bindgen-test-url]: https://docs.rs/wasm-bindgen-test
[toml]: https://img.shields.io/badge/toml-blue
[toml-url]: https://docs.rs/toml
[js-sys]: https://img.shields.io/badge/js_sys-blue
[js-sys-url]: https://docs.rs/js-sys
[console_error_panic_hook]: https://img.shields.io/badge/console_error_panic_hook-blue
[console_error_panic_hook-url]: https://docs.rs/console_error_panic_hook
[anyhow]: https://img.shields.io/badge/anyhow-blue
[anyhow-url]: https://docs.rs/anyhow
