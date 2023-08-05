## unsharex

### Status

Currently this is identical to the upstream unshare command. It does not implement additional functionality yet.

The code's being refactored first.

### Why?

unsharex (unshare eXtended) is a modified version of the util-linux "unshare" utility.

This version of unshare exists for two reasons:

1. The rewrite-it-in-rust brigade knows no bounds
2. More importantly, a multi-step unshare command has valid usecases, and this implements that. See [this upstream issue](https://github.com/util-linux/util-linux/issues/885).

### License/Copyright

The original code for this repository was generated from [util-linux](https://git.kernel.org/pub/scm/utils/util-linux/util-linux.git) at commit [9f592cc21](https://git.kernel.org/pub/scm/utils/util-linux/util-linux.git/commit/?id=9f592cc2164c3db3c76a2898975ceb58d09f0cbb)

The original AUTHORS and COPYING file are preserved.

As such, this is only available under the GPLv2 license.


### Differences

Known differnces:

* More integ tests than upstream (upstream does not have integ tests for unshare it appears)
* Command name differs (unsharex vs unshare)
