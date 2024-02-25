# Developer's Guide

When developing custom styles for the Blog Builder, developers are strongly encouraged to heed the following guidelines.  Doing so will maximize functionality of the Blog Builder's functionalities across styles.

## Implementation

### Required Implementations

CSS style implementations must implement the following styles. (`css.style => \ctrl-sequence`)

```
h1 => \header

h2 => #

h3 => ##

h4 => ###

h5 => \subtitle

h6 => \subsubtitle

p => ~

a => \href

div.tiles => \tiles

div.menu => \menu

div.tile => \tile

h6.footer => \footer
```

### Recommended Implementations

```
body

p a

p.block, p.citation => \block

div.topblock => \topblock
```

## Beginning an Implementation

When starting an implementation, begin using the `template.css` style template.