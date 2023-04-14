#!/bin/bash

pm2 start "./target/release/ddns-updater" -c "30 * * * *"