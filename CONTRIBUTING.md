# Contributing Guide

Thanks for that you are interested in contributing to Rspress. Before starting your contribution, please take a moment to read the following guidelines.

---

## Setup the Dev Environment

### Fork the Repo

[Fork](https://help.github.com/articles/fork-a-repo/) this repository to your
own GitHub account and then [clone](https://help.github.com/articles/cloning-a-repository/) it to your local.

### Install Rust

We recommend using Rustup to manage the rust version. You can setup your Rustup and Cargo development environment with [Getting-started](https://www.rust-lang.org/learn/get-started).

### Install Node.js

We recommend using Node.js 18. You can check your currently used Node.js version with the following command:

```bash
node -v
```

If you do not have Node.js installed in your current environment, you can use [nvm](https://github.com/nvm-sh/nvm) or [fnm](https://github.com/Schniz/fnm) to install it.

Here is an example of how to install the Node.js 18 LTS version via nvm:

```bash
# Install the LTS version of Node.js 18
nvm install 18 --lts

# Make the newly installed Node.js 18 as the default version
nvm alias default 18

# Switch to the newly installed Node.js 18
nvm use 18
```

### Install pnpm

```sh
# Enable pnpm with corepack, only available on Node.js >= `v14.19.0`
corepack enable
```

### Install Dependencies

```sh
pnpm install
```

### Set Git Email

Please make sure you have your email set up in `<https://github.com/settings/emails>`. This will be needed later when you want to submit a pull request.

Check that your git client is already configured the email:

```sh
git config --list | grep email
```

Set the email to global config:

```sh
git config --global user.email "SOME_EMAIL@example.com"
```

Set the email for local repo:

```sh
git config user.email "SOME_EMAIL@example.com"
```

---

## Making Changes and Building

Once you have set up the local development environment in your forked repo, we can start development.

### Checkout A New Branch

It is recommended to develop on a new branch, as it will make things easier later when you submit a pull request:

```sh
git checkout -b MY_BRANCH_NAME
```

## Submitting Changes

### Committing your Changes

Commit your changes to your forked repo, and [create a pull request](https://help.github.com/articles/creating-a-pull-request/).

### Format of PR titles

The format of PR titles follow Conventional Commits.

An example:

```
feat(plugin-swc): Add `myOption` config
^    ^    ^
|    |    |__ Subject
|    |_______ Scope
|____________ Type
```

---

## Releasing

Repository maintainers can publish a new version of all packages to npm.

Here are the steps to publish (we generally use CI for releases and avoid publishing npm packages locally):

1. Modify version field in [package.json](./package.json) to bump major/minor/patch version number

2. Execute `pnpm run release-commit` to create a `chore(release)` commit and open a pull request

3. Review and merge the release pull request
