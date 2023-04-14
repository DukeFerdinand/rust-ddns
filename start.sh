#!/bin/bash

pm2 start "./target/release/ddns-updater" --name rust-ddns --no-autorestart --cron-restart="0,30 * * * *"