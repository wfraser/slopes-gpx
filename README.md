slopes-gpx
==========

The Slopes ski tracking app on Android doesn't support exporting to GPX for
some reason, though it does on iOS.

Instead, it exports an incompatible `.slopes` file. This program can take that
file and make a regular GPX file out of it.

Usage: `slopes-gpx foobar.slopes > output.gpx`

## TODO
* The file also contains metadata about each lift and run; it would be cool to use that to split
  the output into separate track segments maybe.
