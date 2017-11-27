Chess host fingerprint
======================

Using chess games instead of random numbers or abstract ascii art for humans to perceive MiM attacks easier.

based on the original abstract art idea on [Announce: OpenSSH 5.1 released](http://lists.mindrot.org/pipermail/openssh-unix-dev/2008-July/026693.html)

```
from this:              to this:            or this (no color out of terminal):
+---[RSA 2048]----+    +---[RSA 2048]----+   Richard
| =+o...+=o..     |    |8                | : | : | : | : |
|o++... *o .      |    |7                | | : | : | : | :
|*.o.  *o.        |    |6  k p           | : |♚:♟| : | : |
|oo.  ..o.= .     |    |5      R         | | : | :♖| : | :
|.+o. .. S =      |    |4                | : | : | : | : |
|*=+ .  o = .     |    |3                | | : | : | : | :
|OE .  . o        |    |2                | :♔| : | : | : |
| o     .         |    |1K               |
|                 |    | a b c d e f g h |  William
+----[SHA256]-----+    +----[SHA256]-----+
```

Entropy
=======

State space of the input is 128 bits. e.g. 43:51:43:a1:b5:fc:8b:b7:0a:3a:a9:b1:0f:66:73:a8

State space of playable chess games is said to be power(10 43).

Biggest challenge is to add some sort of belivability into the position without making colision too easy.

player names
------------

first and last bits of the key will generate names for the "players", to make colision harder, in case the code does not generate too unique output.

Notation
========

This probably only matters for code contributors.

will be using https://chessprogramming.wikispaces.com/Forsyth-Edwards+Notation


Transient state
---------------

To increase our space, we can signal things like last move, and castling spent or not, etc. Last move can be shown with an arrow from the departing square (if we can point to all needed angles. and maybe use a different thing for the knight?. maybe also show the last moved piece in a different color and only mark the originating square, so that a enpassant option or not can be distingued)

Drawing
=======

Still not caring about terminal support and compatibility.

pieces
------

UTF8 glyphs. Alternatively, we can show one ascii letter for each piece.

```
♜♞♝♛♚♝♞♜
♟♟♟♟♟♟♟♟
♙♙♙♙♙♙♙♙
♖♘♗♕♔♗♘♖
```

Board
-----

The background (light/dark squares) will have the be drawn using terminal (color) modes.



Fairy chess
===========

No.

reason is very few people know any of it, let alone all of it. So it cannot be usefull in remebering a chess board.

License
=======

GNU GPLv3
