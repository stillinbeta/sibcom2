[![CircleCI](https://circleci.com/gh/stillinbeta/sibcom2.svg?style=svg)](https://circleci.com/gh/stillinbeta/sibcom2)
<a href='http://www.recurse.com' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a>

# sibcom2

The [second iteration][archive] of [stillinbeta.com](https://stillinbeta.com).

Built in [Rust] using [Rocket].

## Architecture

[`site.yaml`](/site.yaml) is parsed and validated at compile time.
The [`generator`](/generator) crate parses the yaml, and the [`bmon`](/bmon) crate serializes it back to a Rust AST.
The `bmon` crate also has the [handler that turns that AST into HTML or JSON][handler].
The [`server`](/server) crate only serves the handler.

[archive]: https://web.archive.org/web/20181222124211/http://stillinbeta.com/
[rust]: https://www.rust-lang.org/
[rocket]: https://rocket.rs
[handler]: /bmon/src/handler.rs

### Dynamic content

Most of the content is static, parsed directly from `site.yaml`.
The `latest` section of `/hello` is the only dynamic content.
It's pulled from a `redis` instance, one key per service.
The redis instance is populated by the [`updater`](/updater) crate.

## Deployment

All commits are built and validated by [CircleCI][circleci].
Commits to master are then built as [docker images](/Dockerfile) and pushed to [docker hub][hub].

The site is deployed to my digital ocean droplet.
I configure it with ansible.

Push to deploy takes about 10 minutes.

[circleci]: https://circleci.com/gh/stillinbeta/sibcom2
[hub]:https://hub.docker.com/r/stillinbeta/sibcom2
