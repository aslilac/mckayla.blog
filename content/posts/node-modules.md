---
title: node_modules/ is worse than you think.
description: and we deserve better.
author: Kayla Washburn
status: draft
accent_color: "#d178d3"
cover:
  avif: https://cdn.mckayla.cloud/-/ieff75fd8xybx3/uv.avif
  default: https://cdn.mckayla.cloud/-/ieff75fd8xybx3/uv.webp
---

## Managing upgrades

- lock files (package-lock.json 24kloc)
  - even a yarn.lock can be >30kloc, 2mb
- supply chain security
  - what if adding a new dependency was a breaking change?
- what if we need incompatible versions?
- not getting along with the in-laws — what if I like _my_ dependency, but I don't like _their_ dependencies
- what if upgrading was more interactive?
  - the major version is a breaking change because...
    - this is totally possible. elm has had this for years!
  - John Boberic has been added as a maintainer
  - maybe

## Wasted cycles

- a _bunch_ of files I don't care about
- package.json
- LICENSE (arguable, but who the hell looks at the LICENSE files in their node_modules)
- CODE_OF_CONDUCT.md
- CHANGELOG.md
- README.md
- .github/ folders!?!
- .editorconfig
- plenty of other config files I don't want to spell out
- if you have a TypeScript dependency, every one source file is probably three separate delivery files (or more!!)
- Tests

All of these unnecessary file writes really add up.

## ~~Wasted~~ Dangerous cycles

- `postinstall`, `prepare`, etc.

...in any other context, we'd consider remote arbitrary code execution to be a disasterous outcome. Languages like elm, won't even let libraries run arbitrary code at _runtime_

- "it's a culture thing"
  - it's up to everyone to cultivate an ecosystem where we use dependencies responsibly

## Absolute trash

As a case study, I picked one of my small-but-reasonably-complicated projects, and looked through it's node_modules/ folder.

- 1608 Markdown files
- 172 Yaml files

```
➜  tree $(find node_modules/ -type d -name .github)
node_modules//lzma-native/.github
└── workflows
    └── ci.yml
node_modules//plist/.github
└── workflows
    └── ci.yml
node_modules//is-bigint/.github
├── FUNDING.yml
└── workflows
    ├── node-4+.yml
    ├── node-iojs.yml
    ├── node-pretest.yml
    ├── node-zero.yml
    ├── rebase.yml
    └── require-allow-edits.yml
node_modules//which-boxed-primitive/.github
└── FUNDING.yml
node_modules//flatted/.github
└── FUNDING.yml
node_modules//call-bind/.github
└── FUNDING.yml
node_modules//fast-json-stable-stringify/.github
└── FUNDING.yml
node_modules//resolve/.github
└── FUNDING.yml
node_modules//is-number-object/.github
├── FUNDING.yml
└── workflows
    └── rebase.yml
node_modules//is-negative-zero/.github
└── workflows
    ├── node-4+.yml
    ├── node-iojs.yml
    ├── node-pretest.yml
    ├── node-zero.yml
    ├── rebase.yml
    └── require-allow-edits.yml
node_modules//jsx-ast-utils/.github
└── FUNDING.yml
node_modules//is-symbol/.github
├── FUNDING.yml
└── workflows
    └── rebase.yml
node_modules//is-date-object/.github
├── FUNDING.yml
└── workflows
    └── rebase.yml
node_modules//array-includes/.github
└── FUNDING.yml
node_modules//get-intrinsic/.github
└── FUNDING.yml
node_modules//object.entries/.github
└── workflows
    ├── node-4+.yml
    ├── node-iojs.yml
    ├── node-pretest.yml
    ├── node-zero.yml
    ├── rebase.yml
    └── require-allow-edits.yml
node_modules//es-to-primitive/.github
└── FUNDING.yml
node_modules//simple-get/.github
├── dependabot.yml
└── workflows
    └── ci.yml
node_modules//rechoir/node_modules/resolve/.github
└── FUNDING.yml
node_modules//encoding/node_modules/iconv-lite/.github
└── dependabot.yml
node_modules//jest-resolve/node_modules/resolve/.github
└── FUNDING.yml
node_modules//minimist/.github
└── FUNDING.yml
node_modules//cacache/node_modules/brace-expansion/.github
└── FUNDING.yml
node_modules//unbox-primitive/.github
└── FUNDING.yml
node_modules//supports-preserve-symlinks-flag/.github
└── FUNDING.yml
node_modules//has-symbols/.github
└── FUNDING.yml
node_modules//internal-slot/.github
└── FUNDING.yml
node_modules//rfdc/.github
└── workflows
    └── ci.yml
node_modules//electron-packager/node_modules/resolve/.github
└── FUNDING.yml
node_modules//node-abi/.github
├── CODEOWNERS
└── workflows
    ├── semantic.yml
    └── update-abi.yml
node_modules//sumchecker/.github
├── FUNDING.yml
└── workflows
    └── ci.yml
node_modules//table/node_modules/json-schema-traverse/.github
├── FUNDING.yml
└── workflows
    ├── build.yml
    └── publish.yml
node_modules//undefsafe/.github
└── workflows
    └── release.yml
node_modules//shell-quote/.github
└── FUNDING.yml
node_modules//has-bigints/.github
└── FUNDING.yml
node_modules//side-channel/.github
└── FUNDING.yml
node_modules//node-gyp/gyp/.github
└── workflows
    ├── Python_tests.yml
    ├── node-gyp.yml
    ├── nodejs-windows.yml
    └── release-please.yml
node_modules//node-gyp/.github
├── ISSUE_TEMPLATE.md
├── PULL_REQUEST_TEMPLATE.md
└── workflows
    ├── release-please.yml
    ├── tests.yml
    └── visual-studio.yml
node_modules//is-boolean-object/.github
├── FUNDING.yml
└── workflows
    ├── node-4+.yml
    ├── node-harmony.yml
    ├── node-iojs.yml
    ├── node-pretest.yml
    ├── node-zero.yml
    ├── rebase.yml
    └── require-allow-edits.yml
node_modules//electron-winstaller/.github
└── workflows
    └── semantic.yml
node_modules//is-callable/.github
├── FUNDING.yml
└── main.workflow
node_modules//fastq/.github
└── workflows
    └── ci.yml
node_modules//string.prototype.matchall/.github
└── FUNDING.yml
node_modules//is-string/.github
├── FUNDING.yml
└── workflows
    └── rebase.yml
node_modules//array.prototype.flatmap/.github
└── FUNDING.yml
node_modules//object-inspect/.github
└── workflows
    ├── node-4+.yml
    ├── node-iojs.yml
    ├── node-pretest.yml
    ├── node-zero.yml
    ├── rebase.yml
    └── require-allow-edits.yml
node_modules//object.assign/.github
├── FUNDING.yml
└── workflows
    ├── rebase.yml
    └── require-allow-edits.yml
node_modules//buffer-equal/.github
└── FUNDING.yml

70 directories, 96 files
```

is-negative-zero caught my attention, so I dug a little further.

```
node_modules/is-negative-zero
├── .editorconfig
├── .eslintignore
├── .eslintrc
├── .github
│   └── workflows
│       ├── node-4+.yml
│       ├── node-iojs.yml
│       ├── node-pretest.yml
│       ├── node-zero.yml
│       ├── rebase.yml
│       └── require-allow-edits.yml
├── .nycrc
├── CHANGELOG.md
├── LICENSE
├── README.md
├── index.js
├── package.json
└── test
    └── index.js
```

That's 16 files for what seems like a very simple package. If we look at the contents of the only file in here that is actually useful...

```
───────┬──────────────────────────────────────────────────────────────────────────────────
       │ File: node_modules/is-negative-zero/index.js
───────┼──────────────────────────────────────────────────────────────────────────────────
   1   │ 'use strict';
   2   │
   3   │ module.exports = function isNegativeZero(number) {
   4   │     return number === 0 && (1 / number) === -Infinity;
   5   │ };
   6   │
───────┴──────────────────────────────────────────────────────────────────────────────────
```

That's it. A single meaningful line of code, amongst 500 lines of configuration, documentation, and tests. Better yet, this is just a less useful version of the `Object.is` function, which has been in browsers for a decade, and Node.js since before 1.0! Why has this package _ever_ existed? As far as I can tell, `Object.is` predates is-negative-zero by at least a couple months, but it does seem like they appeared at around the same time. Instead of all this waste, you can just say...

```js
Object.is(x, -0);
```
