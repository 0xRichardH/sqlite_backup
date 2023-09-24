#### Bug Fixes
- 278851883c9af8965d71099237849bf0db64f019 - **(ci)** fix the workflow name - Richard Hao
- b6ae2f8219731d7cedea54f4559f753fce9a3155 - **(ci)** using body_path to load latest changelog - Richard Hao
- 9e47f02ce4492bb29e1fea69d069b0fd08688c6e - **(ci)** fix GITHUB_OUTPUT - Richard Hao
- 8915f0ba34df02049b449cacb040687b22f8a1aa - add base64 to encode the changelog output - Richard Hao

#### Documentation
- 6619ebabce2cebcebb7f7113b834fb2167a77403 - **(todo)** update todo list - Richard Hao
- 689f0a43596227e59bbf62e5ee72e5bdf958ac06 - move TODO to docs folder - Richard Hao

#### Features
- 83203b13dfc1d1ef07949d61efa9ffed759d2eb5 - **(ci)** update workflow - Richard Hao
- 56947eb46e1631a93c5c0d8e2f53430269932776 - **(ci)** add github action configs to do the release and build - Richard Hao
- 8ebf0a69fabed12fda4dfd61d0c97302f44e8104 - **(ci)** create rust.yml - Richard Hao
- 549b4e0e773174d9376e20c329cc47b5403b89a7 - **(cog)** set cargo version when running cog bump - Richard Hao
- 54e99bf45f7f7622d3ef462a7990057f6785daf7 - **(uploader)** upload object to r2 - Richard Hao
- e9d043dd532f5e9b1c6e39de5a8c05453b3ce87a - create new PR for releasing - Richard Hao

- d49a30df14fcc0355a4839fd597659f7645dba2d - add cog config - Richard Hao

- 1c1498adb5da4fb60dd763a27331b70f83219098 - add dockerignore - Richard Hao

- 35aec520b5f7e96089fd8d4e3e6ad832fb382594 - create dependabot.yml - Richard Hao

- ae227a62d6152f69227cab6ed48ab3045418e2ae - add dockerfile to build docker image - Richard Hao

- 5bd1585634542a05a572808eff838d74acd86b95 - get file info from source file - Richard Hao

- df7876cd6af490bc65a059be6ee91f73f952985c - add .env.example - Richard Hao

- 7cc72b9bb2ad41d20ed7c901ee5d5409c8ab5ce3 - upload file to r2 - Richard Hao

- 994200d7f244787b6299daf4897b32d769f59008 - call R2Uploader form main func - Richard Hao

- 1d517a1fb473ded4886e58d58d096d1b9757ec2e - backup soruce.db (sqlite) to destination db - Richard Hao

- 0dc917542185de7343f3957effebe170aac51cce - add functions to backup sqlite - Richard Hao

#### Miscellaneous Chores
- ef25a7b36696ce8e181434a7dd3d3bcb972bbaac - **(ci)** update releasee ci - Richard Hao
- cf774b359269511ae8c74f8fc56c9a9a90b1611b - **(ci)** install cargo-edit on ci - Richard Hao
- 87e65ee6e1d712e8b89402cba1d6c5190e5b2e85 - **(ci)** use create-pull-request to push commits - Richard Hao
- adfd13f37b72156320e1d751f60f31dd7716cc31 - **(ci)** update base branch for create-pull-request action - Richard Hao
- fffdaac9b71df49e5550c0f93e7366bbcb3c77e1 - **(deps)** bump aws-sdk-s3 from 0.31.1 to 0.31.2 - dependabot[bot]
- 58b58dbe11374d5b89b20c212aa38eefa6980f1a - **(deps)** bump lukemathwalker/cargo-chef in /build - dependabot[bot]
- 5929b8f493cdad6ab471ad53dc234d1d99b96892 - **(version)** v0.1.0 - release-bot
- a2f44b14fca36b70d7d21b9af8ae06a7694f552e - **(version)** v0.1.0 - release-bot
- 914b219fcea132b28c21eefc3313282760a0cff5 - Revert "release(version): Bump to v0.1.0 (#12)" (#13) - Richard Hao

- 38b4204537fd3c0c8bae34100b3d2732eb11e328 - chore update - Richard Hao

- 0d42bff8f529027b140d1e86a9e276e5b7b451df - Revert "release(version): Bump to v0.1.0" - Richard Hao

- 31739b453711bbd387a5db0ea752414f5464ae4a - update - Richard Hao

- 2331a5ff7db9470068a3fe5aefc9613a4b8a3e00 - only allow to trigger the workflow on main - Richard Hao

- 8f78768be10305875ef54460da8003d7e5c62f1e - temporaily allow current to run `cog bump` - Richard Hao

- 8f7cda3fc50c9c9d28cb89d0964a75a67048b8d4 - handle parsing filename errors - Richard Hao

- 228cab57fac49408de72b4f08e0dd9043cd9741e - remove unused file - Richard Hao

- 622e5c6ce1cd0de36f66e236aa98f706d05c81e4 - init rust project - Richard Hao

#### Refactoring
- 792b03ca2991d7c08baf724872b56b9abadcb3df - move out backup logic to backup.rs - Richard Hao

- faaa301e09bb6677da091afd7d96522edd2eb457 - extract args to Argument struct - Richard Hao

#### Releases
- 2bc89997090579a94c0cf091faa124eadda13b25 - **(version)** Bump to v0.1.0 (#12) - github-actions[bot]
- 7055b335361e649a31486eec97bb05bfe4c0953f - **(version)** bump to v0.1.0 - 0xRichardH


