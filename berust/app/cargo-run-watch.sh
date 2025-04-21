#!/bin/bash

cargo watch -c -w src -w migration -w entity -w ../../communication -x 'run'
