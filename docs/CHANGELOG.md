# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v0.4.1 - 2023-10-10
#### Bug Fixes
- **(ci)** able to trigger workflow within a workflow (#37) - (6365ea8) - Richard Hao
- build docker image on tags created (#34) - (80feb5c) - Richard Hao
#### Miscellaneous Chores
- Revert "release(version): Bump to v0.4.1 (#35)" (#36) - (56d520a) - Richard Hao
#### Releases
- **(version)** Bump to v0.4.1 (#35) - (e069389) - github-actions[bot]

- - -

## v0.4.0 - 2023-10-10
#### Features
- beauty the log (#32) - (2d6cfe0) - Richard Hao
#### Miscellaneous Chores
- **(deps)** bump tokio from 1.32.0 to 1.33.0 (#31) - (9906651) - dependabot[bot]

- - -

## v0.3.0 - 2023-10-09
#### Bug Fixes
- **(ci)** trigger docker image building whenever build/version chagned (#21) - (944e3b5) - Richard Hao
- resolve the cron permission issue - (a08ed04) - Richard Hao
#### Documentation
- **(todo)** update todo list - (2f93e1b) - Richard Hao
- add new todo `keep recent n backups` - (51e7548) - Richard Hao
- update TODO - (10cb48e) - Richard Hao
#### Features
- **(docker)** install cron in the container - (8bd0763) - Richard Hao
- add gpg encryption (#27) - (ac0a909) - Richard Hao
- keep recent n backup records (#26) - (359a111) - Richard Hao
- add nix (#25) - (086dfb6) - Richard Hao
#### Miscellaneous Chores
- **(ci)** update docker.yml - (7d59d01) - Richard Hao
- **(deps)** bump aws-sdk-s3 from 0.32.0 to 0.33.0 (#28) - (c9de330) - dependabot[bot]
- **(deps)** bump lukemathwalker/cargo-chef in /build (#29) - (e66e3fa) - dependabot[bot]
- **(deps)** bump aws-sdk-s3 from 0.31.2 to 0.32.0 (#23) - (0134b11) - dependabot[bot]
- **(deps)** bump clap from 4.4.5 to 4.4.6 (#22) - (372325c) - dependabot[bot]
- release new docker image - (9fed1ec) - Richard Hao
- test if `docker builder` ci works - (f3fa2e9) - Richard Hao

- - -

## v0.2.0 - 2023-09-26
#### Documentation
- update todo - (e38ffe7) - Richard Hao
- update readme - (0b4a100) - Richard Hao
#### Features
- **(enhancement)** Add Project Name and App Env to R2 key (#19) - (33758f5) - Richard Hao
#### Miscellaneous Chores
- **(deps)** bump time from 0.3.28 to 0.3.29 (#18) - (ddd855f) - dependabot[bot]
- Create LICENSE - (a544dce) - Richard Hao

- - -

## v0.1.0 - 2023-09-24
#### Bug Fixes
- **(ci)** fix the workflow name - (2788518) - Richard Hao
- **(ci)** using body_path to load latest changelog - (b6ae2f8) - Richard Hao
- **(ci)** fix GITHUB_OUTPUT - (9e47f02) - Richard Hao
- add base64 to encode the changelog output - (8915f0b) - Richard Hao
#### Documentation
- **(todo)** update todo list - (6619eba) - Richard Hao
- move TODO to docs folder - (689f0a4) - Richard Hao
#### Features
- **(ci)** update workflow - (83203b1) - Richard Hao
- **(ci)** add github action configs to do the release and build - (56947eb) - Richard Hao
- **(ci)** create rust.yml - (8ebf0a6) - Richard Hao
- **(cog)** set cargo version when running cog bump - (549b4e0) - Richard Hao
- **(uploader)** upload object to r2 - (54e99bf) - Richard Hao
- create new PR for releasing - (e9d043d) - Richard Hao
- add cog config - (d49a30d) - Richard Hao
- add dockerignore - (1c1498a) - Richard Hao
- create dependabot.yml - (35aec52) - Richard Hao
- add dockerfile to build docker image - (ae227a6) - Richard Hao
- get file info from source file - (5bd1585) - Richard Hao
- add .env.example - (df7876c) - Richard Hao
- upload file to r2 - (7cc72b9) - Richard Hao
- call R2Uploader form main func - (994200d) - Richard Hao
- backup soruce.db (sqlite) to destination db - (1d517a1) - Richard Hao
- add functions to backup sqlite - (0dc9175) - Richard Hao
#### Miscellaneous Chores
- **(ci)** update releasee ci - (ef25a7b) - Richard Hao
- **(ci)** install cargo-edit on ci - (cf774b3) - Richard Hao
- **(ci)** use create-pull-request to push commits - (87e65ee) - Richard Hao
- **(ci)** update base branch for create-pull-request action - (adfd13f) - Richard Hao
- **(deps)** bump aws-sdk-s3 from 0.31.1 to 0.31.2 - (fffdaac) - dependabot[bot]
- **(deps)** bump lukemathwalker/cargo-chef in /build - (58b58db) - dependabot[bot]
- **(version)** v0.1.0 - (a2f44b1) - release-bot
- update release.yml - (dc9e42e) - Richard Hao
- Revert "release(version): Bump to v0.1.0 (#14)" (#15) - (03e0b2c) - Richard Hao
- Revert "release(version): Bump to v0.1.0 (#12)" (#13) - (914b219) - Richard Hao
- chore update - (38b4204) - Richard Hao
- Revert "release(version): Bump to v0.1.0" - (0d42bff) - Richard Hao
- update - (31739b4) - Richard Hao
- only allow to trigger the workflow on main - (2331a5f) - Richard Hao
- temporaily allow current to run `cog bump` - (8f78768) - Richard Hao
- handle parsing filename errors - (8f7cda3) - Richard Hao
- remove unused file - (228cab5) - Richard Hao
- init rust project - (622e5c6) - Richard Hao
#### Refactoring
- move out backup logic to backup.rs - (792b03c) - Richard Hao
- extract args to Argument struct - (faaa301) - Richard Hao
#### Releases
- **(version)** Bump to v0.1.0 (#14) - (5bdf435) - github-actions[bot]
- **(version)** Bump to v0.1.0 (#12) - (2bc8999) - github-actions[bot]
- **(version)** bump to v0.1.0 - (7055b33) - 0xRichardH

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).