# Pomodorust

No one asked for it, no one needs it, it brings no added value, but here it is:

A command line interface [pomodoro](https://en.wikipedia.org/wiki/Pomodoro_Technique) app.
With very little features.

## Usage

Useless by itself, works pretty well with a bit of bash magic
and [notify-send](http://manpages.ubuntu.com/manpages/xenial/man1/notify-send.1.html):

`pomodorust | while read TOMATO; do notify-send "$TOMATO"; done`

or if you like kinda pretending your computer is people,
it also works well with [espeak](http://espeak.sourceforge.net/):

`pomodorust | espeak`

More details on usage available with `pomodorust -h`.

## Motivation

I wanted to write something quick using Rust.

## Existing (and better) work

[pomo](https://github.com/kevinschoon/pomo)

## Upcoming features

Customisable messages.
More documentation.
