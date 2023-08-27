## [0.2.5](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.2.4...v0.2.5) (2023-08-27)


### Performance Improvements

* lazy static optimization for regexp ([447289c](https://github.com/web-infra-dev/mdx-rs-binding/commit/447289c649887c48faadf01d4b0ac644d012dfd7))
* use `mimalloc` ([984c0cc](https://github.com/web-infra-dev/mdx-rs-binding/commit/984c0cc91cddeb9ef4229d85a34c03527692afb9))



## [0.2.4](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.2.3...v0.2.4) (2023-07-25)


### Bug Fixes

* extract text when header including link node ([06063a3](https://github.com/web-infra-dev/mdx-rs-binding/commit/06063a3eaa312c5c98a3a13250f13f9ad6e13bbd))



## [0.2.3](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.2.2...v0.2.3) (2023-07-13)


### Bug Fixes

* slugger normalize ([73c43d8](https://github.com/web-infra-dev/mdx-rs-binding/commit/73c43d86bd4a34256c339ccde071d562fa4b67d2))



## [0.2.2](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.2.1...v0.2.2) (2023-06-20)


### Bug Fixes

* delete export in mdx output ([386ebc1](https://github.com/web-infra-dev/mdx-rs-binding/commit/386ebc1345e208a102f4b152f44941ec50db4841))



## [0.2.1](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.2.0...v0.2.1) (2023-06-01)


### Bug Fixes

* avoid panic when swc parse error ([3c5f779](https://github.com/web-infra-dev/mdx-rs-binding/commit/3c5f7795d92bcff3e26744a962880396a4300de3))



# [0.2.0](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.1.8...v0.2.0) (2023-05-23)


### Features

* delete code_block plugin ([a72fdff](https://github.com/web-infra-dev/mdx-rs-binding/commit/a72fdff08bc7702520f7af624079a386f20bbb5e))



## [0.1.8](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.1.7...v0.1.8) (2023-05-12)


### Bug Fixes

* panic when hash include . ([e092412](https://github.com/web-infra-dev/mdx-rs-binding/commit/e092412401549a2d3b4dd183d3ca3c49855b55db))



## [0.1.7](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.1.6...v0.1.7) (2023-05-12)


### Bug Fixes

* hash missed in link ([30885b6](https://github.com/web-infra-dev/mdx-rs-binding/commit/30885b65f03713a18e46277a93fa9ec52fa59bdd))



## [0.1.6](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.1.5...v0.1.6) (2023-05-10)


### Bug Fixes

* **container:** should render container type with space correctly ([690de1a](https://github.com/web-infra-dev/mdx-rs-binding/commit/690de1a19c1f6c442d0a63a01d7674bee4cb8eb0))


### Features

* remove copy button in code block ([9d44cac](https://github.com/web-infra-dev/mdx-rs-binding/commit/9d44cac28b57929d3fada999fb75dfc2b474dab1))



## [0.1.5](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.1.4...v0.1.5) (2023-05-05)


### Bug Fixes

* slug normalize for `.` ([5f52075](https://github.com/web-infra-dev/mdx-rs-binding/commit/5f5207520f70977faad7e4d2ef5fc77d05377f9a))



## [0.1.4](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.1.3...v0.1.4) (2023-04-24)


### Bug Fixes

* nest ../ path not work ([1478e06](https://github.com/web-infra-dev/mdx-rs-binding/commit/1478e06fd23e125b7b9139818750b784af948021))


### Features

* add install and usage in readme ([2973e1a](https://github.com/web-infra-dev/mdx-rs-binding/commit/2973e1a08115a337a8a5fd1caa8b2290748b4013))



## [0.1.3](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.1.2...v0.1.3) (2023-04-13)


### Bug Fixes

* jsx element support in container ([0f880cf](https://github.com/web-infra-dev/mdx-rs-binding/commit/0f880cf4c3eb2829d45fd5224a3846646b665f71))



## [0.1.2](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.1.1...v0.1.2) (2023-04-11)


### Bug Fixes

* slug normalize ([80eb470](https://github.com/web-infra-dev/mdx-rs-binding/commit/80eb47019c232a8f087d8d914f2a5dda977ca3c6))



## [0.1.1](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.1.0...v0.1.1) (2023-04-11)


### Bug Fixes

* handle external image src ([bc515fb](https://github.com/web-infra-dev/mdx-rs-binding/commit/bc515fbf54a33b8e643e448458467b27a75121d6))



# [0.1.0](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.0.2...v0.1.0) (2023-04-11)


### Bug Fixes

* should render p tag for every line in container ([8bc96b5](https://github.com/web-infra-dev/mdx-rs-binding/commit/8bc96b53ce9fc6e7f9c70eccc1fbce681cae3bc8))
* should wrap container content with p tag ([ef34367](https://github.com/web-infra-dev/mdx-rs-binding/commit/ef3436736a61d6ba4d655e9691bd1568a249208e))
* skip class name in htm plugin ([bc2a3a4](https://github.com/web-infra-dev/mdx-rs-binding/commit/bc2a3a44d224aafe32ad59f205fbe593e5e6f0ee))


### Features

* remove default_lang in normalize_link ([90fc8c3](https://github.com/web-infra-dev/mdx-rs-binding/commit/90fc8c38337233ebdacb1ce688d27c2e8aebfbc5))
* support img assets in normalize_link plugin ([cb00c66](https://github.com/web-infra-dev/mdx-rs-binding/commit/cb00c66f15375a4032b04c5b9fe1a85ca0307efe))


### Performance Improvements

* prefer using reference ([1de9387](https://github.com/web-infra-dev/mdx-rs-binding/commit/1de9387753681970509429ecbe1e2c6d50678251))



## [0.0.2](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.0.1...v0.0.2) (2023-04-03)



## [0.0.1](https://github.com/web-infra-dev/mdx-rs-binding/compare/v0.0.0...v0.0.1) (2023-03-30)


### Bug Fixes

* development mode specify ([9ff54d9](https://github.com/web-infra-dev/mdx-rs-binding/commit/9ff54d9dfb475ac1fe9b984997b37b26b10ef1a7))
* public access ([1e8eb5e](https://github.com/web-infra-dev/mdx-rs-binding/commit/1e8eb5ec26f146be77536da48d2140ad09dd2bf6))



# [0.0.0](https://github.com/web-infra-dev/mdx-rs-binding/compare/6e5b1f29211ef840aab03708ccfb8a9f841239d0...v0.0.0) (2023-03-30)


### Bug Fixes

* ci ([624d729](https://github.com/web-infra-dev/mdx-rs-binding/commit/624d7299dba2490b94f9f041b66c3d560530ee69))
* container plugin error in mdx expression title ([cd6dbe7](https://github.com/web-infra-dev/mdx-rs-binding/commit/cd6dbe77ca12322e711f9406d807bce92b8b269a))


### Features

* add normalize_link plugin ([254720a](https://github.com/web-infra-dev/mdx-rs-binding/commit/254720ab3a05c5d13c56d41574d43f16677e0e45))
* add plugin_html and export more info to js ([6f0c942](https://github.com/web-infra-dev/mdx-rs-binding/commit/6f0c942abaaaecaac02ed8beda46e4e2cc372087))
* init workspace and plugins ([6e5b1f2](https://github.com/web-infra-dev/mdx-rs-binding/commit/6e5b1f29211ef840aab03708ccfb8a9f841239d0))
* object form options ([84b8680](https://github.com/web-infra-dev/mdx-rs-binding/commit/84b86807c6d8c8ee74f2ba07811ae6f54925c793))
* update ci & add bench ([0bad411](https://github.com/web-infra-dev/mdx-rs-binding/commit/0bad411e724986c61f922d544b560b8b15da1069))



