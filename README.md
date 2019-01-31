# t

This is a simple command-line todo manager.

I loved todo.sh, but after using it for a while, I found that having to manage a bunch of separate plugins to be a bit of a pain.

As a result, I re-wrote the logic I *did* use.  Initially this was as part of a shell script, but then I decided to write it in rust as I've been bitten in the past with differences in installed utilities (e.g. using sed), and whether ripgrep or grep is installed.
