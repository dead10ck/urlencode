# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

== [1.0.1] - 2022-05-21
=== Changed

* Minor optimization with encode set selection

== [1.0.0] - 2022-05-21
=== Changed

* Upgrade to Rust edition 2021
* Updated dependencies
  * This included a *breaking change* in `percent-encoding`, which removed the
    canned encode sets. Therefore, the encode sets have been replaced with ones
    defined in the WHATWG URL spec. See https://url.spec.whatwg.org/

== [0.1.2] - 2019-04-25
=== Changed

* Upgrade to Rust edition 2018
* Updated dependencies

== [0.1.1] - 2018-02-12
=== Changed

* Updated dependencies

== [0.1.0] - 2017-08-11

* Initial release
