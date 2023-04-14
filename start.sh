#!/bin/bash

pm2 start "./target/release/ddns-updater" --cron-restart="30 * * * *"