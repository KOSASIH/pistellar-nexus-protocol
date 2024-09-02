#!/bin/bash

# Function to get Stellar asset information
get_stellar_asset() {
  local asset_code=$1
  local stellar_data=$(jq -r ".stellar_assets[] | select(.code == \"$asset_code\")" data/stellar_data.json)
  echo "$stellar_data"
}

# Function to get Pi asset information
get_pi_asset() {
  local asset_symbol=$1
  local pi_data=$(jq -r ".pi_assets[] | select(.symbol == \"$asset_symbol\")" data/pi_data.json)
  echo "$pi_data"
}

# Function to get Stellar network information
get_stellar_network() {
  local network=$1
  local stellar_data=$(jq -r ".stellar_network.$network" data/stellar_data.json)
  echo "$stellar_data"
}

# Function to get Pi network information
get_pi_network() {
  local network=$1
  local pi_data=$(jq -r ".pi_network.$network" data/pi_data.json)
  echo "$pi_data"
}
