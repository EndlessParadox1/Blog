#!/bin/bash
nohup cargo run --relase > /root/out.log 2 > &1 &
