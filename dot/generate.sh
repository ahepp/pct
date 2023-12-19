#!/bin/bash

cd $(dirname $0)
mkdir -p out
dot -T svg states.dot > out/states.svg
