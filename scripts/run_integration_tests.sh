#!/usr/bin/env bash

function get_frequency_pid () {
    lsof -i tcp:9933 | grep frequency | xargs | awk '{print $2}'
}

function cleanup () {
    local signal="$1"

    case "$signal" in
        TERM|INT) 
            # Catch TERM and INT signals and exit gracefully
            exit
            ;;
        EXIT)
            if [ "${SHOULD_KILL}" = true ]
            then
                # kill_freq.sh is not used here because we do not know what directory
                # the script is in when a signal is received. Therefore, we do not
                # know how to navigate to the kill_freq.sh script with relative paths.
                PID=$( get_frequency_pid )
                if [ -n "${PID}" ]
                then
                    echo "Killing Frequency... PID: ${PID}"
                    kill -9 ${PID}
                    echo "Frequency has been killed. 💀"
                else
                    echo "Frequency is already gone. 💀"
                fi 
            fi
            ;;
    esac
}

RUNDIR=$(dirname ${0})
SKIP_JS_BUILD=
trap 'cleanup EXIT' EXIT
trap 'cleanup TERM' TERM
trap 'cleanup INT' INT

while getopts "s:" OPTNAME
do
    case "${OPTNAME}" in
        "s") SKIP_JS_BUILD=$OPTARG
        ;;
    esac
done
shift $((OPTIND-1))

TEST="test"
START="start"

if [[ "$1" == "load" ]]; then
    TEST="test:load"
    START="start-manual"
fi

echo "The integration test output will be logged on this console"
echo "and the Frequency node output will be logged to the file frequency.log."
echo "You can 'tail -f frequency.log' in another terminal to see both side-by-side."
echo ""
echo -e "Checking to see if Frequency is running..."

PID=$( get_frequency_pid )

SHOULD_KILL=false

if [ -z "${PID}" ]
then
    echo "Building local Frequency executable..."
    if ! make build-local
    then
        echo "Error building Frequency executable; aborting."
        exit 1
    fi

    echo "Starting a Frequency Node with ${START}..."
    case ${START} in
        "start") ${RUNDIR}/init.sh start-frequency-instant >& frequency.log &
        ;;
        "start-manual") ${RUNDIR}/init.sh start-frequency-manual >& frequency.log &
        ;;
    esac
    SHOULD_KILL=true
fi

declare -i timeout_secs=30
declare -i i=0
while [[ -z "${PID}" && i -lt timeout_secs ]] 
do
   PID=$( get_frequency_pid )
   sleep 1
   (( i += 1 ))
done

if [ -z "${PID}" ]
then
    echo "Unable to find or start a Frequency node; aborting."
    exit 1
fi
echo "---------------------------------------------"
echo "Frequency running here:"
echo "PID: ${PID}"
echo "---------------------------------------------"

if [ "${SKIP_JS_BUILD}" = "1" ]
then
    echo "Skipping js/api-augment build"
else
    echo "Building js/api-augment..."
    cd js/api-augment
    npm i
    npm run fetch:local
    npm run --silent build
    cd dist
    echo "Packaging up into js/api-augment/dist/frequency-chain-api-augment-0.0.0.tgz"
    npm pack --silent
    cd ../../..
fi


cd integration-tests
echo "Installing js/api-augment/dist/frequency-chain-api-augment-0.0.0.tgz"
npm i ../js/api-augment/dist/frequency-chain-api-augment-0.0.0.tgz
npm install
echo "---------------------------------------------"
echo "Starting Tests..."
echo "---------------------------------------------"
WS_PROVIDER_URL="ws://127.0.0.1:9944" npm run $TEST
