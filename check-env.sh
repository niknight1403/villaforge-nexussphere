#!/bin/sh

# Check if Rust is installed
if ! command -v rustc &> /dev/null
then
    echo "Rust could not be found, please install it first."
    exit 1
fi

# Check if Cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo could not be found, please install it first."
    exit 1
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null
then
    echo "Node.js could not be found, please install it first."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null
then
    echo "npm could not be found, please install it first."
    exit 1
fi

# Check if Python is installed
if ! command -v python3 &> /dev/null
then
    echo "Python could not be found, please install it first."
    exit 1
fi

# Check if Docker is installed
if ! command -v docker &> /dev/null
then
    echo "Docker could not be found, please install it first."
    exit 1
fi

# Check if Kubernetes is installed
if ! command -v kubectl &> /dev/null
then
    echo "Kubernetes could not be found, please install it first."
    exit 1
fi

# If all checks pass, exit with success
exit 0