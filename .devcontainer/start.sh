#!/bin/sh
nohup sh -c 'cargo run &'
cd frontend
npm install
nohup sh -c 'npm run serve &'