#!/usr/bin/env sh

convert -delay $2 out/* -delay 150 out/out$1.jpg out/out000.jpg out/out$1.jpg $3.gif && firefox $3.gif
